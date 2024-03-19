use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub enum HangManGameState {
    WordGuessed,
    OutOfTries,
    OnGoing,
}

#[derive(Debug, Clone)]
pub struct HangMan {
    game_state: HangManGameState,
    letters_to_guess: HashMap<String, u16>,
    guessed_letters: HashSet<String>,
    tries: u64,
}

pub enum HangManError {
    LetterNotFound,
    LetterAlreadyGuessed,
    OutOfTries
}

pub enum HangManEvent {
    LetterFound,
}

impl HangMan {
    pub fn new(word: &str, tries: u64) -> HangMan {
        let mut letters_to_guess: HashMap<String, u16> = HashMap::new();

        word.chars().for_each(|letter| {
            let letter: String = letter.to_string();
            match letters_to_guess.get(&letter) {
                Some(ocurrences) => letters_to_guess.insert(letter, ocurrences + 1),
                None => letters_to_guess.insert(letter, 1),
            };
        });

        HangMan {
            game_state: HangManGameState::OnGoing,
            guessed_letters: HashSet::new(),
            letters_to_guess,
            tries,
        }
    }

    pub fn game_still_going(&self) -> bool {
        matches!(self.game_state, HangManGameState::OnGoing)
    }

    pub fn game_state(&self) -> HangManGameState {
        self.game_state
    }

    pub fn play_turn(&mut self, letter: &String) -> Result<HangManEvent, HangManError> {
        if self.out_of_tries() {
            self.game_state = HangManGameState::OutOfTries;
            return Err(HangManError::OutOfTries)
        }else {
            self.tries -= 1;
        }

        if self.letter_already_guessed(letter) || self.letter_without_occurrences_left(letter) {
            return Err(HangManError::LetterAlreadyGuessed);
        } else if self.letter_not_in_word(letter) {
            return Err(HangManError::LetterNotFound);
        }

        let occurrences = self.letters_to_guess[letter] - 1;
        self.letters_to_guess.insert(letter.to_owned(), occurrences);

        if self.word_guessed() {
            self.game_state = HangManGameState::WordGuessed;
        }

        Ok(HangManEvent::LetterFound)
    }

    fn letter_already_guessed(&self, letter: &String) -> bool {
        self.guessed_letters.contains(letter)
    }

    fn letter_without_occurrences_left(&self, letter: &String) -> bool {
        if !self.letters_to_guess.contains_key(letter) {
            return false;
        }
        self.letters_to_guess[letter] == 0
    }

    fn letter_not_in_word(&self, letter: &String) -> bool {
        !self.letters_to_guess.contains_key(letter)
    }

    fn out_of_tries(&self) -> bool {
        self.tries == 0
    }

    fn word_guessed(&self) -> bool{
        self.letters_to_guess.values().all(|occurrences | occurrences.eq(&0))
    }

}
