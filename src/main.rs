mod cmd;
mod format;
mod locale;
mod pty;
mod recorder;
mod util;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[command(name = "asciinema")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// asciinema server URL
    #[arg(long)]
    server_url: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Record a terminal session
    #[command(name = "rec")]
    Record(cmd::record::Cli),

    /// Replay a terminal session
    Play(cmd::play::Cli),

    /// Concatenate multiple recordings
    Cat(cmd::cat::Cli),

    /// Upload recording to an asciinema server
    Upload(cmd::upload::Cli),

    /// Authenticate this CLI with an asciinema server account
    Auth(cmd::auth::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let server_url = &cli.server_url;

    match cli.command {
        Commands::Record(record) => record.run(),
        Commands::Play(play) => play.run(),
        Commands::Cat(cat) => cat.run(),
        Commands::Upload(upload) => upload.run(server_url),
        Commands::Auth(auth) => auth.run(server_url),
    }
}