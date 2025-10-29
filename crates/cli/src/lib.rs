#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod opt;
mod database;
use crate::{opt::{AntCli, Commands}};
use crate::database::postgres::run_database_command;

pub async fn run(opt: AntCli) -> anyhow::Result<()>{
    match opt.command{
        Commands::Database(database_opt) => run_database_command(database_opt).await?,
    }
    Ok(())
}