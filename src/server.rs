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

fn default_200() -> usize {
    200
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UnreadParams {
    /// 返回数量上限，默认 20
    #[serde(default = "default_20")]
    pub limit: usize,
    /// 按会话类型过滤（可选）：private / group / official / folded / all
    pub filter: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MembersParams {
    /// 群聊会话 ID 或显示名称（必填）
    pub chat: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NewMessagesParams {
    /// 上次检查时各会话的 last_timestamp 快照（首次调用时省略）
    pub state: Option<std::collections::HashMap<String, i64>>,
    /// 返回数量上限，默认 200
    #[serde(default = "default_200")]
    pub limit: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StatsParams {
    /// 会话 ID 或显示名称（必填）
    pub chat: String,
    /// 统计起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 统计结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
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

    #[tool(description = "返回有未读消息的会话列表，可按类型过滤（private/group/official）。")]
    async fn wx_unread(
        &self,
        Parameters(p): Parameters<UnreadParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Unread {
            limit: p.limit,
            filter: p.filter,
            with_meta: false,
            debug_source: false,
        })
        .await
        .and_then(to_tool_result)
    }

    #[tool(description = "获取群聊成员列表，包含成员的 username 和显示名称。")]
    async fn wx_members(
        &self,
        Parameters(p): Parameters<MembersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Members { chat: p.chat })
            .await
            .and_then(to_tool_result)
    }

    #[tool(
        description = "获取自上次检查以来的新消息。首次调用不传 state，保存返回的 new_state 供下次使用。"
    )]
    async fn wx_new_messages(
        &self,
        Parameters(p): Parameters<NewMessagesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::NewMessages {
            state: p.state,
            limit: p.limit,
            with_meta: false,
            debug_source: false,
        })
        .await
        .and_then(to_tool_result)
    }

    #[tool(description = "查看指定会话的聊天统计：消息总数、活跃时段分布等。")]
    async fn wx_stats(
        &self,
        Parameters(p): Parameters<StatsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Stats {
            chat: p.chat,
            since: p.since,
            until: p.until,
            with_meta: false,
            debug_source: false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn unread_params_use_default_limit_and_optional_filter() {
        let params: UnreadParams = serde_json::from_value(json!({})).unwrap();

        assert_eq!(params.limit, 20);
        assert_eq!(params.filter, None);
    }

    #[test]
    fn new_messages_params_default_limit_is_200_and_preserves_state() {
        let mut state = HashMap::new();
        state.insert("chat-1".to_string(), 123_i64);
        let params: NewMessagesParams = serde_json::from_value(json!({
            "state": state,
        }))
        .unwrap();

        assert_eq!(params.limit, 200);
        assert_eq!(params.state.unwrap().get("chat-1"), Some(&123_i64));
    }

    #[test]
    fn stats_params_accept_optional_since_until() {
        let params: StatsParams = serde_json::from_value(json!({
            "chat": "demo-chat",
            "since": 10,
            "until": 20,
        }))
        .unwrap();

        assert_eq!(params.chat, "demo-chat");
        assert_eq!(params.since, Some(10));
        assert_eq!(params.until, Some(20));
    }
}
