use clap::Parser;
use cli::commands::XtaskCommands;
use env_logger::{Builder, Env};
use tracing::Level;

pub(crate) mod cli;

fn main() -> anyhow::Result<()> {
    let xtask_invocation = cli::Xtask::parse();
    init_env_logger(verbosity_to_log_level(xtask_invocation.verbose));

    match xtask_invocation.command {
        XtaskCommands::Greet { name } => println!("Hello {name}"),
    }
    Ok(())
}

fn init_env_logger(level: Level) {
    Builder::from_env(
        Env::default()
            .default_filter_or(level.to_string().to_ascii_lowercase())
            .default_write_style_or("auto"),
    )
    .init();
}

fn verbosity_to_log_level(verbosity: u8) -> Level {
    match verbosity {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    }
}
