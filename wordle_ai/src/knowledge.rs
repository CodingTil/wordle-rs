use std::collections::{HashMap, HashSet};
use wordle_core::LetterResult;

/// Knowledge base for tracking what we know about the hidden word
#[derive(Clone, Debug)]
pub struct Knowledge {
    /// For each position (0-4), which letters are still possible
    pub possible_letters: [HashSet<char>; 5],
    /// Letters that must appear in the word at least # times
    pub must_contain: HashMap<char, u8>,
    /// Letters that are fixed in certain positions
    pub fixed_positions: [bool; 5],
}

impl Knowledge {
    pub fn new() -> Self {
        // Initially, all positions can have any letter
        let all_letters: HashSet<char> = ('a'..='z').collect();
        Self {
            possible_letters: [
                all_letters.clone(),
                all_letters.clone(),
                all_letters.clone(),
                all_letters.clone(),
                all_letters.clone(),
            ],
            must_contain: HashMap::new(),
            fixed_positions: [false; 5],
        }
    }

    /// Update knowledge based on a guess and its result
    pub fn update(&mut self, guess: [char; 5], result: [LetterResult; 5]) {
        // First: compute, per letter, how many Correct or Misplaced we have in this guess
        let mut positive_counts: HashMap<char, u8> = HashMap::new();
        for (&letter, &letter_result) in guess.iter().zip(result.iter()) {
            match letter_result {
                LetterResult::Correct | LetterResult::Misplaced => {
                    positive_counts
                        .entry(letter)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                LetterResult::Absent => { /* do nothing here */ }
            }
        }

        // Now handle positional constraints
        for (position, (&letter, &letter_result)) in guess.iter().zip(result.iter()).enumerate() {
            match letter_result {
                LetterResult::Correct => {
                    // This position must be this letter
                    self.possible_letters[position].clear();
                    self.possible_letters[position].insert(letter);
                    self.fixed_positions[position] = true;
                    // don't update must_contain here per position; we'll apply totals below
                }
                LetterResult::Misplaced => {
                    // Letter is in the word but not at this position
                    self.possible_letters[position].remove(&letter);
                    // we'll account for the count in must_contain after the loop
                }
                LetterResult::Absent => {
                    // If this letter had ZERO positive hits in this guess, it truly doesn't appear in the word.
                    // Remove it from all non-fixed positions in that case.
                    // If it *did* have positive hits, then this absent only means "not at this position"
                    // (the total occurrences are limited to positive_counts[letter]).
                    let pos_count = *positive_counts.get(&letter).unwrap_or(&0);
                    if pos_count == 0 {
                        for pos in 0..5 {
                            if !self.fixed_positions[pos] {
                                self.possible_letters[pos].remove(&letter);
                            }
                        }
                    } else {
                        // remove only from this position (we already counted the known occurrences)
                        self.possible_letters[position].remove(&letter);
                    }
                }
            }
        }

        // Finally, update must_contain to be the max of previously known and what we observed in this guess
        for (&letter, &count) in &positive_counts {
            self.must_contain
                .entry(letter)
                .and_modify(|prev| {
                    *prev = (*prev).max(count);
                })
                .or_insert(count);
        }
    }

    /// Check if a word matches our current knowledge
    pub fn matches(&self, word: &[char; 5]) -> bool {
        // Check that each position has a valid letter
        for (position, &letter) in word.iter().enumerate() {
            if !self.possible_letters[position].contains(&letter) {
                return false;
            }
        }

        // Check that all must-contain letters are present
        for (&letter, &count) in &self.must_contain {
            let actual_count = word.iter().filter(|&&c| c == letter).count() as u8;
            if actual_count < count {
                return false;
            }
        }

        true
    }
}
