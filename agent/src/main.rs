use cfg_iif::cfg_iif;
use daemonize::Daemonize;
#[cfg(debug_assertions)]
use simplelog::*;
use single_instance::SingleInstance;
use std::{env, fs::File, time::Duration};

mod config;
mod error;
mod init;
mod install;
mod run;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    cfg_iif!(target_family="unix" {
        let stdout = File::create(config::AGENT_STDOUT_FILE).unwrap();
        let stderr = File::create(config::AGENT_STDERR_FILE).unwrap();

        let daemonize = Daemonize::new()
            .pid_file(config::AGENT_PID_FILE)
            .working_directory(env::temp_dir())
            .stdout(stdout)
            .stderr(stderr);

        match daemonize.start() {
            Ok(_) => println!("Success, daemonized"),
            Err(e) => eprintln!("Error, {}", e),
        }
    });

    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent(config::USER_AGENT_STRING)
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
