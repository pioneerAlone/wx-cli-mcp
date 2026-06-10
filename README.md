# wx-mcp

MCP (Model Context Protocol) server for [wx-cli](https://github.com/jackwener/wx-cli).
Bridges AI agents to your local WeChat data via the wx-daemon IPC protocol.

## Prerequisites

1. Install wx-cli and run `wx init` (to initialize local WeChat data access)
2. Start daemon: `wx daemon start`
3. Keep daemon running while using AI agents

## Quick Start

```bash
# Step 1: Install wx-mcp
npm install -g @bakewell/wx-mcp

# Step 2: Configure your AI agent (see "AI Agent Configuration" below)

# Step 3 (optional): Install agent skill for trigger-word recognition
npx skills add pioneerAlone/wx-cli-mcp
```

> **Note**: Both Step 1 and Step 2 are required. `npx skills add` alone is not enough — it only teaches the agent what wx-mcp can do, but the MCP server must be configured separately for tools to actually work.

## Installation

### Option 1: npm (recommended, requires Node.js >= 14)

```bash
npm install -g @bakewell/wx-mcp
```

**Update to latest:**
```bash
npm install -g @bakewell/wx-mcp@latest
```

**Verify:**
```bash
wx-mcp --version
```

### Option 2: curl (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/pioneerAlone/wx-cli-mcp/main/install.sh | bash
```

### Option 3: PowerShell (Windows)

```powershell
irm https://raw.githubusercontent.com/pioneerAlone/wx-cli-mcp/main/install.ps1 | iex
```

### Option 4: GitHub Releases

Download the pre-built binary for your platform from [Releases](https://github.com/pioneerAlone/wx-cli-mcp/releases).

| Platform | File |
|----------|------|
| macOS Apple Silicon | `wx-mcp-macos-arm64` |
| macOS Intel | `wx-mcp-macos-x86_64` |
| Linux x86_64 | `wx-mcp-linux-x86_64` |
| Linux ARM64 | `wx-mcp-linux-arm64` |
| Windows x64 | `wx-mcp-windows-x64.exe` |

<details>
<summary>Build from source</summary>

```bash
git clone https://github.com/pioneerAlone/wx-cli-mcp
cd wx-cli-mcp
cargo build --release
# Add to PATH:
#   Windows: copy target\release\wx-mcp.exe to a directory in PATH
#   Linux/macOS: cp target/release/wx-mcp ~/.local/bin/
```

</details>

## AI Agent Configuration

After installing wx-mcp (Step 1), configure your AI agent to use it as an MCP server. All agents support the `npx` command format which always uses the latest version:

### GitHub Copilot CLI

Edit `~/.copilot/mcp-config.json`:
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

### Claude Code

```bash
claude mcp add wx -- npx -y @bakewell/wx-mcp
```

Or in `~/.claude.json`:
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

### WorkBuddy / Tencent CodeBuddy

In `~/.codebuddy/.mcp.json`:
```json
{
  "mcpServers": {
    "wx": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@bakewell/wx-mcp"]
    }
  }
}
```

### Cursor

In `.cursor/mcp.json`:
```json
{
  "mcpServers": {
    "wx-mcp": {
      "command": "wx-mcp"
    }
  }
}
```

## Available Tools

| Tool | Description |
|------|-------------|
| `wx_ping` | Test MCP server ↔ daemon connection |
| `wx_sessions` | List recent WeChat sessions |
| `wx_history` | Get chat history (by session name/ID) |
| `wx_search` | Search messages by keyword |
| `wx_contacts` | Query contacts (fuzzy search) |
| `wx_unread` | Sessions with unread messages |
| `wx_members` | Group chat member list |
| `wx_new_messages` | Incremental new messages since last check |
| `wx_stats` | Chat statistics for a session |
| `wx_favorites` | Browse WeChat favorites |
| `wx_sns_feed` | Moments timeline |
| `wx_sns_search` | Search Moments content |
| `wx_sns_notifications` | Moments interaction notifications |
| `wx_biz_articles` | Official account articles |

## Troubleshooting

**"cannot connect to wx-daemon"**: The daemon is not running. Run `wx daemon start`.

**"daemon error"**: The daemon is running but returned an error. Check `wx daemon status` and wx-cli logs.

**No data returned**: WeChat data may not be initialized. Run `wx init` first.

## ⚠️ Security Notice

`wx init` reads WeChat's local SQLCipher database key from process memory. This technique
may be detected by WeChat's risk-control system and could result in temporary account
restrictions. Use at your own risk.

See [wx-cli issues](https://github.com/jackwener/wx-cli/issues) for community reports.
