use crate::{WordleAI, knowledge::Knowledge};
use std::collections::{HashMap, HashSet};
use wordle_core::LetterResult;

/// AI #4: Entropy-Based Guesser (Optimal Information Gain)
///
/// This AI picks guesses that maximize the expected information gain (entropy),
/// i.e., guesses that most effectively split the remaining candidate set.
pub struct EntropyGuesser {
    /// All allowed guesses
    wordlist: Vec<[char; 5]>,
    /// Knowledge about the hidden word
    knowledge: Knowledge,
    /// Words that have been marked invalid (not in game's list)
    invalid_words: HashSet<[char; 5]>,
}

impl EntropyGuesser {
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
            .filter(|&&w| !self.invalid_words.contains(&w) && self.knowledge.matches(&w))
            .copied()
            .collect()
    }

    /// Compute expected information gain (entropy) for a guess
    fn guess_entropy(&self, guess: &[char; 5], candidates: &[[char; 5]]) -> f64 {
        let mut pattern_counts: HashMap<[LetterResult; 5], usize> = HashMap::new();

        for &candidate in candidates {
            let pattern = wordle_core::take_guess(&candidate, guess);
            *pattern_counts.entry(pattern).or_insert(0) += 1;
        }

        let total = candidates.len() as f64;
        pattern_counts
            .values()
            .map(|&count| {
                let p = count as f64 / total;
                -p * p.log2()
            })
            .sum()
    }
}

impl WordleAI for EntropyGuesser {
    fn make_guess(&mut self) -> Option<[char; 5]> {
        let candidates = self.get_candidates();

        if candidates.is_empty() {
            return None;
        }

        // When we've narrowed down to very few candidates, just guess one of them
        // When there's only 1-2 candidates left, all guesses have entropy ≈ 0,
        // so we might as well guess the actual answer
        if candidates.len() <= 2 {
            return Some(candidates[0]);
        }

        // Compute entropy for every possible guess and take max
        self.wordlist
            .iter()
            .filter(|&word| !self.invalid_words.contains(word))
            .map(|word| (word, self.guess_entropy(word, &candidates)))
            .max_by(|(_, entropy_a), (_, entropy_b)| entropy_a.partial_cmp(entropy_b).unwrap())
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
