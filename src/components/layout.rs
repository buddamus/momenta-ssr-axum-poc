use momenta::prelude::*;
use super::navbar::Navbar;
use super::footer::Footer;

// Layout component that wraps page content with Navbar, main wrapper, and Footer
pub struct LayoutProps {
    pub children: Vec<Node>,
}

#[component]
pub fn Layout(props: &LayoutProps) -> Node {
    rsx! {
        <>
            <Navbar />
            <main class="main-content">
                {&props.children}
            </main>
            <Footer />
        </>
    }
}
