#[cfg(feature = "server")]
extern crate alloc;

// Conditional import - only in debug builds
#[cfg(all(feature = "server", debug_assertions))]
mod hot_reload;

#[cfg(feature = "server")]
use axum::{
    extract::Query,
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};

#[cfg(feature = "server")]
use momenta::prelude::*;

#[cfg(feature = "server")]
use std::collections::HashMap;

#[cfg(feature = "server")]
use tower_http::services::ServeDir;

// Import components from our library crate
#[cfg(feature = "server")]
use rust_momenta::{CounterServer, Layout};

#[cfg(feature = "server")]
fn render_home_page(count: i32) -> String {
    // For server-side rendering, we'll render the components directly as HTML strings
    rsx!(
        <Layout>
            <div class="home-page">
                <h1>"Welcome to Momenta SSR + WASM!"</h1>
                <p>"This page is server-side rendered, then hydrated with WASM and reactive signals."</p>
                <div class="counter-container" id="counter-app">
                    <CounterServer initial_count={count} />
                </div>
                <div class="features">
                    <h3>"Features:"</h3>
                    <ul>
                        <li>"Server-side rendered HTML"</li>
                        <li>"WASM hydration with create_signal"</li>
                        <li>"Reactive state management"</li>
                        <li>"Shared Rust components"</li>
                    </ul>
                </div>
            </div>
        </Layout>
    ).to_string()
}

#[cfg(feature = "server")]
fn render_about_page() -> String {
    rsx!(
        <Layout>
            <div class="about-page">
                <h1>"About This App"</h1>
                <p>"This demonstrates true SSR + WASM hydration with Momenta."</p>
                <p>"The same Rust components render on both server and client!"</p>
                <div class="tech-stack">
                    <h3>"Architecture:"</h3>
                    <ul>
                        <li><strong>"Server:"</strong> " Rust + Axum + Momenta SSR"</li>
                        <li><strong>"Client:"</strong> " WASM + Momenta + create_signal"</li>
                        <li><strong>"Hydration:"</strong> " Shared components + reactive state"</li>
                    </ul>
                </div>
                <a href="/" class="btn btn-primary">"Back to Home"</a>
                
                <div style="margin-top: 2rem; padding: 1rem; background: #f8f9fa; border-radius: 8px;">
                    <h4>"Try this:"</h4>
                    <p>"1. Disable JavaScript in your browser - the page still works (SSR)"</p>
                    <p>"2. Re-enable JavaScript - the counter becomes interactive (WASM hydration)"</p>
                </div>
            </div>
        </Layout>
    ).to_string()
}

#[cfg(feature = "server")]
fn render_404_page() -> String {
    rsx!(
        <Layout>
            <div class="home-page">
                <h1>"404 - Page Not Found"</h1>
                <p>"The page you're looking for doesn't exist."</p>
                <a href="/" class="btn btn-primary">"Go Home"</a>
            </div>
        </Layout>
    ).to_string()
}

#[cfg(feature = "server")]
fn generate_html_document(body_content: &str, title: &str) -> String {
    let hot_reload_script = {
        #[cfg(debug_assertions)]
        {
            hot_reload::inject_hot_reload_script()
        }
        #[cfg(not(debug_assertions))]
        {
            ""
        }
    };
    
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        
        .navbar {{
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(10px);
            padding: 1rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            box-shadow: 0 2px 20px rgba(0,0,0,0.1);
        }}
        
        .nav-brand h2 {{
            color: #667eea;
            font-weight: 600;
        }}
        
        .nav-links {{
            display: flex;
            gap: 2rem;
        }}
        
        .nav-link {{
            text-decoration: none;
            color: #333;
            font-weight: 500;
            transition: color 0.3s ease;
        }}
        
        .nav-link:hover {{
            color: #667eea;
        }}
        
        .main-content {{
            max-width: 800px;
            margin: 2rem auto;
            padding: 0 2rem;
        }}
        
        .home-page, .about-page {{
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(10px);
            padding: 3rem;
            border-radius: 20px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
        }}
        
        .counter-container {{
            text-align: center;
            margin: 2rem 0;
            padding: 2rem;
            background: #f8f9fa;
            border-radius: 15px;
            border: 2px solid #e9ecef;
        }}
        
        .counter-title {{
            color: #495057;
            margin-bottom: 1.5rem;
            font-size: 1.8rem;
        }}
        
        .counter-display {{
            margin: 2rem 0;
            font-size: 1.5rem;
        }}
        
        .counter-label {{
            color: #6c757d;
            font-weight: 500;
        }}
        
        .counter-value {{
            color: #667eea;
            font-weight: bold;
            font-size: 2rem;
            margin-left: 0.5rem;
        }}
        
        .counter-buttons {{
            display: flex;
            gap: 1rem;
            justify-content: center;
            margin-top: 2rem;
        }}
        
        .btn {{
            padding: 12px 24px;
            border: none;
            border-radius: 8px;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            text-decoration: none;
            display: inline-block;
            text-align: center;
        }}
        
        .btn-primary {{
            background: #667eea;
            color: white;
        }}
        
        .btn-primary:hover {{
            background: #5a67d8;
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(102, 126, 234, 0.3);
        }}
        
        .btn-secondary {{
            background: #6c757d;
            color: white;
        }}
        
        .btn-secondary:hover {{
            background: #5a6268;
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(108, 117, 125, 0.3);
        }}
        
        .features, .tech-stack {{
            margin: 2rem 0;
        }}
        
        .features h3, .tech-stack h3 {{
            color: #495057;
            margin-bottom: 1rem;
        }}
        
        .features ul, .tech-stack ul {{
            list-style: none;
            padding-left: 0;
        }}
        
        .features li, .tech-stack li {{
            padding: 0.5rem 0;
            padding-left: 1.5rem;
            position: relative;
        }}
        
        .features li::before, .tech-stack li::before {{
            content: "✓";
            position: absolute;
            left: 0;
            color: #667eea;
            font-weight: bold;
        }}
        
        .footer {{
            text-align: center;
            padding: 2rem;
            color: rgba(255, 255, 255, 0.8);
            margin-top: 2rem;
        }}
        
        h1 {{
            color: #495057;
            margin-bottom: 1rem;
            font-size: 2.5rem;
        }}
        
        p {{
            margin-bottom: 1rem;
            color: #6c757d;
            font-size: 1.1rem;
        }}
    </style>
</head>
<body>
    {}
    <script type="module">
        import init from './pkg/rust_momenta.js';
        
        let wasmInitialized = false;
        
        async function loadWasm() {{
            try {{
                // Add cache-busting parameter to force reload
                const wasmUrl = './pkg/rust_momenta.js?t=' + Date.now();
                const {{ default: init }} = await import(wasmUrl);
                
                // Clear any existing WASM state
                if (wasmInitialized) {{
                    // Remove existing counter if it exists
                    const counterApp = document.getElementById('counter-app');
                    if (counterApp) {{
                        counterApp.innerHTML = counterApp.innerHTML; // Reset to original HTML
                    }}
                }}
                
                // Load and initialize the WASM module
                await init();
                wasmInitialized = true;
                console.log('🚀 WASM hydration complete! Momenta reactive signals are now active.');
            }} catch (error) {{
                console.warn('⚠️ WASM hydration failed, falling back to server-rendered content:', error);
                // Fallback: the page still works without WASM (pure SSR)
            }}
        }}
        
        // Start WASM hydration
        await loadWasm();
        
        {}
    </script>
</body>
</html>"#,
        title, body_content, hot_reload_script
    )
}

#[cfg(feature = "server")]
async fn home_handler(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let count = params
        .get("count")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    let body_content = render_home_page(count);
    let html = generate_html_document(&body_content, "Home - Momenta SSR + WASM");
    Html(html)
}

#[cfg(feature = "server")]
async fn about_handler() -> Html<String> {
    let body_content = render_about_page();
    let html = generate_html_document(&body_content, "About - Momenta SSR");
    Html(html)
}

#[cfg(feature = "server")]
async fn not_found() -> (StatusCode, Html<String>) {
    let body_content = render_404_page();
    let html = generate_html_document(&body_content, "404 - Page Not Found");
    (StatusCode::NOT_FOUND, Html(html))
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // Create base router
    let mut app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/pkg", ServeDir::new("pkg"))  // Serve WASM files
        .fallback(not_found);

    // Conditionally add hot reload in debug builds
    #[cfg(debug_assertions)]
    {
        app = hot_reload::setup_hot_reload(app);
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    println!("📄 Try these routes:");
    println!("   • http://localhost:3000/ (home with counter)");
    println!("   • http://localhost:3000/?count=10 (with initial count)");
    println!("   • http://localhost:3000/about (about page)");
    println!("📦 WASM files served from:");
    println!("   • http://localhost:3000/pkg/rust_momenta.js");
    println!("   • http://localhost:3000/pkg/rust_momenta_bg.wasm");
    #[cfg(debug_assertions)]
    println!("🔥 Hot reload WebSocket available at /ws");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("🚫 Server feature not enabled.");
    println!("📖 To run the server, use: cargo run --features server");
    println!("🔧 To build WASM, use: wasm-pack build --target web");
}
