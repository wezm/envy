use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "envy", about = "context-based environment variables")]
pub struct Envy {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Export environment variables based on the current directory
    #[structopt(name = "export")]
    Export { shell: String },
    /// Print the hook to activate envy for your shell
    #[structopt(name = "hook")]
    Hook { shell: String },
    /// Edit the envy config file
    #[structopt(name = "edit")]
    Edit {},
    /// Show envy config for current directory
    #[structopt(name = "show")]
    Show {},
    /// Print path to envy config file
    #[structopt(name = "path")]
    Path {},
    /// Grants envy to load the given `.env` file
    #[structopt(name = "allow")]
    Allow {
        #[structopt(parse(from_os_str), default_value = ".env")]
        env_file: PathBuf,
    },
    /// Revokes the authorization of a given `.env` file
    #[structopt(name = "deny")]
    Deny {},
}
