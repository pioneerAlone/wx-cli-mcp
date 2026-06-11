use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, ErrorCode};
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};
use serde::Deserialize;

use crate::ipc_client::ipc_send;
use crate::ipc_types::Request;

async fn ipc_query(req: Request) -> Result<CallToolResult, ErrorData> {
    let resp = tokio::task::spawn_blocking(move || ipc_send(req))
        .await
        .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?
        .map_err(|e| ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;

    if resp.ok {
        let text = serde_json::to_string_pretty(&resp.data)
            .unwrap_or_else(|_| resp.data.to_string());
        Ok(CallToolResult::success(vec![Content::text(text)]))
    } else {
        let msg = resp.error.unwrap_or_else(|| "daemon error".into());
        Ok(CallToolResult::error(vec![Content::text(msg)]))
    }
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FavoritesParams {
    /// 返回数量上限，默认 50
    #[serde(default = "default_50")]
    pub limit: usize,
    /// 类型过滤（可选）：1=文本 2=图片 5=文章 19=名片 20=视频
    pub fav_type: Option<i64>,
    /// 内容关键词搜索（可选）
    pub query: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SnsFeedParams {
    /// 返回数量上限，默认 20
    #[serde(default = "default_20")]
    pub limit: usize,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 按作者过滤（昵称/备注名/微信 ID，模糊匹配，可选）
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SnsSearchParams {
    /// 朋友圈正文关键词（必填）
    pub keyword: String,
    /// 返回数量上限，默认 20
    #[serde(default = "default_20")]
    pub limit: usize,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 限定作者（可选）
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SnsNotificationsParams {
    /// 返回数量上限，默认 50
    #[serde(default = "default_50")]
    pub limit: usize,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 是否包含已读通知，默认 false
    #[serde(default)]
    pub include_read: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BizArticlesParams {
    /// 返回数量上限，默认 50
    #[serde(default = "default_50")]
    pub limit: usize,
    /// 公众号名称模糊过滤（可选）
    pub account: Option<String>,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 只看有未读消息的公众号，默认 false
    #[serde(default)]
    pub unread: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AttachmentsParams {
    /// 会话 ID 或显示名称（必填，可从 wx_sessions 结果获取）
    pub chat: String,
    /// 返回数量上限，默认 50
    #[serde(default = "default_50")]
    pub limit: usize,
    /// 分页偏移，默认 0
    #[serde(default)]
    pub offset: usize,
    /// 起始时间 Unix timestamp（可选）
    pub since: Option<i64>,
    /// 结束时间 Unix timestamp（可选）
    pub until: Option<i64>,
    /// 附件类型（可选，当前仅支持 image）
    pub kinds: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExtractParams {
    /// 由 wx_attachments 返回的不透明 attachment_id（base64url 字符串）
    pub attachment_id: String,
    /// 输出文件路径（绝对路径，daemon 直接写盘；扩展名建议保留如 .jpg）
    pub output: String,
    /// 目标已存在时覆盖，默认 false
    #[serde(default)]
    pub overwrite: bool,
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
        ipc_query(Request::Ping).await
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
        
    }

    #[tool(description = "获取群聊成员列表，包含成员的 username 和显示名称。")]
    async fn wx_members(
        &self,
        Parameters(p): Parameters<MembersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Members { chat: p.chat })
            .await
            
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
        
    }

    #[tool(description = "查看微信收藏内容，支持按类型（文字/图片/文章/视频）和关键词过滤。")]
    async fn wx_favorites(
        &self,
        Parameters(p): Parameters<FavoritesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Favorites {
            limit: p.limit,
            fav_type: p.fav_type,
            query: p.query,
        })
        .await
        
    }

    #[tool(description = "浏览朋友圈时间线，可按时间范围和作者过滤。")]
    async fn wx_sns_feed(
        &self,
        Parameters(p): Parameters<SnsFeedParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::SnsFeed {
            limit: p.limit,
            since: p.since,
            until: p.until,
            user: p.user,
        })
        .await
        
    }

    #[tool(description = "全文搜索朋友圈内容，匹配帖子正文中的关键词。")]
    async fn wx_sns_search(
        &self,
        Parameters(p): Parameters<SnsSearchParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::SnsSearch {
            keyword: p.keyword,
            limit: p.limit,
            since: p.since,
            until: p.until,
            user: p.user,
        })
        .await
        
    }

    #[tool(description = "查看朋友圈互动通知：别人对我的朋友圈点赞/评论，以及我评论过的帖子的后续回复。")]
    async fn wx_sns_notifications(
        &self,
        Parameters(p): Parameters<SnsNotificationsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::SnsNotifications {
            limit: p.limit,
            since: p.since,
            until: p.until,
            include_read: p.include_read,
        })
        .await
        
    }

    #[tool(description = "查询本地缓存的公众号文章推送，可按公众号名称过滤，可只看有未读的。")]
    async fn wx_biz_articles(
        &self,
        Parameters(p): Parameters<BizArticlesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::BizArticles {
            limit: p.limit,
            account: p.account,
            since: p.since,
            until: p.until,
            unread: p.unread,
        })
        .await
        
    }

    #[tool(description = "列出某会话的图片附件，返回不透明 attachment_id 列表。两步流程：先 wx_attachments 获取 id，再用 wx_extract 解密写盘。支持时间范围和分页过滤。")]
    async fn wx_attachments(
        &self,
        Parameters(p): Parameters<AttachmentsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Attachments {
            chat: p.chat,
            kinds: p.kinds,
            limit: p.limit,
            offset: p.offset,
            since: p.since,
            until: p.until,
            with_meta: false,
            debug_source: false,
        })
        .await
    }

    #[tool(description = "把单个 attachment_id 对应的资源解密写到指定文件路径。output 为绝对路径，由 daemon 直接写盘。需先用 wx_attachments 获取 attachment_id。返回 format/decoder/output_size 元数据。")]
    async fn wx_extract(
        &self,
        Parameters(p): Parameters<ExtractParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ipc_query(Request::Extract {
            attachment_id: p.attachment_id,
            output: p.output,
            overwrite: p.overwrite,
        })
        .await
    }
}

#[tool_handler(
    router = self.tool_router,
    name = "wx-mcp",
    version = "0.1.1",
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

    #[test]
    fn favorites_params_default_limit_and_optional_filters() {
        let params: FavoritesParams = serde_json::from_value(json!({})).unwrap();

        assert_eq!(params.limit, 50);
        assert_eq!(params.fav_type, None);
        assert_eq!(params.query, None);
    }

    #[test]
    fn sns_feed_params_default_limit_and_optional_filters() {
        let params: SnsFeedParams = serde_json::from_value(json!({
            "user": "alice",
        }))
        .unwrap();

        assert_eq!(params.limit, 20);
        assert_eq!(params.since, None);
        assert_eq!(params.until, None);
        assert_eq!(params.user.as_deref(), Some("alice"));
    }

    #[test]
    fn sns_search_params_require_keyword_and_default_limit() {
        let params: SnsSearchParams = serde_json::from_value(json!({
            "keyword": "coffee",
        }))
        .unwrap();

        assert_eq!(params.keyword, "coffee");
        assert_eq!(params.limit, 20);
        assert_eq!(params.user, None);
    }

    #[test]
    fn sns_notifications_params_default_limit_and_include_read_false() {
        let params: SnsNotificationsParams = serde_json::from_value(json!({})).unwrap();

        assert_eq!(params.limit, 50);
        assert_eq!(params.since, None);
        assert_eq!(params.until, None);
        assert!(!params.include_read);
    }

    #[test]
    fn biz_articles_params_default_limit_and_unread_false() {
        let params: BizArticlesParams = serde_json::from_value(json!({})).unwrap();

        assert_eq!(params.limit, 50);
        assert_eq!(params.account, None);
        assert_eq!(params.since, None);
        assert_eq!(params.until, None);
        assert!(!params.unread);
    }

    #[test]
    fn attachments_params_default_limit_is_50_and_offset_is_0() {
        let params: AttachmentsParams = serde_json::from_value(json!({
            "chat": "张三",
        }))
        .unwrap();

        assert_eq!(params.limit, 50);
        assert_eq!(params.offset, 0);
        assert_eq!(params.kinds, None);
        assert_eq!(params.since, None);
        assert_eq!(params.until, None);
    }

    #[test]
    fn extract_params_default_overwrite_is_false() {
        let params: ExtractParams = serde_json::from_value(json!({
            "attachment_id": "dGVzdA",
            "output": "/tmp/test.jpg",
        }))
        .unwrap();

        assert_eq!(params.attachment_id, "dGVzdA");
        assert_eq!(params.output, "/tmp/test.jpg");
        assert!(!params.overwrite);
    }
}
