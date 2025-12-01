use crate::WordleAI;
use rand::SeedableRng;
use rand::prelude::*;
use std::collections::HashSet;
use wordle_core::LetterResult;

/// AI #1: Random Guesser
///
/// This strategy randomly guesses words from the candidate list without replacement,
/// ignoring any information given from past guesses.
pub struct RandomGuesser {
    /// All available words
    wordlist: Vec<[char; 5]>,
    /// Indices of words that haven't been guessed yet
    available_indices: Vec<usize>,
    /// Words that have been marked as invalid (not in the game's word list)
    invalid_words: HashSet<[char; 5]>,
    /// Random number generator
    rng: StdRng,
}

impl RandomGuesser {
    /// Create a new RandomGuesser with the given word list
    pub fn new(wordlist: Vec<[char; 5]>) -> Self {
        let available_indices = (0..wordlist.len()).collect();
        Self {
            wordlist,
            available_indices,
            invalid_words: HashSet::new(),
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    /// Create a new RandomGuesser with a specific seed (useful for testing)
    pub fn with_seed(wordlist: Vec<[char; 5]>, seed: u64) -> Self {
        let available_indices = (0..wordlist.len()).collect();
        Self {
            wordlist,
            available_indices,
            invalid_words: HashSet::new(),
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl WordleAI for RandomGuesser {
    fn make_guess(&mut self) -> Option<[char; 5]> {
        // Keep trying to find a valid word that's not marked as invalid
        while !self.available_indices.is_empty() {
            let idx = self.rng.random_range(0..self.available_indices.len());
            let word_idx = self.available_indices.swap_remove(idx);
            let word = self.wordlist[word_idx];

            // If this word is not invalid, return it
            if !self.invalid_words.contains(&word) {
                return Some(word);
            }
            // Otherwise, continue to the next word
        }

        // No valid words left
        None
    }

    fn update(&mut self, _guess: [char; 5], _result: [LetterResult; 5]) {
        // Random guesser ignores feedback
    }

    fn mark_invalid(&mut self, word: [char; 5]) {
        self.invalid_words.insert(word);
    }

    fn reset(&mut self) {
        self.available_indices = (0..self.wordlist.len()).collect();
        self.invalid_words.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_guesser_makes_guesses() {
        let wordlist = vec![
            ['h', 'e', 'l', 'l', 'o'],
            ['w', 'o', 'r', 'l', 'd'],
            ['t', 'e', 's', 't', 's'],
        ];
        let mut guesser = RandomGuesser::with_seed(wordlist.clone(), 42);

        // Should be able to make guesses
        let guess1 = guesser.make_guess();
        assert!(guess1.is_some());

        let guess2 = guesser.make_guess();
        assert!(guess2.is_some());

        let guess3 = guesser.make_guess();
        assert!(guess3.is_some());

        // After 3 guesses, no more words available
        let guess4 = guesser.make_guess();
        assert!(guess4.is_none());
    }

    #[test]
    fn test_random_guesser_no_duplicates() {
        let wordlist = vec![
            ['h', 'e', 'l', 'l', 'o'],
            ['w', 'o', 'r', 'l', 'd'],
            ['t', 'e', 's', 't', 's'],
        ];
        let mut guesser = RandomGuesser::with_seed(wordlist.clone(), 42);

        let guess1 = guesser.make_guess().unwrap();
        let guess2 = guesser.make_guess().unwrap();
        let guess3 = guesser.make_guess().unwrap();

        // All guesses should be different
        assert_ne!(guess1, guess2);
        assert_ne!(guess2, guess3);
        assert_ne!(guess1, guess3);

        // All guesses should be from the wordlist
        assert!(wordlist.contains(&guess1));
        assert!(wordlist.contains(&guess2));
        assert!(wordlist.contains(&guess3));
    }

    #[test]
    fn test_random_guesser_reset() {
        let wordlist = vec![['h', 'e', 'l', 'l', 'o'], ['w', 'o', 'r', 'l', 'd']];
        let mut guesser = RandomGuesser::with_seed(wordlist.clone(), 42);

        // Make some guesses
        guesser.make_guess();
        guesser.make_guess();

        // After using all words, no more guesses
        assert!(guesser.make_guess().is_none());

        // Reset should allow guessing again
        guesser.reset();
        assert!(guesser.make_guess().is_some());
    }

    #[test]
    fn test_random_guesser_ignores_update() {
        let wordlist = vec![
            ['h', 'e', 'l', 'l', 'o'],
            ['w', 'o', 'r', 'l', 'd'],
            ['t', 'e', 's', 't', 's'],
        ];
        let mut guesser = RandomGuesser::with_seed(wordlist.clone(), 42);

        let guess1 = guesser.make_guess().unwrap();

        // Update with feedback (which RandomGuesser should ignore)
        guesser.update(guess1, [LetterResult::Absent; 5]);

        // Should still be able to make more guesses
        let guess2 = guesser.make_guess();
        assert!(guess2.is_some());
    }
}
