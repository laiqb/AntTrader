use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about, author)]
pub struct AntCli{
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands{
    Database(DatabaseOpt),
    // Conf(ConfOpt),
    // Backtest(BacktestOpt),
    // Live(LiveOpt)
}


#[derive(Parser, Debug)]
#[command(about = "Postgres database operations", long_about = None)]
pub struct DatabaseOpt {
    #[clap(subcommand)]
    pub command: DatabaseCommand,
}

#[derive(Parser, Debug, Clone)]
pub enum DatabaseCommand{
    Init(DatabaseConfig),
    Drop(DatabaseConfig),
}

#[derive(Parser, Debug, Clone)]
pub struct DatabaseConfig{
    #[arg(long)]
    pub host: Option<String>,
    /// Port number of the database server.
    #[arg(long)]
    pub port: Option<u16>,
    /// Username for connecting to the database.
    #[arg(long)]
    pub username: Option<String>,
    /// Name of the database.
    #[arg(long)]
    pub database: Option<String>,
    /// Password for connecting to the database.
    #[arg(long)]
    pub password: Option<String>,
    /// Directory path to the schema files.
    #[arg(long)]
    pub schema: Option<String>,
}

