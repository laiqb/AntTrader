mod option_hedge;

use dioxus::prelude::*;

fn main() {
    // 启动桌面应用
    launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    let msg: String = "hello".to_string();
    println!("{}", msg);

    rsx! {
        div {
            style: "text-align: center; font-family: Arial, sans-serif; padding: 20px;",

            h1 { "AntTrader 桌面应用" }
            p { "这是一个 Dioxus 桌面应用" }

            p { "计数器: {count}" }

            button {
                onclick: move |_| count += 1,
                style: "padding: 10px 20px; font-size: 16px; margin: 5px;",
                "点击我!"
            }

            button {
                onclick: move |_| count.set(0),
                style: "padding: 10px 20px; font-size: 16px; margin: 5px;",
                "重置"
            }
        }
    }
}