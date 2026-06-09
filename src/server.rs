use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, ErrorCode};
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};
use serde::Deserialize;
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

fn default_20() -> usize {
    20
}

fn default_50() -> usize {
    50
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SessionsParams {
    /// 返回会话数量上限，默认 20
    #[serde(default = "default_20")]
    pub limit: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct HistoryParams {
    /// 会话 ID 或显示名称（必填，可从 wx_sessions 获取）
    pub chat: String,
    /// 返回消息数量，默认 50
    #[serde(default = "default_50")]
    pub limit: usize,
    /// 分页偏移，默认 0
    #[serde(default)]
    pub offset: usize,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 消息类型（可选）：1=文字 3=图片 34=语音 43=视频
    pub msg_type: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    /// 搜索关键词（必填）
    pub keyword: String,
    /// 限定搜索的会话列表（可选，为空时搜索全部）
    pub chats: Option<Vec<String>>,
    /// 返回结果数量上限，默认 20
    #[serde(default = "default_20")]
    pub limit: usize,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 消息类型过滤（可选）
    pub msg_type: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactsParams {
    /// 按名称/备注模糊过滤（可选）
    pub query: Option<String>,
    /// 返回数量上限，默认 50
    #[serde(default = "default_50")]
    pub limit: usize,
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

    #[tool(description = "列出最近的微信会话，包含会话名称、未读数量、最新消息时间。")]
    async fn wx_sessions(
        &self,
        Parameters(p): Parameters<SessionsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Sessions {
            limit: p.limit,
            with_meta: false,
            debug_source: false,
        })
        .await
        .and_then(to_tool_result)
    }

    #[tool(description = "获取指定会话的聊天记录。chat 参数使用联系人显示名称或会话 ID。")]
    async fn wx_history(
        &self,
        Parameters(p): Parameters<HistoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::History {
            chat: p.chat,
            limit: p.limit,
            offset: p.offset,
            since: p.since,
            until: p.until,
            msg_type: p.msg_type,
            with_meta: false,
            debug_source: false,
        })
        .await
        .and_then(to_tool_result)
    }

    #[tool(description = "全局搜索微信消息，按关键词匹配正文。可限定会话范围和时间范围。")]
    async fn wx_search(
        &self,
        Parameters(p): Parameters<SearchParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Search {
            keyword: p.keyword,
            chats: p.chats,
            limit: p.limit,
            since: p.since,
            until: p.until,
            msg_type: p.msg_type,
            with_meta: false,
            debug_source: false,
        })
        .await
        .and_then(to_tool_result)
    }

    #[tool(description = "查询本地微信联系人，支持按名称/备注模糊过滤。")]
    async fn wx_contacts(
        &self,
        Parameters(p): Parameters<ContactsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Contacts {
            query: p.query,
            limit: p.limit,
        })
        .await
        .and_then(to_tool_result)
    }
}

#[tool_handler(
    router = self.tool_router,
    name = "wx-mcp",
    version = "0.1.0",
    instructions = "本地微信数据 MCP server。需要先运行 wx daemon start 启动 daemon。"
)]
impl ServerHandler for WxServer {}
