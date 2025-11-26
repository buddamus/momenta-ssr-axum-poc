#[cfg(feature = "server")]
use axum::{
    extract::{WebSocketUpgrade, ws::{WebSocket, Message}, State},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
#[cfg(feature = "server")]
use std::sync::Arc;
#[cfg(feature = "server")]
use tokio::sync::broadcast;
#[cfg(feature = "server")]
use serde_json::json;

#[cfg(feature = "server")]
pub type HotReloadState = Arc<broadcast::Sender<String>>;

#[cfg(feature = "server")]
pub fn setup_hot_reload(app: Router) -> Router {
    let state = create_hot_reload_state();
    // Merge the existing router with hot reload routes
    app.route("/ws", get(ws_handler_wrapper))
       .route("/api/reload", post(reload_notify_wrapper))
       .layer(axum::extract::Extension(state))
}

#[cfg(feature = "server")]
async fn ws_handler_wrapper(
    ws: WebSocketUpgrade,
    axum::extract::Extension(state): axum::extract::Extension<HotReloadState>
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

#[cfg(feature = "server")]
async fn reload_notify_wrapper(
    axum::extract::Extension(state): axum::extract::Extension<HotReloadState>
) -> Json<serde_json::Value> {
    let _ = state.send("reload".to_string());
    Json(json!({"status": "ok"}))
}

#[cfg(feature = "server")]
fn create_hot_reload_state() -> HotReloadState {
    let (tx, _) = broadcast::channel(10);
    Arc::new(tx)
}

#[cfg(feature = "server")]
async fn handle_socket(mut socket: WebSocket, tx: HotReloadState) {
    let mut rx = tx.subscribe();
    
    // Send connection confirmation
    if socket.send(Message::Text("connected".to_string())).await.is_err() {
        return;
    }
    
    // Listen for reload notifications and forward to browser
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

pub fn inject_hot_reload_script() -> &'static str {
    r#"
    // WebSocket hot reload (dev only)
    function setupHotReload() {
        if (window.location.hostname !== 'localhost' && window.location.hostname !== '127.0.0.1') {
            return;
        }
        
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        function connect() {
            const ws = new WebSocket(wsUrl);
            
            ws.onopen = () => {
                console.log('🔥 Hot reload WebSocket connected');
            };
            
            ws.onmessage = (event) => {
                if (event.data === 'reload') {
                    console.log('🔄 WASM files updated, reloading...');
                    loadWasm();
                }
            };
            
            ws.onclose = () => {
                console.log('🔌 Hot reload WebSocket disconnected, retrying...');
                setTimeout(connect, 2000);
            };
            
            ws.onerror = (error) => {
                console.log('❌ WebSocket error:', error);
            };
        }
        
        connect();
    }
    
    setupHotReload();
    "#
}
