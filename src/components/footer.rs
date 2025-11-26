use momenta::prelude::*;

// Reusable Footer component
#[component]
pub fn Footer() -> Node {
    rsx! {
        <footer class="footer">
            <p>"Powered by Rust + Momenta SSR + WASM"</p>
        </footer>
    }
}
