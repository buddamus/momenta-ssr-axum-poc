use momenta::prelude::*;

// Reusable Navbar component
#[component]
pub fn Navbar() -> Node {
    rsx! {
        <nav class="navbar">
            <div class="nav-brand">
                <h2>"Momenta SSR + WASM"</h2>
            </div>
            <div class="nav-links">
                <a href="/" class="nav-link">"Home"</a>
                <a href="/about" class="nav-link">"About"</a>
            </div>
        </nav>
    }
}
