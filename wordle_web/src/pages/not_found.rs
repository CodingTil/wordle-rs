use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::Footer;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="app">
            <div class="not-found">
                <div class="not-found__content">
                    <h1 class="not-found__title">"404"</h1>
                    <h2 class="not-found__subtitle">"Page Not Found"</h2>
                    <p class="not-found__message">
                        "The page you're looking for doesn't exist."
                    </p>

                    <div class="not-found__actions">
                        <button
                            class="button button--primary"
                            on:click={
                                let navigate = use_navigate();
                                move |_| {
                                    navigate("/", Default::default());
                                }
                            }
                        >
                            "Play Wordle"
                        </button>
                        <button
                            class="button button--secondary"
                            on:click={
                                let navigate = use_navigate();
                                move |_| {
                                    navigate("/ai", Default::default());
                                }
                            }
                        >
                            "AI Solver"
                        </button>
                    </div>
                </div>
            </div>

            <Footer />
        </div>
    }
}
