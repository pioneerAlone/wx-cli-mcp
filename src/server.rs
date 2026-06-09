use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{CallToolResult, Content, ErrorCode};
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};
use serde_json::Value;

use crate::ipc_client::ipc_send;
use crate::ipc_types::Request;

async fn ipc_query(req: Request) -> Result<Value, ErrorData> {
    tokio::task::spawn_blocking(move || ipc_send(req))
        .await
        .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?
        .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))
        .and_then(|resp| {
            if resp.ok {
                Ok(resp.data)
            } else {
                Err(ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    resp.error.unwrap_or_else(|| "daemon error".into()),
                    None,
                ))
            }
        })
}

fn to_tool_result(data: Value) -> Result<CallToolResult, ErrorData> {
    let text = serde_json::to_string_pretty(&data).unwrap_or_else(|_| data.to_string());
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[derive(Clone)]
pub struct WxServer {
    tool_router: ToolRouter<Self>,
}

impl WxServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for WxServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl WxServer {
    #[tool(description = "ping wx-daemon，测试 MCP server 与 daemon 的连接是否正常。")]
    async fn wx_ping(&self) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Ping).await.and_then(to_tool_result)
    }
}

#[tool_handler(
    router = self.tool_router,
    name = "wx-mcp",
    version = "0.1.0",
    instructions = "本地微信数据 MCP server。需要先运行 wx daemon start 启动 daemon。"
)]
impl ServerHandler for WxServer {}
