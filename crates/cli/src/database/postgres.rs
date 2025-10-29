use anyhow::Ok;
use tokio::time::{ Duration};

use crate::opt::{DatabaseCommand, DatabaseOpt};

pub async fn run_database_command(opt: DatabaseOpt) -> anyhow::Result<()>{
    match opt.command {
        DatabaseCommand::Init(_config) => {
            tokio::time::sleep(Duration::from_millis(1)).await;
            log::info!("init database");
        },
        DatabaseCommand::Drop(_config) => {
            tokio::time::sleep(Duration::from_millis(1)).await;
            log::info!("drop database");
        }
    }
    Ok(())
}