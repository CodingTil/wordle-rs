use rand::prelude::*;
use std::collections::HashSet;
use std::sync::LazyLock;

use wordle_proc::include_wordlist;

const WORDLIST_EN_ARRAY: &[[char; 5]] = &include_wordlist!("wordlist-en.txt");
const WORDLIST_DE_ARRAY: &[[char; 5]] = &include_wordlist!("wordlist-de.txt");

static WORDLIST_EN: LazyLock<HashSet<[char; 5]>> =
    LazyLock::new(|| WORDLIST_EN_ARRAY.iter().copied().collect());
static WORDLIST_DE: LazyLock<HashSet<[char; 5]>> =
    LazyLock::new(|| WORDLIST_DE_ARRAY.iter().copied().collect());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    English,
    German,
}

impl Language {
    pub fn wordlist_array(&self) -> &'static [[char; 5]] {
        match self {
            Language::English => WORDLIST_EN_ARRAY,
            Language::German => WORDLIST_DE_ARRAY,
        }
    }

    fn wordlist_set(&self) -> &'static HashSet<[char; 5]> {
        match self {
            Language::English => &WORDLIST_EN,
            Language::German => &WORDLIST_DE,
        }
    }
}

#[derive(Debug)]
pub enum WordListError {
    WordListEmpty,
}

#[derive(Debug)]
pub enum GameError {
    WordNotInList,
}

pub enum GuessResult {
    Continue([LetterResult; 5]),
    Won([LetterResult; 5]),
    Lost {
        last_guess: [LetterResult; 5],
        solution: [char; 5],
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LetterResult {
    Correct,
    Misplaced,
    Absent,
}

pub fn take_guess(solution: &[char; 5], guess: &[char; 5]) -> [LetterResult; 5] {
    let mut result = [LetterResult::Absent; 5];
    let mut solution_used = [false; 5];

    // First pass: mark correct positions
    for (i, &guess_char) in guess.iter().enumerate() {
        if guess_char == solution[i] {
            result[i] = LetterResult::Correct;
            solution_used[i] = true;
        }
    }

    // Second pass: mark misplaced letters
    for (i, &guess_char) in guess.iter().enumerate() {
        if result[i] == LetterResult::Correct {
            continue;
        }

        for (j, &sol_char) in solution.iter().enumerate() {
            if !solution_used[j] && sol_char == guess_char {
                result[i] = LetterResult::Misplaced;
                solution_used[j] = true;
                break;
            }
        }
    }

    result
}

#[derive(Clone)]
pub struct Game {
    solution: [char; 5],
    max_attempts: usize,
    attempts: usize,
    language: Language,
}

impl Game {
    pub fn new(max_attempts: usize, language: Language) -> Result<Game, WordListError> {
        let mut rng = rand::rng();
        match language.wordlist_array().choose(&mut rng) {
            Some(&word) => Ok(Game {
                solution: word,
                max_attempts,
                attempts: 0,
                language,
            }),
            None => Err(WordListError::WordListEmpty),
        }
    }

    pub fn take_guess(&mut self, guess: &[char; 5]) -> Result<GuessResult, GameError> {
        if !self.language.wordlist_set().contains(guess) {
            return Err(GameError::WordNotInList);
        }

        let result = take_guess(&self.solution, guess);
        self.attempts += 1;

        let is_won = result.iter().all(|&r| r == LetterResult::Correct);
        let is_last_attempt = !self.has_attempts_left();

        Ok(match (is_won, is_last_attempt) {
            (true, _) => GuessResult::Won(result),
            (false, true) => GuessResult::Lost {
                last_guess: result,
                solution: self.solution,
            },
            (false, false) => GuessResult::Continue(result),
        })
    }

    pub fn has_attempts_left(&self) -> bool {
        self.attempts < self.max_attempts
    }

    pub fn attempts(&self) -> usize {
        self.attempts
    }

    pub fn max_attempts(&self) -> usize {
        self.max_attempts
    }

    pub fn language(&self) -> Language {
        self.language
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_guess() {
        let solution = ['a', 'b', 'c', 'd', 'e'];
        let guess = ['a', 'c', 'e', 'd', 'f'];
        let result = take_guess(&solution, &guess);
        assert_eq!(
            result,
            [
                LetterResult::Correct,
                LetterResult::Misplaced,
                LetterResult::Misplaced,
                LetterResult::Correct,
                LetterResult::Absent
            ]
        );
    }

    #[test]
    fn test_take_guess_double_letters() {
        let solution = ['a', 'a', 'b', 'c', 'd'];
        let guess = ['a', 'b', 'a', 'c', 'a'];
        let result = take_guess(&solution, &guess);
        assert_eq!(
            result,
            [
                LetterResult::Correct,
                LetterResult::Misplaced,
                LetterResult::Misplaced,
                LetterResult::Correct,
                LetterResult::Absent
            ]
        );
    }

    #[test]
    fn test_take_guess_double_letters_at_start_and_end() {
        let solution = ['a', 'x', 'a', 'x', 'a'];
        let guess = ['a', 'a', 'y', 'a', 'a'];
        let result = take_guess(&solution, &guess);
        assert_eq!(
            result,
            [
                LetterResult::Correct,
                LetterResult::Misplaced,
                LetterResult::Absent,
                LetterResult::Absent,
                LetterResult::Correct
            ]
        );
    }
}
