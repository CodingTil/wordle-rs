use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MessageType {
    Success,
    Error,
    Info,
}

#[component]
pub fn MessageBanner(message: Signal<Option<(String, MessageType)>>) -> impl IntoView {
    view! {
        {move || {
            message.get().map(|(msg, msg_type)| {
                let class = match msg_type {
                    MessageType::Success => "message-banner message-banner--success",
                    MessageType::Error => "message-banner message-banner--error",
                    MessageType::Info => "message-banner message-banner--info",
                };
                view! { <div class=class>{msg}</div> }
            })
        }}
    }
}
