---
name: wx-mcp
description: "wx-mcp — WeChat MCP server. 当用户需要查微信聊天记录、联系人、会话列表、未读消息、群成员、朋友圈、公众号文章、收藏内容时，使用此 skill 配置并调用 wx-mcp MCP server。"
---

# wx-mcp

wx-mcp 是一个本地 MCP (Model Context Protocol) server，将 AI agent 连接到你本地的微信数据。
它通过 wx-cli daemon 的 IPC 协议读取微信数据库，全部在本机运行，数据不离开本地。

## Triggers

- 查微信聊天记录
- 微信未读消息
- 微信联系人
- 微信群成员
- 微信朋友圈
- 微信公众号文章
- 微信收藏
- wechat history / messages / contacts
- wx-mcp
- 帮我看看微信里
- 搜索微信消息

## Prerequisites

1. 已安装 wx-cli 并完成初始化（`wx init`）
2. wx daemon 正在运行（`wx daemon start` 或首次调用时自动启动）
3. 已安装 Node.js >= 14（npx 方式需要）

---

## 安装

### 方式一：npx skills（AI agent 用户推荐）

**两步完成安装：**

**Step 1** — 安装 agent skill（让 agent 识别微信相关触发词）：
```bash
npx skills add pioneerAlone/wx-cli-mcp -g --yes
```

**Step 2** — 配置 MCP server（工具调用必需，见下方 Agent 配置章节）。

> ⚠️ **两步都需要**：`npx skills add` 只安装"知识层"，让 agent 知道有 wx-mcp；实际工具调用还需要配置 MCP server。配置后重启 agent 生效。

### 方式二：npm 全局安装（需要 Node.js >= 14）

```bash
npm install -g @bakewell/wx-mcp
```

更新到最新版：

```bash
npm install -g @bakewell/wx-mcp@latest
```

验证：

```bash
wx-mcp --version
```

安装后配置 MCP server（见下方 Agent 配置章节），重启 agent 生效。

### 方式三：curl（macOS / Linux）

```bash
curl -fsSL https://raw.githubusercontent.com/pioneerAlone/wx-cli-mcp/main/install.sh | bash
```

### 方式四：PowerShell（Windows）

```powershell
irm https://raw.githubusercontent.com/pioneerAlone/wx-cli-mcp/main/install.ps1 | iex
```

---

## Agent 配置

### GitHub Copilot CLI

编辑 `~/.copilot/mcp-config.json`（Windows: `C:\Users\<用户名>\.copilot\mcp-config.json`）：

```json
{
  "mcpServers": {
    "wx": {
      "command": "npx",
      "args": ["-y", "@bakewell/wx-mcp"]
    }
  }
}
```

重启 Copilot CLI 生效。

### Claude Code

```bash
claude mcp add wx -- npx -y @bakewell/wx-mcp
```

或编辑 `~/.claude.json`：

```json
{
  "mcpServers": {
    "wx": {
      "command": "npx",
      "args": ["-y", "@bakewell/wx-mcp"]
    }
  }
}
```

### WorkBuddy

编辑 `~/.workbuddy/mcp.json`：

```json
{
  "mcpServers": {
    "wx": {
      "command": "npx",
      "args": ["-y", "@bakewell/wx-mcp"]
    }
  }
}
```

> **说明**：`npx -y @bakewell/wx-mcp` 每次启动时自动使用 npm 上的最新版本，无需手动更新。如已通过 `npm install -g` 全局安装，也可直接使用 `"command": "wx-mcp"`。

---

## 工具列表

wx-mcp 向 AI agent 暴露以下 14 个工具，覆盖微信数据的全部核心功能。

---

### `wx_ping` — 测试连接

测试 MCP server 与 wx daemon 的连接是否正常。无参数。

```
# 测试连接
工具: wx_ping
参数: {}
```

---

### `wx_sessions` — 列出会话

列出最近的微信会话，包含会话名称、未读数量、最新消息时间。

**参数：**
- `limit`（可选，默认 20）：返回会话数量上限

```
# 列出最近 20 个会话
工具: wx_sessions
参数: {}

# 列出最近 50 个会话
工具: wx_sessions
参数: { "limit": 50 }
```

---

### `wx_history` — 聊天记录

获取指定会话的聊天记录。支持昵称/备注名模糊匹配。

**参数：**
- `chat`（必填）：会话 ID 或显示名称
- `limit`（可选，默认 50）：返回消息数量
- `offset`（可选，默认 0）：分页偏移
- `since`（可选）：起始时间 Unix timestamp
- `until`（可选）：结束时间 Unix timestamp
- `msg_type`（可选）：1=文字 3=图片 34=语音 43=视频

```
# 获取"张三"最近 50 条消息
工具: wx_history
参数: { "chat": "张三" }

# 获取"AI群"4月的消息（最多 200 条）
工具: wx_history
参数: { "chat": "AI群", "since": 1743436800, "until": 1746028800, "limit": 200 }

# 只看图片消息
工具: wx_history
参数: { "chat": "张三", "msg_type": 3, "limit": 20 }
```

---

### `wx_search` — 全局搜索

全局搜索微信消息，按关键词匹配正文，可限定会话范围和时间范围。

**参数：**
- `keyword`（必填）：搜索关键词
- `chats`（可选）：限定搜索的会话列表（数组）
- `limit`（可选，默认 20）：返回结果数量上限
- `since`（可选）：起始时间 Unix timestamp
- `until`（可选）：结束时间 Unix timestamp
- `msg_type`（可选）：消息类型过滤

```
# 搜索所有会话里含"发票"的消息
工具: wx_search
参数: { "keyword": "发票" }

# 在"项目群"里搜索"会议"，最多 50 条
工具: wx_search
参数: { "keyword": "会议", "chats": ["项目群"], "limit": 50 }

# 搜索本月的"合同"消息
工具: wx_search
参数: { "keyword": "合同", "since": 1748736000, "limit": 100 }
```

---

### `wx_contacts` — 联系人

查询本地微信联系人，支持按名称/备注模糊过滤。

**参数：**
- `query`（可选）：按名称/备注模糊过滤
- `limit`（可选，默认 50）：返回数量上限

```
# 列出所有联系人
工具: wx_contacts
参数: {}

# 搜索含"李"的联系人
工具: wx_contacts
参数: { "query": "李" }
```

---

### `wx_unread` — 未读消息

返回有未读消息的会话列表，可按类型过滤（过滤公众号、折叠群等）。

**参数：**
- `limit`（可选，默认 20）：返回数量上限
- `filter`（可选）：按类型过滤，值为数组，可选项：`private` / `group` / `official` / `folded` / `all`

```
# 查看所有未读会话
工具: wx_unread
参数: {}

# 只看真人未读（私聊 + 群聊，过滤公众号和折叠入口）
工具: wx_unread
参数: { "filter": ["private", "group"] }
```

---

### `wx_members` — 群成员

获取群聊成员列表，包含成员的 username 和显示名称（群昵称 > 备注名 > 微信昵称）。

**参数：**
- `chat`（必填）：群聊会话 ID 或显示名称

```
# 查看"AI交流群"的成员
工具: wx_members
参数: { "chat": "AI交流群" }
```

---

### `wx_new_messages` — 增量新消息

获取自上次检查以来的新消息。首次调用不传 `state`，把返回的 `new_state` 存下来，下次传入实现增量轮询。

**参数：**
- `state`（可选）：上次检查时各会话的 `last_timestamp` 快照（首次调用时省略）
- `limit`（可选，默认 200）：返回数量上限

```
# 首次检查新消息
工具: wx_new_messages
参数: {}

# 增量检查（使用上次返回的 new_state）
工具: wx_new_messages
参数: { "state": { "wxid_abc@chatroom": 1748900000, "wxid_xyz": 1748899000 } }
```

---

### `wx_stats` — 聊天统计

查看指定会话的聊天统计：消息总数、活跃时段分布、发言人 Top N 等。

**参数：**
- `chat`（必填）：会话 ID 或显示名称
- `since`（可选）：统计起始时间 Unix timestamp
- `until`（可选）：统计结束时间 Unix timestamp

```
# 查看"项目群"的全部统计
工具: wx_stats
参数: { "chat": "项目群" }

# 查看今年的统计
工具: wx_stats
参数: { "chat": "项目群", "since": 1735660800 }
```

---

### `wx_favorites` — 收藏内容

查看微信收藏内容，支持按类型和关键词过滤。

**参数：**
- `limit`（可选，默认 50）：返回数量上限
- `fav_type`（可选）：1=文本 2=图片 5=文章 19=名片 20=视频
- `query`（可选）：内容关键词搜索

```
# 查看所有收藏
工具: wx_favorites
参数: {}

# 只看收藏的文章
工具: wx_favorites
参数: { "fav_type": 5 }

# 搜索含"合同"的收藏
工具: wx_favorites
参数: { "query": "合同" }
```

---

### `wx_sns_feed` — 朋友圈时间线

浏览朋友圈时间线，可按时间范围和作者过滤。**只包含本地刷到过的帖子。**

**参数：**
- `limit`（可选，默认 20）：返回数量上限
- `since`（可选）：起始时间 Unix timestamp
- `until`（可选）：结束时间 Unix timestamp
- `user`（可选）：按作者过滤（昵称/备注名/微信 ID，模糊匹配）

```
# 最近 20 条朋友圈
工具: wx_sns_feed
参数: {}

# 只看"张三"的朋友圈
工具: wx_sns_feed
参数: { "user": "张三" }

# 查看本周的朋友圈（最多 50 条）
工具: wx_sns_feed
参数: { "since": 1748649600, "limit": 50 }
```

---

### `wx_sns_search` — 朋友圈搜索

全文搜索朋友圈内容，匹配帖子正文中的关键词。

**参数：**
- `keyword`（必填）：朋友圈正文关键词
- `limit`（可选，默认 20）：返回数量上限
- `since`（可选）：起始时间 Unix timestamp
- `until`（可选）：结束时间 Unix timestamp
- `user`（可选）：限定作者（模糊匹配）

```
# 搜索含"旅行"的朋友圈
工具: wx_sns_search
参数: { "keyword": "旅行" }

# 搜索"李四"发的含"婚礼"的帖子
工具: wx_sns_search
参数: { "keyword": "婚礼", "user": "李四" }

# 搜索去年的"年会"帖子
工具: wx_sns_search
参数: { "keyword": "年会", "since": 1704038400, "until": 1735660800 }
```

---

### `wx_sns_notifications` — 朋友圈互动通知

查看朋友圈互动通知：别人对我的朋友圈点赞/评论，以及我评论过的帖子的后续回复。

**参数：**
- `limit`（可选，默认 50）：返回数量上限
- `since`（可选）：起始时间 Unix timestamp
- `until`（可选）：结束时间 Unix timestamp
- `include_read`（可选，默认 false）：是否包含已读通知

```
# 查看未读的朋友圈通知
工具: wx_sns_notifications
参数: {}

# 查看包含已读的最近所有通知（最多 100 条）
工具: wx_sns_notifications
参数: { "include_read": true, "limit": 100 }

# 查看今天的互动通知
工具: wx_sns_notifications
参数: { "since": 1748908800, "include_read": true }
```

---

### `wx_biz_articles` — 公众号文章

查询本地缓存的公众号文章推送。可按公众号名称过滤，也可只看未读。

**参数：**
- `limit`（可选，默认 50）：返回数量上限
- `account`（可选）：公众号名称模糊过滤
- `since`（可选）：起始时间 Unix timestamp（文章发布时间）
- `until`（可选）：结束时间 Unix timestamp
- `unread`（可选，默认 false）：只看有未读推送的公众号

```
# 查看最近 50 篇公众号文章
工具: wx_biz_articles
参数: {}

# 查看"返朴"公众号的文章
工具: wx_biz_articles
参数: { "account": "返朴" }

# 查看今天有哪些公众号有新推送
工具: wx_biz_articles
参数: { "unread": true }

# 查看本月的公众号文章（最多 200 篇）
工具: wx_biz_articles
参数: { "since": 1748736000, "limit": 200 }
```

---

## 常见问题

**wx-mcp 启动失败 / 连接超时**：确认 `wx daemon start` 已运行，或先执行 `wx daemon status` 检查状态。

**消息数量不够**：增大 `limit` 参数，默认值只是上限，不是硬限制。

**找不到某个会话**：先用 `wx_contacts` 搜索联系人名，或用 `wx_sessions` 列出所有会话确认确切名称。

**Windows 下 `wx-mcp` 找不到**：改用 npx 方式配置，`"command": "npx", "args": ["-y", "@bakewell/wx-mcp"]`，无需本地安装二进制。

**微信重启后密钥失效**：重新运行 `sudo wx init --force`（微信必须正在运行）。
