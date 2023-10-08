pub(crate) mod commands;

use clap::{command, Parser};

use self::commands::XtaskCommands;

#[derive(Parser)]
#[command(name = "cargo xtask")]
#[command(
    about = "Run custom build and workflow automation commands using `cargo xtask <subcommand>` from the workspace root"
)]
pub(crate) struct Xtask {
    /// Override `RUST_LOG` for the `xtask` crate. If there is no log level set,
    /// `xtask` defaults to Error level. Each `-v` switch increases the log level
    /// by one.
    #[arg(global = true, verbatim_doc_comment, short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
    #[command(subcommand)]
    pub(crate) command: XtaskCommands,
}
