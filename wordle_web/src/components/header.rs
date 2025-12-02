use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wordle_core::Language;

#[component]
pub fn Header(
    title: &'static str,
    language: Signal<Language>,
    on_language_change: impl Fn(Language) + 'static,
    show_nav: bool,
    nav_to: Option<&'static str>,
    nav_label: Option<&'static str>,
) -> impl IntoView {
    let navigate = use_navigate();

    let nav_content = move || {
        if show_nav {
            let nav_path = nav_to.unwrap_or("/");
            let label = nav_label.unwrap_or("Navigate");
            let nav = navigate.clone();
            view! {
                <button
                    class="button button--small button--secondary"
                    on:click=move |_| {
                        nav(nav_path, Default::default());
                    }
                >
                    {label}
                </button>
            }
            .into_any()
        } else {
            ().into_any()
        }
    };

    view! {
        <div class="header">
            <div class="header__title">{title}</div>
            <div class="header__controls">
                {nav_content}
                <select
                    class="language-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let new_lang = match value.as_str() {
                            "de" => Language::German,
                            _ => Language::English,
                        };
                        on_language_change(new_lang);
                    }
                    prop:value=move || match language.get() {
                        Language::English => "en",
                        Language::German => "de",
                    }
                >
                    <option value="en">"English"</option>
                    <option value="de">"German"</option>
                </select>
            </div>
        </div>
    }
}
