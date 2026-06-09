// SYNC: Keep in sync with wx-cli/src/ipc.rs
// When wx-cli adds new IPC commands, mirror them here.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Request {
    Ping,
    Sessions {
        #[serde(default = "default_limit_20")]
        limit: usize,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    History {
        chat: String,
        #[serde(default = "default_limit_50")]
        limit: usize,
        #[serde(default)]
        offset: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        msg_type: Option<i64>,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    Search {
        keyword: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        chats: Option<Vec<String>>,
        #[serde(default = "default_limit_20")]
        limit: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        msg_type: Option<i64>,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    Contacts {
        #[serde(skip_serializing_if = "Option::is_none")]
        query: Option<String>,
        #[serde(default = "default_limit_50")]
        limit: usize,
    },
    Unread {
        #[serde(default = "default_limit_20")]
        limit: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filter: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    Members {
        chat: String,
    },
    NewMessages {
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<HashMap<String, i64>>,
        #[serde(default = "default_limit_200")]
        limit: usize,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    Stats {
        chat: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    Favorites {
        #[serde(default = "default_limit_50")]
        limit: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        fav_type: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        query: Option<String>,
    },
    SnsNotifications {
        #[serde(default = "default_limit_50")]
        limit: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(default)]
        include_read: bool,
    },
    SnsFeed {
        #[serde(default = "default_limit_20")]
        limit: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<String>,
    },
    BizArticles {
        #[serde(default = "default_limit_50")]
        limit: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        account: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(default)]
        unread: bool,
    },
    SnsSearch {
        keyword: String,
        #[serde(default = "default_limit_20")]
        limit: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<String>,
    },
    ReloadConfig,
    Attachments {
        chat: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kinds: Option<Vec<String>>,
        #[serde(default = "default_limit_50")]
        limit: usize,
        #[serde(default)]
        offset: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        since: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<i64>,
        #[serde(default, skip_serializing_if = "is_false")]
        with_meta: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        debug_source: bool,
    },
    Extract {
        attachment_id: String,
        output: String,
        #[serde(default)]
        overwrite: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub data: Value,
}

fn default_limit_20() -> usize { 20 }
fn default_limit_50() -> usize { 50 }
fn default_limit_200() -> usize { 200 }
fn is_false(v: &bool) -> bool { !*v }
