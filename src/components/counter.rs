use momenta::prelude::*;

// Shared RSX macro to avoid code duplication between client and server components
macro_rules! counter_rsx {
    ($count:expr, $inc_handler:expr, $dec_handler:expr) => {
        rsx! {
            <>
                <h1 class="counter-title">"Momenta SSR + WASM Counter"</h1>
                <div class="counter-display">
                    <span class="counter-label">"Count: "</span>
                    <span class="counter-value">{$count}</span>
                </div>
                <div class="counter-buttons">
                    <button 
                        class="btn btn-primary"
                        on_click={$inc_handler}
                    >
                        "Increment"
                    </button>
                    
                    <button 
                        class="btn btn-secondary"
                        on_click={$dec_handler}
                    >
                        "Decrement"
                    </button>
                </div>
            </>
        }
    };
}

// WASM-specific Counter component that uses reactive signals
#[cfg(target_arch = "wasm32")]
#[component]
pub fn Counter() -> Node {
    let initial_count = get_initial_count_from_dom().unwrap_or(0);
    web_sys::console::log_1(&format!("📊 WASM reading initial count from DOM: {}", initial_count).into());
    
    // Create reactive signal within component context
    let count = create_signal(initial_count);
    web_sys::console::log_1(&format!("🎯 Reactive counter created with initial count: {}", initial_count).into());
    
    // Use shared macro with reactive handlers
    counter_rsx!(
        count,
        move |_| {
            count.set(count.get() + 1);
            web_sys::console::log_1(&format!("🔺 Counter incremented to: {}", count.get()).into());
        },
        move |_| {
            count.set(count.get() - 1);
            web_sys::console::log_1(&format!("🔻 Counter decremented to: {}", count.get()).into());
        }
    )
}

// Server-side counter component that renders static HTML
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, PartialEq)]
pub struct CounterServerProps {
    pub initial_count: i32,
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn CounterServer(props: &CounterServerProps) -> Node {
    // Use shared macro with no-op handlers
    counter_rsx!(
        props.initial_count,
        |_| {},
        |_| {}
    )
}

// Helper function to read initial count from DOM (WASM only)
#[cfg(target_arch = "wasm32")]
pub fn get_initial_count_from_dom() -> Option<i32> {
    web_sys::window()?
        .document()?
        .query_selector(".counter-value")
        .ok()??
        .text_content()?
        .trim()
        .parse()
        .ok()
}
