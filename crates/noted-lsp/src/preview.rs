use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};

/// Shared state for the preview server.
#[derive(Clone)]
pub struct PreviewState {
    /// Current rendered HTML body (updated on each document change).
    pub html: Arc<RwLock<String>>,
    /// Broadcast channel to notify all connected WebSocket clients of updates.
    pub tx: broadcast::Sender<String>,
}

impl PreviewState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self {
            html: Arc::new(RwLock::new(String::new())),
            tx,
        }
    }

    /// Update the preview HTML and notify all connected clients.
    pub async fn update(&self, html: String) {
        *self.html.write().await = html.clone();
        // Ignore send errors (no receivers connected)
        let _ = self.tx.send(html);
    }
}

/// Start the preview HTTP server on a random available port.
/// Returns the address the server is listening on.
pub async fn start_preview_server(
    state: PreviewState,
) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .route("/css", get(css_handler))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    Ok(addr)
}

/// Serve the preview HTML page with embedded WebSocket client.
async fn index_handler(State(state): State<PreviewState>) -> impl IntoResponse {
    let content = state.html.read().await.clone();
    Html(render_preview_page(&content))
}

/// WebSocket upgrade handler — streams HTML updates to the browser.
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<PreviewState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, state: PreviewState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Send current content immediately on connect
    let current = state.html.read().await.clone();
    if !current.is_empty() {
        let _ = sender.send(Message::Text(current.into())).await;
    }

    // Forward broadcast updates to this WebSocket client
    let mut send_task = tokio::spawn(async move {
        while let Ok(html) = rx.recv().await {
            if sender.send(Message::Text(html.into())).await.is_err() {
                break;
            }
        }
    });

    // Consume incoming messages (keep connection alive, handle close)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if matches!(msg, Message::Close(_)) {
                break;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

/// Wrap HTML content in a full preview page with WebSocket client script.
fn render_preview_page(content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Noted Preview</title>
<link rel="stylesheet" href="/css">
</head>
<body>
<div id="status"></div>
<article id="content">{content}</article>
<script>
{WS_CLIENT_JS}
</script>
</body>
</html>"#,
        content = content,
        WS_CLIENT_JS = WS_CLIENT_JS,
    )
}

/// CSS endpoint — served separately so browsers can cache it.
pub async fn css_handler() -> impl IntoResponse {
    ([("content-type", "text/css; charset=utf-8")], PREVIEW_CSS)
}

/// Preview CSS — Verdant Garden light palette, optimized for Markdown reading.
const PREVIEW_CSS: &str = r#"
/* ── Base ──────────────────────────────────────────────────────────── */
:root {
  --bg: #fcfcfa;
  --fg: #2a2e22;
  --muted: #8a8c82;
  --faint: #c0c2b8;
  --border: #e4e6de;
  --accent: #4d8a28;
  --accent-light: #dce8d4;
  --code-bg: #eeefe8;
  --code-fg: #9a7018;
  --link: #2e7e8c;
  --tag-bg: #f4ede2;
  --tag-fg: #8e6420;
  --callout-note: #306a96;
  --callout-tip: #4d8a28;
  --callout-warning: #9a7018;
  --callout-danger: #a03030;
  --callout-important: #8a3850;
  --callout-info: #306a96;
  --callout-question: #645088;
  --callout-example: #4d8a28;
  --callout-quote: #6a6e62;
  --callout-abstract: #2e7e8c;
  --max-width: 800px;
}
* { box-sizing: border-box; }
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 1.5rem 1rem 4rem;
  line-height: 1.7;
  color: var(--fg);
  background: var(--bg);
  -webkit-font-smoothing: antialiased;
}

/* ── Status bar ────────────────────────────────────────────────────── */
#status {
  position: fixed; top: 0; left: 0; right: 0;
  text-align: center; font-size: 0.75rem;
  padding: 2px 0; z-index: 100;
  transition: opacity 0.3s;
}
#status.connected { background: var(--accent); color: #fff; opacity: 0; }
#status.connecting { background: var(--callout-warning); color: #fff; opacity: 1; }
#status.disconnected { background: var(--callout-danger); color: #fff; opacity: 1; }

/* ── Typography ────────────────────────────────────────────────────── */
h1, h2, h3, h4, h5, h6 {
  margin-top: 1.5em; margin-bottom: 0.5em;
  line-height: 1.3; font-weight: 600;
  color: var(--fg);
}
h1 { font-size: 1.8rem; font-weight: 700; border-bottom: 2px solid var(--accent-light); padding-bottom: 0.3em; }
h2 { font-size: 1.4rem; border-bottom: 1px solid var(--border); padding-bottom: 0.2em; }
h3 { font-size: 1.2rem; }
h4 { font-size: 1.05rem; }
h5, h6 { font-size: 1rem; color: var(--muted); }
p { margin: 0.8em 0; }

/* ── Links ─────────────────────────────────────────────────────────── */
a { color: var(--link); text-decoration: none; }
a:hover { text-decoration: underline; }
a.wikilink {
  color: var(--link);
  border-bottom: 1px dashed var(--link);
  padding-bottom: 1px;
}
a.wikilink:hover { border-bottom-style: solid; }

/* ── Code ──────────────────────────────────────────────────────────── */
code {
  font-family: "SF Mono", "Fira Code", "Cascadia Code", "Consolas", monospace;
  font-size: 0.88em;
  background: var(--code-bg);
  color: var(--code-fg);
  padding: 0.15em 0.35em;
  border-radius: 4px;
}
pre {
  background: var(--code-bg);
  padding: 1em 1.2em;
  border-radius: 6px;
  overflow-x: auto;
  line-height: 1.5;
  border: 1px solid var(--border);
}
pre code { background: none; padding: 0; color: var(--fg); font-size: 0.85em; }

/* ── Blockquotes ───────────────────────────────────────────────────── */
blockquote {
  margin: 1em 0;
  padding: 0.5em 1em;
  border-left: 3px solid var(--accent);
  color: var(--muted);
  background: var(--code-bg);
  border-radius: 0 4px 4px 0;
}
blockquote p { margin: 0.4em 0; }

/* ── Callouts ──────────────────────────────────────────────────────── */
.callout {
  margin: 1em 0;
  padding: 0.75em 1em;
  border-radius: 6px;
  border-left: 4px solid var(--accent);
  background: var(--code-bg);
}
.callout-title {
  font-weight: 600;
  margin: 0 0 0.4em 0;
}
.callout-note      { border-left-color: var(--callout-note); }
.callout-info      { border-left-color: var(--callout-info); }
.callout-tip       { border-left-color: var(--callout-tip); }
.callout-example   { border-left-color: var(--callout-example); }
.callout-abstract  { border-left-color: var(--callout-abstract); }
.callout-question  { border-left-color: var(--callout-question); }
.callout-quote     { border-left-color: var(--callout-quote); }
.callout-warning   { border-left-color: var(--callout-warning); background: #faf5ec; }
.callout-danger    { border-left-color: var(--callout-danger); background: #faf0f0; }
.callout-important { border-left-color: var(--callout-important); background: #f8f0f2; }
.callout-note .callout-title      { color: var(--callout-note); }
.callout-info .callout-title      { color: var(--callout-info); }
.callout-tip .callout-title       { color: var(--callout-tip); }
.callout-example .callout-title   { color: var(--callout-example); }
.callout-abstract .callout-title  { color: var(--callout-abstract); }
.callout-question .callout-title  { color: var(--callout-question); }
.callout-quote .callout-title     { color: var(--callout-quote); }
.callout-warning .callout-title   { color: var(--callout-warning); }
.callout-danger .callout-title    { color: var(--callout-danger); }
.callout-important .callout-title { color: var(--callout-important); }

/* ── Tables ────────────────────────────────────────────────────────── */
table { border-collapse: collapse; width: 100%; margin: 1em 0; }
th, td { border: 1px solid var(--border); padding: 0.5em 0.75em; text-align: left; }
th { background: var(--code-bg); font-weight: 600; }
tr:nth-child(even) { background: #f8f9f4; }

/* ── Lists ─────────────────────────────────────────────────────────── */
ul, ol { padding-left: 1.5em; }
li { margin: 0.25em 0; }
li > ul, li > ol { margin: 0.15em 0; }
.task-list-item { list-style: none; margin-left: -1.5em; }
.task-list-item input[type="checkbox"] { margin-right: 0.5em; vertical-align: middle; }

/* ── Horizontal rule ───────────────────────────────────────────────── */
hr { border: none; border-top: 1px solid var(--border); margin: 2em 0; }

/* ── Images ────────────────────────────────────────────────────────── */
img { max-width: 100%; border-radius: 4px; }

/* ── Tags (rendered as spans by future enhancement) ────────────────── */
.tag {
  display: inline-block;
  background: var(--tag-bg);
  color: var(--tag-fg);
  font-size: 0.85em;
  padding: 0.1em 0.5em;
  border-radius: 3px;
}

/* ── Math ──────────────────────────────────────────────────────────── */
.math, math { font-style: italic; color: var(--accent); }

/* ── Strikethrough ─────────────────────────────────────────────────── */
del { color: var(--muted); text-decoration: line-through; }

/* ── Emphasis ──────────────────────────────────────────────────────── */
strong { font-weight: 700; }
em { font-style: italic; }

/* ── Print ─────────────────────────────────────────────────────────── */
@media print {
  body { max-width: none; padding: 0; }
  #status { display: none; }
  pre { border: 1px solid #ccc; }
  a.wikilink { border-bottom: none; }
}
"#;

/// WebSocket client JavaScript — handles connection, reconnection, and content updates.
const WS_CLIENT_JS: &str = r#"
(function() {
  var content = document.getElementById('content');
  var status = document.getElementById('status');
  var ws;
  var reconnectDelay = 1000;
  var maxReconnectDelay = 10000;

  function setStatus(state, text) {
    status.className = state;
    status.textContent = text;
    if (state === 'connected') {
      setTimeout(function() { status.style.opacity = '0'; }, 1500);
    } else {
      status.style.opacity = '1';
    }
  }

  function connect() {
    setStatus('connecting', 'Connecting...');
    ws = new WebSocket('ws://' + location.host + '/ws');

    ws.onopen = function() {
      setStatus('connected', 'Connected');
      reconnectDelay = 1000;
    };

    ws.onmessage = function(event) {
      var data = event.data;

      // JSON message: { type: "update", html: "..." } or { type: "scroll", line: N }
      if (data.charAt(0) === '{') {
        try {
          var msg = JSON.parse(data);
          if (msg.type === 'scroll' && msg.line != null) {
            scrollToLine(msg.line);
            return;
          }
          if (msg.type === 'update' && msg.html != null) {
            updateContent(msg.html);
            return;
          }
        } catch(e) {}
      }

      // Plain HTML string (backward compat)
      updateContent(data);
    };

    ws.onclose = function() {
      setStatus('disconnected', 'Disconnected — reconnecting...');
      setTimeout(function() {
        reconnectDelay = Math.min(reconnectDelay * 1.5, maxReconnectDelay);
        connect();
      }, reconnectDelay);
    };

    ws.onerror = function() {
      ws.close();
    };
  }

  function updateContent(html) {
    var scrollY = window.scrollY;
    content.innerHTML = html;
    window.scrollTo(0, scrollY);
  }

  function scrollToLine(line) {
    var target = document.querySelector('[data-line="' + line + '"]');
    if (target) {
      target.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  }

  connect();
})();
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_preview_page_contains_content() {
        let html = render_preview_page("<h1>Hello</h1>");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("WebSocket"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_render_preview_page_empty_content() {
        let html = render_preview_page("");
        assert!(html.contains("<article id=\"content\"></article>"));
    }

    #[tokio::test]
    async fn test_preview_state_update() {
        let state = PreviewState::new();
        let mut rx = state.tx.subscribe();

        state.update("<p>test</p>".to_string()).await;

        assert_eq!(*state.html.read().await, "<p>test</p>");
        assert_eq!(rx.recv().await.unwrap(), "<p>test</p>");
    }

    #[tokio::test]
    async fn test_start_preview_server_binds() {
        let state = PreviewState::new();
        let addr = start_preview_server(state).await.unwrap();
        assert!(addr.port() > 0);
        assert_eq!(addr.ip(), std::net::Ipv4Addr::LOCALHOST);
    }
}
