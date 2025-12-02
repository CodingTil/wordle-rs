use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer">
            "© 2025 Til Mohr · "
            <a href="https://github.com/CodingTil/wordle-rs" target="_blank" rel="noopener noreferrer">
                "Source Code"
            </a>
        </div>
    }
}
