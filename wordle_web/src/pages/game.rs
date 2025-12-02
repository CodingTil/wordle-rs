use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use wordle_core::{Language, LetterResult};

use crate::components::{Footer, Header, MessageBanner, MessageType, Tile};

const MAX_ATTEMPTS: usize = 6;

#[component]
pub fn Game() -> impl IntoView {
    // State
    let (language, set_language) = signal(Language::English);
    let (solution, set_solution) = signal(pick_random_word(Language::English));
    let (current_guess, set_current_guess) = signal(String::new());
    let (guesses, set_guesses) = signal(Vec::<([char; 5], [LetterResult; 5])>::new());
    let (message, set_message) = signal(None::<(String, MessageType)>);
    let (game_over, set_game_over) = signal(false);
    let (_won, set_won) = signal(false);

    // Submit guess
    let submit_guess = move || {
        let guess = current_guess.get();
        if guess.len() != 5 {
            set_message.set(Some((
                "Word must be 5 letters long!".to_string(),
                MessageType::Info,
            )));
            return;
        }

        let guess_chars: [char; 5] = guess
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("guess must be 5 chars"));

        // Check if word is in wordlist
        let wordlist = language.get().wordlist_array();
        if !wordlist.contains(&guess_chars) {
            set_message.set(Some((
                "Word not in word list!".to_string(),
                MessageType::Error,
            )));
            return;
        }

        // Calculate results
        let results = wordle_core::take_guess(&solution.get(), &guess_chars);

        // Check if won
        if results.iter().all(|&r| r == LetterResult::Correct) {
            set_guesses.update(|g| g.push((guess_chars, results)));
            set_game_over.set(true);
            set_won.set(true);
            set_message.set(Some((
                format!(
                    "Congratulations! You won in {} guesses!",
                    guesses.get().len() + 1
                ),
                MessageType::Success,
            )));
            set_current_guess.set(String::new());
            return;
        }

        // Check if lost
        if guesses.get().len() + 1 >= MAX_ATTEMPTS {
            set_guesses.update(|g| g.push((guess_chars, results)));
            set_game_over.set(true);
            let solution_str: String = solution.get().iter().collect();
            set_message.set(Some((
                format!("Game over! The word was: {}", solution_str),
                MessageType::Error,
            )));
            set_current_guess.set(String::new());
            return;
        }

        // Continue game
        set_guesses.update(|g| g.push((guess_chars, results)));
        set_current_guess.set(String::new());
        set_message.set(None);
    };

    // Change language
    let change_language = move |new_lang: Language| {
        set_language.set(new_lang);
        set_solution.set(pick_random_word(new_lang));
        set_current_guess.set(String::new());
        set_guesses.set(Vec::new());
        set_message.set(None);
        set_game_over.set(false);
        set_won.set(false);
    };

    // Reset
    let reset = move |_| {
        let current_lang = language.get();
        set_solution.set(pick_random_word(current_lang));
        set_current_guess.set(String::new());
        set_guesses.set(Vec::new());
        set_message.set(None);
        set_game_over.set(false);
        set_won.set(false);
    };

    // Handle key press
    let handle_key = move |key: String| {
        if game_over.get() {
            return;
        }

        if key == "Enter" {
            submit_guess();
        } else if key == "Backspace" {
            set_current_guess.update(|g| {
                g.pop();
            });
        } else if key.len() == 1
            && key.chars().next().unwrap().is_alphabetic()
            && current_guess.get().len() < 5
        {
            set_current_guess.update(|g| {
                g.push(key.to_lowercase().chars().next().unwrap());
            });
        }
    };

    // Handle input change (for mobile)
    let handle_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
        let value = input.value();

        if !game_over.get() {
            // Take only alphabetic characters, max 5
            let filtered: String = value
                .chars()
                .filter(|c| c.is_alphabetic())
                .take(5)
                .collect::<String>()
                .to_lowercase();

            set_current_guess.set(filtered);
        }
    };

    // Handle Enter key on input (for mobile)
    let handle_input_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !game_over.get() {
            submit_guess();
        }
    };

    view! {
        <div
            class="app"
            tabindex="0"
            on:keydown=move |ev| {
                handle_key(ev.key());
            }
        >
            <Header
                title="WORDLE"
                language=language.into()
                on_language_change=change_language
                show_nav=true
                nav_to=Some("/ai")
                nav_label=Some("AI Solver")
            />

            <MessageBanner message=message.into() />

            <div class="content">
                <div class="section">
                    <div class="section__title">"Guess the 5-letter word"</div>

                    {/* Mobile input field */}
                    <div class="mobile-input-container">
                        <input
                            type="text"
                            class="mobile-input"
                            placeholder="Type your guess..."
                            maxlength="5"
                            prop:value=move || current_guess.get()
                            on:input=handle_input
                            on:keydown=handle_input_keydown
                            disabled=move || game_over.get()
                        />
                    </div>

                    <div class="game-board">
                        {/* Previous guesses */}
                        {move || {
                            guesses
                                .get()
                                .into_iter()
                                .map(|(word, results)| {
                                    view! {
                                        <div class="word-row">
                                            {word
                                                .into_iter()
                                                .zip(results)
                                                .map(|(ch, result)| {
                                                    view! { <Tile letter=ch result=Some(result) interactive=false /> }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}

                        {/* Current guess row */}
                        {move || {
                            if !game_over.get() {
                                let guess = current_guess.get();
                                let chars: Vec<char> = guess.chars().collect();
                                view! {
                                    <div class="word-row">
                                        {(0..5)
                                            .map(|i| {
                                                let ch = chars.get(i).copied().unwrap_or(' ');
                                                view! { <Tile letter=ch result=None interactive=false /> }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                }
                                .into_any()
                            } else {
                                ().into_any()
                            }
                        }}

                        {/* Empty rows */}
                        {move || {
                            let remaining = if game_over.get() {
                                MAX_ATTEMPTS.saturating_sub(guesses.get().len())
                            } else {
                                MAX_ATTEMPTS.saturating_sub(guesses.get().len() + 1)
                            };

                            (0..remaining)
                                .map(|_| {
                                    view! {
                                        <div class="word-row">
                                            {(0..5)
                                                .map(|_| view! { <Tile letter=' ' result=None interactive=false /> })
                                                .collect::<Vec<_>>()}
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                </div>

                <div class="section">
                    <div class="instructions">
                        <p>"Type your guess and press Enter"</p>
                        <div class="instructions__hints">
                            <Tile letter='A' result=Some(LetterResult::Correct) small=true interactive=false />
                            <span>"Correct letter in correct position"</span>
                        </div>
                        <div class="instructions__hints">
                            <Tile letter='B' result=Some(LetterResult::Misplaced) small=true interactive=false />
                            <span>"Correct letter in wrong position"</span>
                        </div>
                        <div class="instructions__hints">
                            <Tile letter='C' result=Some(LetterResult::Absent) small=true interactive=false />
                            <span>"Letter not in word"</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="button-group">
                <button class="button button--red" on:click=reset>
                    "New Game"
                </button>
            </div>

            <Footer />
        </div>
    }
}

fn pick_random_word(language: Language) -> [char; 5] {
    let wordlist = language.wordlist_array();
    let mut bytes = [0u8; 4];
    getrandom::fill(&mut bytes).expect("Failed to get random bytes");
    let index = u32::from_le_bytes(bytes) as usize % wordlist.len();
    wordlist[index]
}
