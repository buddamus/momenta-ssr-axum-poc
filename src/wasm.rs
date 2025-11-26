use crate::components::Counter;
use wasm_bindgen::prelude::*;
use momenta::prelude::*;

// WASM entry point - Replace server-rendered content
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🚀 WASM starting...".into());
    
    // Only mount Counter component if we're on a page that has the counter element
    if let Some(_) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.query_selector("#counter-app").ok())
        .flatten()
    {
        // Mount Counter component that will automatically read initial_count from DOM
        render_root::<Counter>("#counter-app");
        web_sys::console::log_1(&"✅ WASM replacement complete! Reactive counter mounted.".into());
    } else {
        web_sys::console::log_1(&"✅ WASM loaded successfully! No counter on this page.".into());
    }
}
