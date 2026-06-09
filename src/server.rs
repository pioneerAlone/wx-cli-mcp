// placeholder - will be filled in Task 4
use rmcp::{
    ServerHandler,
    model::{Implementation, ServerCapabilities, ServerInfo},
};

#[derive(Clone)]
pub struct WxServer;

impl ServerHandler for WxServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::default())
            .with_server_info(Implementation::new("wx-mcp", env!("CARGO_PKG_VERSION")))
    }
}
