use wordle_core::LetterResult;

/// Trait for Wordle AI solvers
///
/// Implementations of this trait represent different strategies for solving Wordle puzzles.
/// The trait provides a common interface for making guesses and updating based on feedback.
pub trait WordleAI {
    /// Make the next guess
    ///
    /// Returns `Some([char; 5])` with the next guess, or `None` if no more guesses are available
    fn make_guess(&mut self) -> Option<[char; 5]>;

    /// Update the AI's internal state based on the result of the previous guess
    ///
    /// # Arguments
    /// * `guess` - The word that was guessed
    /// * `result` - The feedback for each letter (Correct, Misplaced, or Absent)
    fn update(&mut self, guess: [char; 5], result: [LetterResult; 5]);

    /// Mark a word as invalid (not in the word list for this particular game)
    ///
    /// This is used when a word is suggested but the game rejects it as not being in
    /// its word list. The AI should ensure it never suggests this word again.
    ///
    /// # Arguments
    /// * `word` - The word to mark as invalid
    fn mark_invalid(&mut self, word: [char; 5]);

    /// Reset the AI to its initial state for a new game
    fn reset(&mut self);
}

mod entropy_guesser;
mod heuristic_guesser;
mod knowledge;
mod random_guesser;
mod random_with_updates;

pub use entropy_guesser::EntropyGuesser;
pub use heuristic_guesser::HeuristicGuesser;
pub use random_guesser::RandomGuesser;
pub use random_with_updates::RandomWithUpdates;
