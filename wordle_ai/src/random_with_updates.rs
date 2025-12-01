use crate::{WordleAI, knowledge::Knowledge};
use rand::SeedableRng;
use rand::prelude::*;
use std::collections::HashSet;
use wordle_core::LetterResult;

/// AI #2: Random Guesser with Updates
///
/// This strategy randomly guesses from a candidate set that is updated based on
/// feedback from previous guesses. It uses a Knowledge base to track constraints
/// and filter out words that don't match what we've learned.
pub struct RandomWithUpdates {
    /// All available words
    wordlist: Vec<[char; 5]>,
    /// Knowledge about the hidden word
    knowledge: Knowledge,
    /// Words that have been marked as invalid (not in the game's word list)
    invalid_words: HashSet<[char; 5]>,
    /// Random number generator
    rng: StdRng,
}

impl RandomWithUpdates {
    /// Create a new RandomWithUpdates with the given word list
    pub fn new(wordlist: Vec<[char; 5]>) -> Self {
        Self {
            wordlist,
            knowledge: Knowledge::new(),
            invalid_words: HashSet::new(),
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    /// Create a new RandomWithUpdates with a specific seed (useful for testing)
    pub fn with_seed(wordlist: Vec<[char; 5]>, seed: u64) -> Self {
        Self {
            wordlist,
            knowledge: Knowledge::new(),
            invalid_words: HashSet::new(),
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Get all candidate words that match current knowledge
    fn get_candidates(&self) -> Vec<[char; 5]> {
        self.wordlist
            .iter()
            .filter(|&&word| !self.invalid_words.contains(&word) && self.knowledge.matches(&word))
            .copied()
            .collect()
    }
}

impl WordleAI for RandomWithUpdates {
    fn make_guess(&mut self) -> Option<[char; 5]> {
        let candidates = self.get_candidates();

        if candidates.is_empty() {
            return None;
        }

        // Pick a random candidate
        let idx = self.rng.random_range(0..candidates.len());
        Some(candidates[idx])
    }

    fn update(&mut self, guess: [char; 5], result: [LetterResult; 5]) {
        self.knowledge.update(guess, result);
    }

    fn mark_invalid(&mut self, word: [char; 5]) {
        self.invalid_words.insert(word);
    }

    fn reset(&mut self) {
        self.knowledge = Knowledge::new();
        self.invalid_words.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_correct_letter() {
        let mut knowledge = Knowledge::new();
        let guess = ['a', 'b', 'c', 'd', 'e'];
        let result = [
            LetterResult::Correct,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
        ];
        knowledge.update(guess, result);

        // Position 0 should only allow 'a'
        assert_eq!(knowledge.possible_letters[0].len(), 1);
        assert!(knowledge.possible_letters[0].contains(&'a'));

        // 'a' must be in the word
        assert!(knowledge.must_contain.get(&'a') == Some(&1));
    }

    #[test]
    fn test_knowledge_misplaced_letter() {
        let mut knowledge = Knowledge::new();
        let guess = ['a', 'b', 'c', 'd', 'e'];
        let result = [
            LetterResult::Misplaced,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
        ];
        knowledge.update(guess, result);

        // Position 0 should not allow 'a'
        assert!(!knowledge.possible_letters[0].contains(&'a'));

        // But 'a' must be in the word somewhere
        assert!(knowledge.must_contain.get(&'a') == Some(&1));

        // Other positions should still allow 'a'
        assert!(knowledge.possible_letters[1].contains(&'a'));
    }

    #[test]
    fn test_knowledge_absent_letter() {
        let mut knowledge = Knowledge::new();
        let guess = ['z', 'b', 'c', 'd', 'e'];
        let result = [
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
        ];
        knowledge.update(guess, result);

        // 'z' should be removed from all positions
        for pos in 0..5 {
            assert!(!knowledge.possible_letters[pos].contains(&'z'));
        }
    }

    #[test]
    fn test_knowledge_matches() {
        let mut knowledge = Knowledge::new();

        // Set up: position 0 must be 'a', and word must contain 'e'
        knowledge.possible_letters[0].clear();
        knowledge.possible_letters[0].insert('a');
        knowledge.must_contain.insert('e', 1);

        // Word starting with 'a' and containing 'e' should match
        assert!(knowledge.matches(&['a', 'p', 'p', 'l', 'e']));

        // Word not starting with 'a' should not match
        assert!(!knowledge.matches(&['b', 'p', 'p', 'l', 'e']));

        // Word starting with 'a' but not containing 'e' should not match
        assert!(!knowledge.matches(&['a', 'b', 'o', 'u', 't']));
    }

    #[test]
    fn test_random_with_updates_filters_candidates() {
        let wordlist = vec![
            ['a', 'p', 'p', 'l', 'e'],
            ['a', 'b', 'o', 'u', 't'],
            ['h', 'e', 'l', 'l', 'o'],
        ];
        let mut ai = RandomWithUpdates::with_seed(wordlist, 42);

        // Simulate guess with first letter 'a' being correct, rest absent
        // This means: position 0 must be 'a', and 'p', 'l', 'e' are not in the word
        let guess = ['a', 'p', 'p', 'l', 'e'];
        let result = [
            LetterResult::Correct,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
        ];
        ai.update(guess, result);

        // Get candidates - should only include words starting with 'a' and not containing 'p', 'l', 'e'
        let candidates = ai.get_candidates();
        assert_eq!(candidates.len(), 1); // Only 'about' matches
        assert!(candidates.contains(&['a', 'b', 'o', 'u', 't']));
        assert!(!candidates.contains(&['a', 'p', 'p', 'l', 'e'])); // contains 'p', 'l', 'e'
        assert!(!candidates.contains(&['h', 'e', 'l', 'l', 'o'])); // doesn't start with 'a'
    }

    #[test]
    fn test_random_with_updates_mark_invalid() {
        let wordlist = vec![['a', 'p', 'p', 'l', 'e'], ['a', 'b', 'o', 'u', 't']];
        let mut ai = RandomWithUpdates::with_seed(wordlist, 42);

        // Mark 'apple' as invalid
        ai.mark_invalid(['a', 'p', 'p', 'l', 'e']);

        // Get candidates - should not include 'apple'
        let candidates = ai.get_candidates();
        assert_eq!(candidates.len(), 1);
        assert!(candidates.contains(&['a', 'b', 'o', 'u', 't']));
    }

    #[test]
    fn test_random_with_updates_reset() {
        let wordlist = vec![['a', 'p', 'p', 'l', 'e'], ['h', 'e', 'l', 'l', 'o']];
        let mut ai = RandomWithUpdates::with_seed(wordlist.clone(), 42);

        // Update knowledge
        ai.update(['a', 'p', 'p', 'l', 'e'], [LetterResult::Correct; 5]);

        // Mark word as invalid
        ai.mark_invalid(['h', 'e', 'l', 'l', 'o']);

        // Reset
        ai.reset();

        // After reset, all words should be candidates again
        let candidates = ai.get_candidates();
        assert_eq!(candidates.len(), 2);
    }
}
