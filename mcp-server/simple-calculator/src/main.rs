use anyhow::Result;
use mylib::calculator::Calculator;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use rmcp::transport::stdio;

use tracing_subscriber::{self, EnvFilter};
mod mylib;

/// npx @modelcontextprotocol/inspector cargo run -p mcp-server-examples --example servers_calculator_stdio
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Simple MCP server");

    // Create an instance of our calculator router
    let calculator = Calculator::new();
    let service = calculator
        .serve(stdio())
        .await
        .map_err(|e| {
            tracing::error!("serving error: {:?}", e);
            e
        })?;
    service.waiting().await?;
    Ok(())
}
