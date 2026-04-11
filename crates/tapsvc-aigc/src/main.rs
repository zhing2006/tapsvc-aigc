#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod cli;
mod cmd;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file (ignore if not present)
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Run command with graceful shutdown support
    tokio::select! {
        result = run(cli) => result,
        _ = tokio::signal::ctrl_c() => {
            eprintln!("\nInterrupted, shutting down...");
            Ok(())
        }
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Image { command } => cmd::image::handle(command).await,
        Command::Audio { command } => cmd::audio::handle(command).await,
        Command::Video { command } => cmd::video::handle(command).await,
    }
}
