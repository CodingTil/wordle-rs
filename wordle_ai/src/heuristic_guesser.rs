use crate::{WordleAI, knowledge::Knowledge};
use std::collections::{HashMap, HashSet};
use wordle_core::LetterResult;

/// AI #3: Heuristic Guesser
///
/// This strategy scores words based on letter frequency to maximize information gain.
/// It calculates the frequency of each letter across all candidate words, then scores
/// each word by summing S(p) = -(p² + (1-p)²) for each unique letter, where p is the
/// letter's frequency. This formula is maximized when p = 0.5, encouraging guesses
/// with letters that appear in about half the candidates.
#[derive(Clone)]
pub struct HeuristicGuesser {
    /// All available words
    wordlist: Vec<[char; 5]>,
    /// Knowledge about the hidden word
    knowledge: Knowledge,
    /// Words that have been marked as invalid (not in the game's word list)
    invalid_words: HashSet<[char; 5]>,
}

fn entropy(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return 0.0;
    }
    let q = 1.0 - p;
    -(p * p.log2() + q * q.log2())
}

impl HeuristicGuesser {
    /// Create a new HeuristicGuesser with the given word list
    pub fn new(wordlist: Vec<[char; 5]>) -> Self {
        Self {
            wordlist,
            knowledge: Knowledge::new(),
            invalid_words: HashSet::new(),
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

    /// Calculate letter frequencies across all candidate words
    fn calculate_letter_frequencies(&self, candidates: &[[char; 5]]) -> HashMap<char, f64> {
        let mut letter_counts: HashMap<char, usize> = HashMap::new();
        let total_words = candidates.len();

        // Count how many words contain each letter (only count once per word)
        for word in candidates {
            let unique_letters: HashSet<char> = word.iter().copied().collect();
            for letter in unique_letters {
                *letter_counts.entry(letter).or_insert(0) += 1;
            }
        }

        // Convert counts to frequencies
        letter_counts
            .into_iter()
            .map(|(letter, count)| (letter, count as f64 / total_words as f64))
            .map(|(letter, frequency)| {
                assert!(
                    0. < frequency && frequency <= 1.,
                    "{}: {}",
                    letter,
                    frequency
                );
                (letter, frequency)
            })
            .collect()
    }

    /// Score a word based on letter frequencies
    /// S(p) = -(p² + (1-p)²) for each unique letter
    fn score_word(&self, word: &[char; 5], frequencies: &HashMap<char, f64>) -> f64 {
        // Deduplicate letters - we only get information from each letter once
        let unique_letters: HashSet<char> = word.iter().copied().collect();

        unique_letters
            .iter()
            .map(|letter| {
                let p = frequencies.get(letter).copied().unwrap_or(0.0);
                entropy(p)
            })
            .sum()
    }
}

impl WordleAI for HeuristicGuesser {
    fn make_guess(&mut self) -> Option<[char; 5]> {
        let candidates = self.get_candidates();

        if candidates.is_empty() {
            return None;
        }

        // Calculate letter frequencies
        let frequencies = self.calculate_letter_frequencies(&candidates);

        // Find the word with the highest score
        candidates
            .iter()
            .map(|word| (word, self.score_word(word, &frequencies)))
            .max_by(|(_, score_a), (_, score_b)| score_a.partial_cmp(score_b).unwrap())
            .map(|(word, _)| *word)
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
    fn test_calculate_letter_frequencies() {
        let wordlist = vec![
            ['a', 'p', 'p', 'l', 'e'],
            ['a', 'b', 'o', 'u', 't'],
            ['h', 'e', 'l', 'l', 'o'],
        ];
        let ai = HeuristicGuesser::new(wordlist.clone());
        let frequencies = ai.calculate_letter_frequencies(&wordlist);

        // 'a' appears in 2/3 words
        assert!((frequencies[&'a'] - 2.0 / 3.0).abs() < 0.01);
        // 'e' appears in 2/3 words
        assert!((frequencies[&'e'] - 2.0 / 3.0).abs() < 0.01);
        // 'l' appears in 2/3 words
        assert!((frequencies[&'l'] - 2.0 / 3.0).abs() < 0.01);
        // 'p' appears in 1/3 words
        assert!((frequencies[&'p'] - 1.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_score_word() {
        let wordlist = vec![['a', 'b', 'c', 'd', 'e']];
        let ai = HeuristicGuesser::new(wordlist);

        let mut frequencies = HashMap::new();
        frequencies.insert('a', 0.5);
        frequencies.insert('b', 0.5);
        frequencies.insert('c', 0.5);
        frequencies.insert('d', 1.0);
        frequencies.insert('e', 1.0);

        let word = ['a', 'b', 'c', 'd', 'e'];
        let score = ai.score_word(&word, &frequencies);

        // For p=0.5: entropy(0.5) = -(0.5*log2(0.5) + 0.5*log2(0.5)) = 1.0
        // For p=1.0: entropy(1.0) = 0.0 (no information gain)
        // Word has 5 unique letters: 3 with p=0.5, 2 with p=1.0
        // Score should be 3 * 1.0 + 2 * 0.0 = 3.0
        assert!((score - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_score_word_deduplicates() {
        let wordlist = vec![['a', 'p', 'p', 'l', 'e']];
        let ai = HeuristicGuesser::new(wordlist);

        let mut frequencies = HashMap::new();
        frequencies.insert('a', 0.5);
        frequencies.insert('p', 0.5);
        frequencies.insert('l', 0.5);
        frequencies.insert('e', 0.5);

        let word = ['a', 'p', 'p', 'l', 'e'];
        let score = ai.score_word(&word, &frequencies);

        // Word has 4 unique letters (a, p, l, e), each with p=0.5
        // Score should be 4 * 1.0 = 4.0
        // NOT 5 * 1.0 = 5.0 (which would happen if we didn't deduplicate)
        assert!((score - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_heuristic_guesser_picks_best_word() {
        // Create a simple wordlist where we can predict the best word
        // With only 2 candidates, any letter in both has p=1.0 (score: -1.0)
        // Any letter in only one has p=0.5 (score: -0.5) - this is optimal!
        let wordlist = vec![
            ['a', 'a', 'a', 'b', 'c'], // Has 'a' (appears in both), 'b', 'c'
            ['a', 'a', 'a', 'x', 'y'], // Has 'a' (appears in both), 'x', 'y'
        ];
        let mut ai = HeuristicGuesser::new(wordlist.clone());

        let guess = ai.make_guess();

        // Both words contain 'a' with p=1.0 (bad score: -1)
        // First word has b,c with p=0.5 each (good score: -0.5 each)
        // Second word has x,y with p=0.5 each (good score: -0.5 each)
        // Both should score equally: -1 + 2*(-0.5) = -2.0
        // So either word is acceptable
        assert!(guess.is_some());
        let result = guess.unwrap();
        assert!(wordlist.contains(&result));
    }

    #[test]
    fn test_heuristic_guesser_filters_candidates() {
        let wordlist = vec![
            ['a', 'p', 'p', 'l', 'e'],
            ['a', 'b', 'o', 'u', 't'],
            ['h', 'e', 'l', 'l', 'o'],
        ];
        let mut ai = HeuristicGuesser::new(wordlist);

        // Simulate guess with first letter 'a' being correct
        let guess = ['a', 'p', 'p', 'l', 'e'];
        let result = [
            LetterResult::Correct,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
            LetterResult::Absent,
        ];
        ai.update(guess, result);

        // Get candidates - should only include words starting with 'a'
        let candidates = ai.get_candidates();
        assert_eq!(candidates.len(), 1);
        assert!(candidates.contains(&['a', 'b', 'o', 'u', 't']));
    }

    #[test]
    fn test_heuristic_guesser_mark_invalid() {
        let wordlist = vec![['a', 'p', 'p', 'l', 'e'], ['a', 'b', 'o', 'u', 't']];
        let mut ai = HeuristicGuesser::new(wordlist);

        // Mark 'apple' as invalid
        ai.mark_invalid(['a', 'p', 'p', 'l', 'e']);

        // Get candidates - should not include 'apple'
        let candidates = ai.get_candidates();
        assert_eq!(candidates.len(), 1);
        assert!(candidates.contains(&['a', 'b', 'o', 'u', 't']));
    }

    #[test]
    fn test_heuristic_guesser_reset() {
        let wordlist = vec![['a', 'p', 'p', 'l', 'e'], ['h', 'e', 'l', 'l', 'o']];
        let mut ai = HeuristicGuesser::new(wordlist.clone());

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
