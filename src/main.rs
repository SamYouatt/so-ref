use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::router::tool::ToolRouter,
    model::*,
    tool, tool_handler, tool_router,
    transport::stdio,
    schemars,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct HelloParams {
    name: String,
}

#[derive(Clone)]
struct SoRef {
    tool_router: ToolRouter<SoRef>,
}

#[tool_router]
impl SoRef {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Say hello to someone")]
    fn hello(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<HelloParams>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Hello, {}!",
            params.name
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for SoRef {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("so-ref", env!("CARGO_PKG_VERSION")))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let service = SoRef::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
