use clap::{AppSettings, Parser, Subcommand};

pub mod agents;
pub mod exec;
pub mod identity;

#[derive(Parser)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all agents
    Agents,
    /// Execute a command
    Exec {
        /// The agent id to execute the command on
        #[clap(short, long = "agent")]
        agent_id: String,
        /// The command to execute, with its arguments
        command: String,
    },
    /// Generates a new identity keypair
    Identity,
}
