use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod pages;
mod styles;

use pages::{AiSolver, Game, NotFound};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <style>{styles::STYLES}</style>
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Game />
                <Route path=path!("/ai") view=AiSolver />
            </Routes>
        </Router>
    }
}
