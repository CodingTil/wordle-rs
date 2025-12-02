use leptos::prelude::*;
use wordle_ai::{HeuristicGuesser, WordleAI};
use wordle_core::{Language, LetterResult};

mod styles;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    // State
    let (language, set_language) = signal(Language::English);
    let mut initial_ai = HeuristicGuesser::new(Language::English.wordlist_array().to_vec());
    let initial_recommendation = initial_ai.make_guess();
    let (ai, set_ai) = signal(initial_ai);
    let (recommendation, set_recommendation) = signal(initial_recommendation);
    let (feedback, set_feedback) = signal([None::<LetterResult>; 5]);
    let (history, set_history) = signal(Vec::<([char; 5], [LetterResult; 5])>::new());
    let (message, set_message) = signal(None::<(String, &'static str)>);
    let (won, set_won) = signal(false);

    // Toggle feedback for a position
    let toggle_feedback = move |pos: usize| {
        if recommendation.get().is_some() && !won.get() {
            set_feedback.update(|f| {
                f[pos] = match f[pos] {
                    None => Some(LetterResult::Absent),
                    Some(LetterResult::Absent) => Some(LetterResult::Misplaced),
                    Some(LetterResult::Misplaced) => Some(LetterResult::Correct),
                    Some(LetterResult::Correct) => Some(LetterResult::Absent),
                };
            });
        }
    };

    // Submit feedback
    let submit_feedback = move |_| {
        if feedback.get().iter().all(|f| f.is_some()) {
            if let Some(word) = recommendation.get() {
                let fb: [LetterResult; 5] = [
                    feedback.get()[0].unwrap(),
                    feedback.get()[1].unwrap(),
                    feedback.get()[2].unwrap(),
                    feedback.get()[3].unwrap(),
                    feedback.get()[4].unwrap(),
                ];

                // Check if won
                if fb.iter().all(|&f| f == LetterResult::Correct) {
                    set_history.update(|h| h.push((word, fb)));
                    set_won.set(true);
                    set_recommendation.set(None);
                    set_feedback.set([None; 5]);
                    set_message.set(Some(("Congratulations! You won!".to_string(), "success")));
                    return;
                }

                // Update AI
                let mut ai_val = ai.get_untracked();
                ai_val.update(word, fb);
                let next = ai_val.make_guess();
                set_ai.set(ai_val);
                set_history.update(|h| h.push((word, fb)));
                set_recommendation.set(next);
                set_feedback.set([None; 5]);
                set_message.set(if next.is_none() {
                    Some(("No more words available!".to_string(), "error"))
                } else {
                    None
                });
            }
        } else {
            set_message.set(Some((
                "Please set feedback for all letters!".to_string(),
                "info",
            )));
        }
    };

    // Mark word as invalid
    let mark_invalid = move |_| {
        if let Some(word) = recommendation.get() {
            let mut ai_val = ai.get_untracked();
            ai_val.mark_invalid(word);
            let next = ai_val.make_guess();
            set_ai.set(ai_val);
            set_recommendation.set(next);
            set_feedback.set([None; 5]);
            set_message.set(if next.is_none() {
                Some(("No more words available!".to_string(), "error"))
            } else {
                Some(("Word marked as invalid".to_string(), "info"))
            });
        }
    };

    // Change language
    let change_language = move |new_lang: Language| {
        let mut ai_val = HeuristicGuesser::new(new_lang.wordlist_array().to_vec());
        let next = ai_val.make_guess();
        set_language.set(new_lang);
        set_ai.set(ai_val);
        set_recommendation.set(next);
        set_feedback.set([None; 5]);
        set_history.set(Vec::new());
        set_message.set(None);
        set_won.set(false);
    };

    // Reset
    let reset = move |_| {
        let current_lang = language.get();
        let mut ai_val = HeuristicGuesser::new(current_lang.wordlist_array().to_vec());
        let next = ai_val.make_guess();
        set_ai.set(ai_val);
        set_recommendation.set(next);
        set_feedback.set([None; 5]);
        set_history.set(Vec::new());
        set_message.set(None);
        set_won.set(false);
    };

    view! {
        <style>{styles::STYLES}</style>

        <div class="app">
            {/* Header */}
            <div class="header">
                "WORDLE SOLVER"
                <select
                    class="language-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let new_lang = match value.as_str() {
                            "de" => Language::German,
                            _ => Language::English,
                        };
                        change_language(new_lang);
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

            {/* Message Banner */}
            {move || {
                message.get().map(|(msg, msg_type)| {
                    let class = match msg_type {
                        "success" => "message-banner message-banner--success",
                        "error" => "message-banner message-banner--error",
                        _ => "message-banner message-banner--info",
                    };
                    view! { <div class=class>{msg}</div> }
                })
            }}

            {/* Main Content */}
            <div class="content">
                {/* AI Recommendation */}
                <div class="section">
                    <div class="section__title">"AI Recommendation"</div>

                    {move || {
                        if let Some(word) = recommendation.get() {
                            let current_feedback = feedback.get();

                            view! {
                                <div class="word-row">
                                    {word
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, ch)| {
                                            let fb = current_feedback[i];
                                            let class = match fb {
                                                None => "tile tile--default",
                                                Some(LetterResult::Absent) => "tile tile--absent",
                                                Some(LetterResult::Misplaced) => "tile tile--misplaced",
                                                Some(LetterResult::Correct) => "tile tile--correct",
                                            };
                                            let ch_str = ch.to_string();

                                            view! {
                                                <div class=class on:click=move |_| toggle_feedback(i)>
                                                    {ch_str}
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="history__empty" style="padding: 2rem 0;">
                                    "No recommendations available"
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </div>

                {/* Guess History */}
                <div class="section">
                    <div class="section__title">"Guess History"</div>
                    <div class="history">
                        {move || {
                            let h = history.get();
                            if h.is_empty() {
                                view! { <div class="history__empty">"No guesses yet"</div> }.into_any()
                            } else {
                                h.into_iter()
                                    .map(|(word, results)| {
                                        view! {
                                            <div class="word-row">
                                                {word
                                                    .into_iter()
                                                    .zip(results)
                                                    .map(|(ch, result)| {
                                                        let class = match result {
                                                            LetterResult::Absent => "tile tile--small tile--absent tile--inactive",
                                                            LetterResult::Misplaced => "tile tile--small tile--misplaced tile--inactive",
                                                            LetterResult::Correct => "tile tile--small tile--correct tile--inactive",
                                                        };
                                                        let ch_str = ch.to_string();
                                                        view! { <div class=class>{ch_str}</div> }
                                                    })
                                                    .collect::<Vec<_>>()}
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            {/* Action Buttons */}
            <div class="button-group">
                {move || {
                    if recommendation.get().is_some() && !won.get() {
                        view! {
                            <>
                                <button class="button button--primary" on:click=submit_feedback>
                                    "Submit Feedback"
                                </button>
                                <button class="button button--yellow" on:click=mark_invalid>
                                    "Not in Word List"
                                </button>
                            </>
                        }
                        .into_any()
                    } else {
                        ().into_any()
                    }
                }}

                <button class="button button--red" on:click=reset>
                    "Reset"
                </button>
            </div>

            {/* Footer */}
            <div class="footer">
                "© 2025 Til Mohr · "
                <a href="https://github.com/CodingTil/wordle-rs" target="_blank" rel="noopener noreferrer">
                    "Source Code"
                </a>
            </div>
        </div>
    }
}
