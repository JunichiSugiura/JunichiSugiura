use dip::cli::{CliPlugin, SubcommandPlugin};

#[derive(CliPlugin, clap::Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone)]
pub enum Action {
    #[clap(subcommand)]
    Dotfiles(DotfilesAction),
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone)]
pub enum DotfilesAction {
    Install,
    Apply,
}
