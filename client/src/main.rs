use clap::Parser;

mod api;
mod cli;
mod config;
mod error;

use cli::Commands;
pub use error::Error;

use crate::config::Config;

fn main() -> Result<(), anyhow::Error> {
    let cli = cli::Cli::parse();

    let api_client = api::Client::new(config::SERVER_URL.to_string());

    match &cli.command {
        Commands::Agents => {
            cli::agents::run(&api_client)?;
        }
        Commands::Exec { agent_id, command } => {
            let conf = Config::load()?;
            cli::exec::run(&api_client, agent_id, command, conf)?;
        }
        Commands::Identity => {
            cli::identity::run();
        }
    }

    Ok(())
}
