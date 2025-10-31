use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f64::consts::PI;
use ant_model::instruments::{OptionContract, FuturesContract};
use ant_model::data::{quote};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScenarioResults {
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    net_delta: f64,
    hedge_ratio: f64,
    max_loss: f64,
    hedge_efficiency: f64,
    risk_level: String,
    total_cost: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scenario {
    id: u64,
    name: String,
    description: Option<String>,
    futures_price: f64,
    futures_quantity: f64,
    futures_direction: String,
    volatility: f64,
    time_to_expiry: f64, // 年为单位
    risk_free_rate: f64,
    options: Vec<OptionContract>,
    selected: bool,
    results: ScenarioResults,
}
