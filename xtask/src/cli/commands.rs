use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum XtaskCommands {
    Greet { name: String },
}
