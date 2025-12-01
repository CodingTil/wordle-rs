use clap::ValueEnum;
use wordle_ai::{EntropyGuesser, HeuristicGuesser, RandomGuesser, RandomWithUpdates, WordleAI};
use wordle_proc::include_wordlist;

pub const WORD_LENGTH: usize = 5;
pub const WORDLIST_ARRAY: &[[char; 5]] = &include_wordlist!("wordlist.txt");

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Hash)]
pub enum AIType {
    /// AI #1: Random Guesser - randomly guesses without using feedback
    Random,
    /// AI #2: Random Guesser with Updates - uses feedback to filter candidate words
    RandomUpdates,
    /// AI #3: Heuristic Guesser - scores words based on letter frequency
    Heuristic,
    /// AI #4: Entropy Guesser - maximizes expected information gain
    Entropy,
}

impl AIType {
    /// Get the display name for this AI type
    pub fn name(&self) -> &'static str {
        match self {
            AIType::Random => "Random Guesser",
            AIType::RandomUpdates => "Random with Updates",
            AIType::Heuristic => "Heuristic Guesser",
            AIType::Entropy => "Entropy Guesser",
        }
    }
}

/// Factory function to create the appropriate AI based on type
pub fn create_ai(ai_type: AIType, wordlist: Vec<[char; 5]>) -> Box<dyn WordleAI> {
    match ai_type {
        AIType::Random => Box::new(RandomGuesser::new(wordlist)),
        AIType::RandomUpdates => Box::new(RandomWithUpdates::new(wordlist)),
        AIType::Heuristic => Box::new(HeuristicGuesser::new(wordlist)),
        AIType::Entropy => Box::new(EntropyGuesser::new(wordlist)),
    }
}
