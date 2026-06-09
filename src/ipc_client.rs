use crate::ipc_types::{Request, Response};
use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};

/// Returns ~/.wx-cli/daemon.sock path (Unix only).
#[cfg(unix)]
pub fn sock_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".wx-cli")
        .join("daemon.sock")
}

/// One-shot synchronous IPC query.
/// Call this from async code via: `tokio::task::spawn_blocking(move || ipc_send(req)).await??`
pub fn ipc_send(req: Request) -> Result<Response> {
    #[cfg(unix)]
    return send_unix(req);

    #[cfg(windows)]
    return send_windows(req);

    #[cfg(not(any(unix, windows)))]
    anyhow::bail!("unsupported platform")
}

#[cfg(unix)]
fn send_unix(req: Request) -> Result<Response> {
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    let path = sock_path();
    let mut stream = UnixStream::connect(&path).with_context(|| {
        format!(
            "cannot connect to wx-daemon at {}. Please run: wx daemon start",
            path.display()
        )
    })?;
    stream.set_read_timeout(Some(Duration::from_secs(120))).ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(120)))
        .ok();

    let req_str = serde_json::to_string(&req)? + "\n";
    stream.write_all(req_str.as_bytes())?;

    let mut line = String::new();
    BufReader::new(&stream).read_line(&mut line)?;
    serde_json::from_str(&line).context("failed to parse daemon response")
}

#[cfg(windows)]
fn send_windows(req: Request) -> Result<Response> {
    use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream};

    let name = "wx-cli-daemon"
        .to_ns_name::<GenericNamespaced>()
        .context("failed to construct pipe name")?;
    let stream = Stream::connect(name)
        .context("cannot connect to wx-daemon named pipe. Please run: wx daemon start")?;
    let mut reader = BufReader::new(stream);

    let req_str = serde_json::to_string(&req)? + "\n";
    reader.get_mut().write_all(req_str.as_bytes())?;

    let mut line = String::new();
    reader.read_line(&mut line)?;
    serde_json::from_str(&line).context("failed to parse daemon response")
}
