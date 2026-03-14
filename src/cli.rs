use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "dev")]
#[command(about = "Run project commands from dev.yaml")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// Action to run (e.g. run, test, lint). Omit to list all actions.
    pub action: Option<String>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Create a new dev.yaml in the current directory with default actions.
    Init,
}

/// What the user asked to do.
pub enum Intent {
    List,
    Init,
    Run(String),
}

/// Parse argv into an Intent.
pub fn parse() -> Intent {
    let cli = Cli::parse();
    if let Some(Command::Init) = cli.command {
        return Intent::Init;
    }
    if let Some(name) = cli.action {
        return Intent::Run(name);
    }
    Intent::List
}
