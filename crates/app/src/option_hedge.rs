// use dioxus::prelude::*;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::f64::consts::PI;
// use ant_model::instruments::{OptionContract, FuturesContract};
// use ant_model::data::{quote};
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct ScenarioResults {
//     delta: f64,
//     gamma: f64,
//     theta: f64,
//     vega: f64,
//     rho: f64,
//     net_delta: f64,
//     hedge_ratio: f64,
//     max_loss: f64,
//     hedge_efficiency: f64,
//     risk_level: String,
//     total_cost: f64,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Scenario {
//     id: u64,
//     name: String,
//     description: Option<String>,
//     futures_price: f64,
//     futures_quantity: f64,
//     futures_direction: String,
//     volatility: f64,
//     time_to_expiry: f64, // 年为单位
//     risk_free_rate: f64,
//     options: Vec<OptionContract>,
//     selected: bool,
//     results: ScenarioResults,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct AppState {
//     futures_contract: String,
//     futures_direction: String,
//     futures_price: f64,
//     futures_quantity: f64,
//     scenario_name: String,
//     volatility: f64,
//     time_to_expiry: u64, // 天为单位
//     risk_free_rate: f64,
//     selected_options: Vec<OptionContract>,
//     scenarios: Vec<Scenario>,
//     current_option_direction: String,
//     t_quote_data: Option<quote>,
//     active_tab: String,
//     is_wireframe: bool,
//     is_animating: bool,
// }
//
//
// // 初始化应用状态
// fn init_app_state() -> AppState {
//     AppState {
//         futures_contract: String::new(),
//         futures_direction: "long".to_string(),
//         futures_price: 3000.0,
//         futures_quantity: 10.0,
//         scenario_name: String::new(),
//         volatility: 20.0,
//         time_to_expiry: 30,
//         risk_free_rate: 3.0,
//         selected_options: Vec::new(),
//         scenarios: Vec::new(),
//         current_option_direction: "long".to_string(),
//         t_quote_data: None,
//         active_tab: "greeks".to_string(),
//         is_wireframe: false,
//         is_animating: false,
//     }
// }
//
// //
// // // 生成T型报价数据
// // fn generate_t_quote_data(futures_price: f64) -> TQuoteData {
// //     let mut strikes = Vec::new();
// //     let mut calls = Vec::new();
// //     let mut puts = Vec::new();
// //
// //     // 生成执行价格（期货价格±10%）
// //     for i in -5..=5 {
// //         let strike = futures_price * (1.0 + (i as f64) * 0.02);
// //         strikes.push(strike);
// //
// //         // 模拟期权价格（基于Black-Scholes）
// //         let call_price = black_scholes(futures_price, strike, 30.0/365.0, 0.03, 0.2, "call");
// //         let put_price = black_scholes(futures_price, strike, 30.0/365.0, 0.03, 0.2, "put");
// //
// //         calls.push(QuotePrice {
// //             bid: call_price * 0.95,
// //             ask: call_price * 1.05,
// //             volume: (rand::random::<f64>() * 1000.0 + 100.0) as u64,
// //         });
// //
// //         puts.push(QuotePrice {
// //             bid: put_price * 0.95,
// //             ask: put_price * 1.05,
// //             volume: (rand::random::<f64>() * 1000.0 + 100.0) as u64,
// //         });
// //     }
// //
// //     TQuoteData { strikes, calls, puts }
// // }
//
// pub fn option_hedge_app() -> Element {
//     // 状态管理
//     let state = use_signal(init_app_state);
//
//     // 加载T型报价
//     let load_t_quote = move |_| {
//         if !state().futures_contract.is_empty() && state().futures_price > 0.0 {
//             let data = generate_t_quote_data(state().futures_price);
//             let mut new_state = state();
//             new_state.t_quote_data = Some(data);
//             state.set(new_state);
//         }
//     };
//
//     // 设置期权方向
//     let set_option_direction = move |direction: &str| {
//         let mut new_state = state();
//         new_state.current_option_direction = direction.to_string();
//         state.set(new_state);
//     };
//
//     // 选择期权
//     let select_option = move |type_: &str, strike: f64, price: f64| {
//         let option_id = format!("{}_{}_{}", type_, strike, state().current_option_direction);
//
//         let mut new_state = state();
//         // 检查是否已经选择该期权
//         if let Some(index) = new_state.selected_options.iter().position(|opt| opt.id == option_id) {
//             // 取消选择
//             new_state.selected_options.remove(index);
//         } else {
//             // 选择期权
//             new_state.selected_options.push(OptionContract {
//                 id: option_id,
//                 type_: type_.to_string(),
//                 strike,
//                 price,
//                 quantity: 1.0,
//                 direction: new_state.current_option_direction.clone(),
//                 contract: new_state.futures_contract.clone(),
//             });
//         }
//         state.set(new_state);
//     };
//
//     // 更新期权数量
//     let update_option_quantity = move |option_id: &str, quantity: f64| {
//         let mut new_state = state();
//         if let Some(option) = new_state.selected_options.iter_mut().find(|opt| opt.id == option_id) {
//             option.quantity = quantity.max(0.1).min(100.0);
//         }
//         state.set(new_state);
//     };
//
//     // 移除期权
//     let remove_option = move |option_id: &str| {
//         let mut new_state = state();
//         new_state.selected_options.retain(|opt| opt.id != option_id);
//         state.set(new_state);
//     };
//
//     // 清除选择
//     let clear_selection = move |_| {
//         let mut new_state = state();
//         new_state.selected_options.clear();
//         state.set(new_state);
//     };
//
//     // 创建对冲方案
//     let create_scenario = move |_| {
//         if state().selected_options.is_empty() {
//             return;
//         }
//
//         let scenario_name = if state().scenario_name.is_empty() {
//             format!("方案{}", state().scenarios.len() + 1)
//         } else {
//             state().scenario_name.clone()
//         };
//
//         let volatility = state().volatility / 100.0;
//         let time_to_expiry = state().time_to_expiry as f64 / 365.0;
//         let risk_free_rate = state().risk_free_rate / 100.0;
//
//         let results = calculate_scenario_results(
//             state().futures_price,
//             state().futures_quantity,
//             &state().futures_direction,
//             &state().selected_options,
//             volatility,
//             time_to_expiry,
//             risk_free_rate,
//         );
//
//         let new_scenario = Scenario {
//             id: std::time::SystemTime::now()
//                 .duration_since(std::time::UNIX_EPOCH)
//                 .unwrap()
//                 .as_millis() as u64,
//             name: scenario_name,
//             description: None,
//             futures_price: state().futures_price,
//             futures_quantity: state().futures_quantity,
//             futures_direction: state().futures_direction.clone(),
//             volatility,
//             time_to_expiry,
//             risk_free_rate,
//             options: state().selected_options.clone(),
//             selected: false,
//             results,
//         };
//
//         let mut new_state = state();
//         new_state.scenarios.push(new_scenario);
//         new_state.scenario_name.clear();
//         new_state.selected_options.clear();
//         state.set(new_state);
//     };
//
//     // 切换方案选择状态
//     let toggle_scenario_selection = move |scenario_id: u64| {
//         let mut new_state = state();
//         if let Some(scenario) = new_state.scenarios.iter_mut().find(|sc| sc.id == scenario_id) {
//             scenario.selected = !scenario.selected;
//         }
//         state.set(new_state);
//     };
//
//     // 删除方案
//     let delete_scenario = move |scenario_id: u64| {
//         let mut new_state = state();
//         new_state.scenarios.retain(|sc| sc.id != scenario_id);
//         state.set(new_state);
//     };
//
//     // 编辑方案
//     let edit_scenario = move |scenario_id: u64| {
//         if let Some(scenario) = state().scenarios.iter().find(|sc| sc.id == scenario_id) {
//             let mut new_state = state();
//             new_state.scenario_name = scenario.name.clone();
//             new_state.volatility = scenario.volatility * 100.0;
//             new_state.time_to_expiry = (scenario.time_to_expiry * 365.0).round() as u64;
//             new_state.risk_free_rate = scenario.risk_free_rate * 100.0;
//             new_state.selected_options = scenario.options.clone();
//             state.set(new_state);
//         }
//     };
//
//     // 切换标签
//     let switch_tab = move |tab_name: &str| {
//         let mut new_state = state();
//         new_state.active_tab = tab_name.to_string();
//         state.set(new_state);
//     };
//
//     // 生成模拟数据
//     let generate_simulation_data = move |_| {
//         let futures_price = state().futures_price;
//         let futures_quantity = state().futures_quantity;
//         let futures_direction = state().futures_direction.clone();
//         let volatility = state().volatility / 100.0;
//         let time_to_expiry = state().time_to_expiry as f64 / 365.0;
//         let risk_free_rate = state().risk_free_rate / 100.0;
//
//         // 生成模拟方案
//         let simulation_scenarios = [
//             ("保守型对冲", "低风险，有限收益", vec![
//                 OptionContract {
//                     id: "put_0.95_long".to_string(),
//                     type_: "put".to_string(),
//                     strike: futures_price * 0.95,
//                     price: 25.0,
//                     quantity: 1.2,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.05_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.05,
//                     price: 20.0,
//                     quantity: 0.8,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//             ]),
//             ("激进型对冲", "高风险，高收益潜力", vec![
//                 OptionContract {
//                     id: "put_0.98_long".to_string(),
//                     type_: "put".to_string(),
//                     strike: futures_price * 0.98,
//                     price: 35.0,
//                     quantity: 2.0,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.02_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.02,
//                     price: 30.0,
//                     quantity: 1.5,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "put_0.92_long".to_string(),
//                     type_: "put".to_string(),
//                     strike: futures_price * 0.92,
//                     price: 15.0,
//                     quantity: 0.5,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//             ]),
//             ("平衡型对冲", "风险收益平衡", vec![
//                 OptionContract {
//                     id: "put_0.97_long".to_string(),
//                     type_: "put".to_string(),
//                     strike: futures_price * 0.97,
//                     price: 28.0,
//                     quantity: 1.5,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.03_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.03,
//                     price: 25.0,
//                     quantity: 1.0,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "put_0.90_long".to_string(),
//                     type_: "put".to_string(),
//                     strike: futures_price * 0.90,
//                     price: 12.0,
//                     quantity: 0.3,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.08_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.08,
//                     price: 18.0,
//                     quantity: 0.7,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//             ]),
//             ("蝶式策略", "中性市场策略", vec![
//                 OptionContract {
//                     id: "call_0.95_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 0.95,
//                     price: 45.0,
//                     quantity: 1.0,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.00_short".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.00,
//                     price: 25.0,
//                     quantity: 2.0,
//                     direction: "short".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.05_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.05,
//                     price: 10.0,
//                     quantity: 1.0,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//             ]),
//             ("宽跨式策略", "高波动率策略", vec![
//                 OptionContract {
//                     id: "put_0.90_long".to_string(),
//                     type_: "put".to_string(),
//                     strike: futures_price * 0.90,
//                     price: 15.0,
//                     quantity: 1.5,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//                 OptionContract {
//                     id: "call_1.10_long".to_string(),
//                     type_: "call".to_string(),
//                     strike: futures_price * 1.10,
//                     price: 12.0,
//                     quantity: 1.5,
//                     direction: "long".to_string(),
//                     contract: state().futures_contract.clone(),
//                 },
//             ]),
//         ];
//
//         let mut new_scenarios = Vec::new();
//         let now = std::time::SystemTime::now()
//             .duration_since(std::time::UNIX_EPOCH)
//             .unwrap()
//             .as_millis() as u64;
//
//         for (idx, (name, desc, options)) in simulation_scenarios.iter().enumerate() {
//             let results = calculate_scenario_results(
//                 futures_price,
//                 futures_quantity,
//                 &futures_direction,
//                 options,
//                 volatility,
//                 time_to_expiry,
//                 risk_free_rate,
//             );
//
//             new_scenarios.push(Scenario {
//                 id: now + idx as u64,
//                 name: name.to_string(),
//                 description: Some(desc.to_string()),
//                 futures_price,
//                 futures_quantity,
//                 futures_direction: futures_direction.clone(),
//                 volatility,
//                 time_to_expiry,
//                 risk_free_rate,
//                 options: options.clone(),
//                 selected: true,
//                 results,
//             });
//         }
//
//         let mut new_state = state();
//         new_state.scenarios = new_scenarios;
//         state.set(new_state);
//     };
//
//     // 清除模拟数据
//     let clear_simulation_data = move |_| {
//         let mut new_state = state();
//         new_state.scenarios.clear();
//         state.set(new_state);
//     };
//
//     // 切换线框模式
//     let toggle_wireframe = move |_| {
//         let mut new_state = state();
//         new_state.is_wireframe = !new_state.is_wireframe;
//         state.set(new_state);
//     };
//
//     // 切换动画
//     let toggle_animation = move |_| {
//         let mut new_state = state();
//         new_state.is_animating = !new_state.is_animating;
//         state.set(new_state);
//     };
//
//     // 重置3D视图
//     let reset_3d_view = move |_| {
//         // 此处仅设置标志，实际重置将在JavaScript中处理
//         // 在实际实现中，需要使用Dioxus的web特性与JavaScript交互
//     };
//
//     // 渲染UI
//     rsx! {
//         div { class: "container",
//             div { class: "header",
//                 h1 { "期货期权对冲策略分析平台" }
//                 p { "专业的期货期权对冲工具，支持T型报价、多方案对比、3D可视化分析" }
//             }
//
//             // 期货合约选择
//             div { class: "card",
//                 h2 { "📈 期货合约选择" }
//                 div { class: "form-section",
//                     div { class: "form-group",
//                         label { r#for: "futuresContract", "期货合约" }
//                         select {
//                             id: "futuresContract",
//                             onchange: move |_| load_t_quote(()),
//                             value: "{state().futures_contract}",
//                             oninput: move |e| {
//                                 let mut new_state = state();
//                                 new_state.futures_contract = e.value().to_string();
//                                 state.set(new_state);
//                             },
//                             option { value: "", "请选择期货合约" }
//                             option { value: "IF2312", "沪深300股指期货 IF2312" }
//                             option { value: "IC2312", "中证500股指期货 IC2312" }
//                             option { value: "IH2312", "上证50股指期货 IH2312" }
//                             option { value: "CU2312", "沪铜期货 CU2312" }
//                             option { value: "AL2312", "沪铝期货 AL2312" }
//                             option { value: "ZN2312", "沪锌期货 ZN2312" }
//                         }
//                     }
//
//                     div { class: "form-group",
//                         label { r#for: "futuresDirection", "期货方向" }
//                         select {
//                             id: "futuresDirection",
//                             value: "{state().futures_direction}",
//                             oninput: move |e| {
//                                 let mut new_state = state();
//                                 new_state.futures_direction = e.value().to_string();
//                                 state.set(new_state);
//                             },
//                             option { value: "long", "多头 (Long)" }
//                             option { value: "short", "空头 (Short)" }
//                         }
//                     }
//
//                     div { class: "form-group",
//                         label { r#for: "futuresPrice", "期货价格 (元)" }
//                         input {
//                             id: "futuresPrice",
//                             r#type: "number",
//                             value: "{state().futures_price}",
//                             step: "0.01",
//                             placeholder: "输入期货价格",
//                             oninput: move |e| {
//                                 if let Ok(val) = e.value().parse::<f64>() {
//                                     let mut new_state = state();
//                                     new_state.futures_price = val;
//                                     state.set(new_state);
//                                 }
//                             },
//                         }
//                     }
//
//                     div { class: "form-group",
//                         label { r#for: "futuresQuantity", "期货数量 (手)" }
//                         input {
//                             id: "futuresQuantity",
//                             r#type: "number",
//                             value: "{state().futures_quantity}",
//                             step: "0.1",
//                             min: "0.1",
//                             placeholder: "输入期货数量",
//                             oninput: move |e| {
//                                 if let Ok(val) = e.value().parse::<f64>() {
//                                     let mut new_state = state();
//                                     new_state.futures_quantity = val;
//                                     state.set(new_state);
//                                 }
//                             },
//                         }
//                     }
//                 }
//             }
//
//             // T型期权报价
//             if let Some(t_quote) = &state().t_quote_data {
//                 div { class: "t-quote-container",
//                     h2 { "📊 T型期权报价" }
//                     div { class: "direction-selector",
//                         div {
//                             class: "direction-btn long",
//                             onclick: move |_| set_option_direction("long"),
//                             "📈 买入期权"
//                         }
//                         div {
//                             class: "direction-btn short",
//                             onclick: move |_| set_option_direction("short"),
//                             "📉 卖出期权"
//                         }
//                     }
//
//                     div { id: "tQuoteTable",
//                         table { class: "t-quote-table",
//                             thead {
//                                 tr {
//                                     th { "看涨期权" }
//                                     th { "执行价" }
//                                     th { "看跌期权" }
//                                     th { "成交量" }
//                                 }
//                             }
//                             tbody {
//                                 for (i, strike) in t_quote.strikes.iter().enumerate() {
//                                     let strike_clone = *strike;
//                                     let call_bid = t_quote.calls[i].bid;
//                                     let put_bid = t_quote.puts[i].bid;
//                                     let call_volume = t_quote.calls[i].volume;
//                                     let put_volume = t_quote.puts[i].volume;
//
//                                     tr {
//                                         class: "call-row",
//                                         td {
//                                             class: "option-cell",
//                                             onclick: move |_| select_option("call", strike_clone, call_bid),
//                                             "{:.2}/{:.2}", call_bid, t_quote.calls[i].ask
//                                         }
//                                         td {
//                                             class: "strike-cell",
//                                             "{:.0}", strike_clone
//                                         }
//                                         td {
//                                             class: "option-cell",
//                                             onclick: move |_| select_option("put", strike_clone, put_bid),
//                                             "{:.2}/{:.2}", put_bid, t_quote.puts[i].ask
//                                         }
//                                         td {
//                                             "{}/{}", call_volume, put_volume
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//
//                     // 已选择的期权
//                     if !state().selected_options.is_empty() {
//                         div { class: "selected-options-container",
//                             div { class: "selected-options-title", "已选择的期权合约" }
//                             div { id: "selectedOptionsList",
//                                 for option in state().selected_options.iter() {
//                                     let id_clone = option.id.clone();
//                                     let option_type_display = if option.type_ == "call" { "看涨" } else { "看跌" };
//                                     let direction_display = if option.direction == "long" { "买入" } else { "卖出" };
//                                     let direction_color = if option.direction == "long" { "#27ae60" } else { "#e17055" };
//
//                                     div { class: "option-row",
//                                         div { class: "option-info",
//                                             div { class: "option-details",
//                                                 div {
//                                                     strong { "{option_type_display}期权" }
//                                                     " - 执行价: {:.0}", option.strike
//                                                 }
//                                                 div { class: "option-price",
//                                                     "价格: {:.1}元 | ", option.price
//                                                     span {
//                                                         style: "color: {direction_color}; font-weight: bold;",
//                                                         "{direction_display}"
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         div { style: "display: flex; align-items: center; gap: 10px;",
//                                             label { "数量:" }
//                                             input {
//                                                 r#type: "number",
//                                                 class: "option-quantity",
//                                                 value: "{option.quantity}",
//                                                 step: "0.1",
//                                                 min: "0.1",
//                                                 max: "100",
//                                                 oninput: move |e| {
//                                                     if let Ok(val) = e.value().parse::<f64>() {
//                                                         update_option_quantity(&id_clone, val);
//                                                     }
//                                                 },
//                                             }
//                                             button {
//                                                 class: "btn-small btn-delete",
//                                                 onclick: move |_| remove_option(&id_clone),
//                                                 "删除"
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//
//                     div { style: "margin-top: 15px;",
//                         button {
//                             r#type: "button",
//                             class: "btn",
//                             onclick: create_scenario,
//                             "➕ 创建对冲方案"
//                         }
//                         button {
//                             r#type: "button",
//                             class: "btn",
//                             onclick: clear_selection,
//                             "🗑️ 清除选择"
//                         }
//                     }
//                 }
//             }
//
//             // 方案管理
//             div { class: "scenario-manager",
//                 h2 { "📋 对冲方案管理" }
//                 div { class: "form-section",
//                     div { class: "form-group",
//                         label { for: "scenarioName", "方案名称" }
//                         input {
//                             id: "scenarioName",
//                             r#type: "text",
//                             value: "{state().scenario_name}",
//                             placeholder: "输入方案名称",
//                             oninput: move |e| {
//                                 let mut new_state = state();
//                                 new_state.scenario_name = e.value().to_string();
//                                 state.set(new_state);
//                             },
//                         }
//                     }
//                     div { class: "form-group",
//                         label { for: "volatility", "波动率 (%)" }
//                         input {
//                             id: "volatility",
//                             r#type: "number",
//                             value: "{state().volatility}",
//                             step: "0.1",
//                             placeholder: "输入波动率",
//                             oninput: move |e| {
//                                 if let Ok(val) = e.value().parse::<f64>() {
//                                     let mut new_state = state();
//                                     new_state.volatility = val;
//                                     state.set(new_state);
//                                 }
//                             },
//                         }
//                     }
//                     div { class: "form-group",
//                         label { for: "timeToExpiry", "到期时间 (天)" }
//                         input {
//                             id: "timeToExpiry",
//                             r#type: "number",
//                             value: "{state().time_to_expiry}",
//                             step: "1",
//                             placeholder: "输入到期天数",
//                             oninput: move |e| {
//                                 if let Ok(val) = e.value().parse::<u64>() {
//                                     let mut new_state = state();
//                                     new_state.time_to_expiry = val;
//                                     state.set(new_state);
//                                 }
//                             },
//                         }
//                     }
//                     div { class: "form-group",
//                         label { for: "riskFreeRate", "无风险利率 (%)" }
//                         input {
//                             id: "riskFreeRate",
//                             r#type: "number",
//                             value: "{state().risk_free_rate}",
//                             step: "0.1",
//                             placeholder: "输入无风险利率",
//                             oninput: move |e| {
//                                 if let Ok(val) = e.value().parse::<f64>() {
//                                     let mut new_state = state();
//                                     new_state.risk_free_rate = val;
//                                     state.set(new_state);
//                                 }
//                             },
//                         }
//                     }
//                 }
//                 div { class: "form-group",
//                     button {
//                         r#type: "button",
//                         class: "btn",
//                         onclick: create_scenario,
//                         "➕ 创建对冲方案"
//                     }
//                     button {
//                         r#type: "button",
//                         class: "btn",
//                         onclick: clear_selection,
//                         "🗑️ 清除选择"
//                     }
//                 }
//             }
//         }
//     }
// }