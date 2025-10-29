use clap::Parser;
use log::LevelFilter;
use ant_cli::opt::AntCli;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();  // 加载环境变量 .env

    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("sqlx", LevelFilter::Off)
        .init()
        .unwrap();

    let binance_api_key = std::env::var("BINANCE_API_KEY")
    .expect("BINANCE_API_KEY not found in .env file");
    log::info!("binance_api_key:{}", binance_api_key);

    if let Err(e) = ant_cli::run(AntCli::parse()).await{
        log::error!("Error executing ant cli{e}");   
    }
    println!("Hello, world Cmd cli!");
}
