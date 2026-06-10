mod ipc_client;
mod ipc_types;
mod server;

use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("wx-mcp {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("wx-mcp {} — WeChat MCP server", env!("CARGO_PKG_VERSION"));
        println!("Usage: configure as an MCP server in your AI agent. See SKILL.md for details.");
        println!("       This process communicates over stdio using the MCP protocol.");
        return Ok(());
    }

    // Logs go to stderr; stdout is the MCP wire
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let server = server::WxServer::new().serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
