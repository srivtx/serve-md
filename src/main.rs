use anyhow::Context;
use clap::Parser;
use serve_md::{cli::Args, scan::scan_directory, server::create_app};
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "serve_md=info,tower_http=info".into()),
        )
        .init();

    let args = Args::parse();

    let root = args
        .path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {:?}", args.path))?;

    tracing::info!("Scanning directory: {}", root.display());
    let index = scan_directory(&root).with_context(|| "Failed to scan directory")?;

    tracing::info!(
        "Found {} files, {} directories",
        index.files.len(),
        index.dirs.len()
    );

    let app = create_app(index);

    let bind_addr: IpAddr = args
        .bind
        .parse()
        .with_context(|| format!("Invalid bind address: {}", args.bind))?;

    let listener = find_and_bind(bind_addr, args.port)
        .await
        .with_context(|| "Could not find an available port")?;

    let addr = listener.local_addr()?;
    tracing::info!("Server running at http://{}", addr);

    if args.open {
        let url = format!("http://{}", addr);
        if let Err(e) = open::that(&url) {
            tracing::warn!("Failed to open browser: {}", e);
        }
    }

    tracing::info!("Press Ctrl+C to stop");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server stopped gracefully");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Received shutdown signal");
}

async fn find_and_bind(bind_addr: IpAddr, start_port: u16) -> anyhow::Result<TcpListener> {
    for port in start_port..=start_port.saturating_add(100) {
        if port == 0 {
            continue;
        }
        let addr = SocketAddr::from((bind_addr, port));
        match TcpListener::bind(addr).await {
            Ok(listener) => return Ok(listener),
            Err(_) => continue,
        }
    }
    anyhow::bail!(
        "No available ports found in range {}-{}",
        start_port,
        start_port.saturating_add(100)
    )
}
