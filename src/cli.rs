use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "serve-md")]
#[command(about = "Serve Markdown files as a clean, navigable local website")]
#[command(version)]
pub struct Args {
    /// Directory to serve (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Port to listen on (tries next available if taken)
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Open the browser automatically
    #[arg(short, long)]
    pub open: bool,

    /// Network interface to bind to (e.g., 127.0.0.1, 0.0.0.0)
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind: String,
}
