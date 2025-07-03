use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CargoSubcommands,
}

#[derive(Parser, Debug)]
pub enum CargoSubcommands {
    /// Forge a new Rust project from templates
    Forge(ForgeCommand),
}

#[derive(Parser, Debug)]
pub struct ForgeCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Create a new project from a template
    New {
        /// The name of the project to create
        name: String,
        /// The template to use (e.g., "cli", "axum-api")
        #[arg(short, long)]
        template: Option<String>,
        /// Enable interactive mode (prompts for options)
        #[arg(short, long)]
        interactive: bool,
    },
}