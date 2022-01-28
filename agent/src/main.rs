use single_instance::SingleInstance;
use std::{env, time::Duration};

mod config;
mod error;
mod init;
mod install;
mod run;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch_11_agent/0.1")
        .build();
    let instance = SingleInstance::new(config::SINGLE_INSTANCE_IDENTIFIER).unwrap();
    if !instance.is_single() {
        return Ok(());
    }

    let conf = init::init(&api_client)?;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        install::install()?;
    }
    run::run(&api_client, conf);
}
