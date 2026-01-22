#![allow(dead_code)]

use anyhow::Result;
use canlib::socketcan::CANServer;
use rmcp::{
    ServerHandler,ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    //transport::stdio
};
use rmcp::transport::stdio;

use tracing_subscriber::{self, EnvFilter};
mod canlib;
/// npx @modelcontextprotocol/inspector cargo run -p socketcan-mcp-server
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Calculator MCP server");

    let vcan = CANServer::new();
    let service = vcan
        .serve(stdio())
        .await
        .map_err(|e| {
            tracing::error!("serving error: {:?}", e);
            e
        })?;
    
    service.waiting().await?;
    Ok(())
}
