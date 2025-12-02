use leptos::prelude::*;
use wordle_core::LetterResult;

#[component]
pub fn Tile(
    letter: char,
    result: Option<LetterResult>,
    #[prop(optional)] small: bool,
    #[prop(optional)] interactive: bool,
) -> impl IntoView {
    let mut classes = vec!["tile"];

    if small {
        classes.push("tile--small");
    }

    match result {
        None => classes.push("tile--default"),
        Some(LetterResult::Absent) => classes.push("tile--absent"),
        Some(LetterResult::Misplaced) => classes.push("tile--misplaced"),
        Some(LetterResult::Correct) => classes.push("tile--correct"),
    }

    if !interactive {
        classes.push("tile--inactive");
    }

    let class = classes.join(" ");
    let ch_str = letter.to_string();

    view! {
        <div class=class>
            {ch_str}
        </div>
    }
}

#[component]
pub fn InteractiveTile<F>(
    letter: char,
    result: Option<LetterResult>,
    #[prop(optional)] small: bool,
    on_click: F,
) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let mut classes = vec!["tile"];

    if small {
        classes.push("tile--small");
    }

    match result {
        None => classes.push("tile--default"),
        Some(LetterResult::Absent) => classes.push("tile--absent"),
        Some(LetterResult::Misplaced) => classes.push("tile--misplaced"),
        Some(LetterResult::Correct) => classes.push("tile--correct"),
    }

    let class = classes.join(" ");
    let ch_str = letter.to_string();

    view! {
        <div class=class on:click=move |_| on_click()>
            {ch_str}
        </div>
    }
}
