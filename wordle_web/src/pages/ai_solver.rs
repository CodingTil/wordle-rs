use leptos::prelude::*;
use wordle_ai::{HeuristicGuesser, WordleAI};
use wordle_core::{Language, LetterResult};

use crate::components::{Footer, Header, InteractiveTile, MessageBanner, MessageType, Tile};

#[component]
pub fn AiSolver() -> impl IntoView {
    // State
    let (language, set_language) = signal(Language::English);
    let mut initial_ai = HeuristicGuesser::new(Language::English.wordlist_array().to_vec());
    let initial_recommendation = initial_ai.make_guess();
    let (ai, set_ai) = signal(initial_ai);
    let (recommendation, set_recommendation) = signal(initial_recommendation);
    let (feedback, set_feedback) = signal([None::<LetterResult>; 5]);
    let (history, set_history) = signal(Vec::<([char; 5], [LetterResult; 5])>::new());
    let (message, set_message) = signal(None::<(String, MessageType)>);
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
                    set_message.set(Some((
                        "Congratulations! You won!".to_string(),
                        MessageType::Success,
                    )));
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
                    Some(("No more words available!".to_string(), MessageType::Error))
                } else {
                    None
                });
            }
        } else {
            set_message.set(Some((
                "Please set feedback for all letters!".to_string(),
                MessageType::Info,
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
                Some(("No more words available!".to_string(), MessageType::Error))
            } else {
                Some(("Word marked as invalid".to_string(), MessageType::Info))
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
        <div class="app">
            <Header
                title="WORDLE SOLVER"
                language=language.into()
                on_language_change=change_language
                show_nav=true
                nav_to=Some("/")
                nav_label=Some("Play Game")
            />

            <MessageBanner message=message.into() />

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
                                            view! {
                                                <InteractiveTile
                                                    letter=ch
                                                    result=fb
                                                    on_click=move || toggle_feedback(i)
                                                />
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
                                                        view! {
                                                            <Tile
                                                                letter=ch
                                                                result=Some(result)
                                                                small=true
                                                                interactive=false
                                                            />
                                                        }
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

            <Footer />
        </div>
    }
}
