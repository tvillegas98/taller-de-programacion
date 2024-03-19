use core::fmt;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum HangManGameState {
    WordGuessed,
    OutOfTries,
    OnGoing,
}

#[derive(Debug, Clone)]
pub struct HangMan {
    word: String,
    game_state: HangManGameState,
    letters_to_guess: HashSet<String>,
    guessed_letters: HashSet<String>,
    tries: u64,
}

pub enum HangManError {
    LetterNotFound,
    LetterAlreadyGuessed,
    OutOfTries,
}

pub enum HangManEvent {
    LetterFound,
}

impl HangMan {
    pub fn new(word: &str, tries: u64) -> HangMan {
        let mut letters_to_guess: HashSet<String> = HashSet::new();

        word.chars().for_each(|letter| {
            letters_to_guess.insert(letter.to_string());
        });

        HangMan {
            word: word.to_owned(),
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
            return Err(HangManError::OutOfTries);
        } else {
            self.tries -= 1;
        }

        if self.letter_already_guessed(letter) {
            return Err(HangManError::LetterAlreadyGuessed);
        } else if !self.letter_in_word(letter) {
            self.guessed_letters.insert(letter.to_owned());
            return Err(HangManError::LetterNotFound);
        }

        self.guessed_letters.insert(letter.to_owned());

        if self.word_guessed() {
            self.game_state = HangManGameState::WordGuessed;
        }

        Ok(HangManEvent::LetterFound)
    }

    fn letter_already_guessed(&self, letter: &String) -> bool {
        self.guessed_letters.contains(letter)
    }

    fn letter_in_word(&self, letter: &String) -> bool {
        self.letters_to_guess.contains(letter)
    }

    fn out_of_tries(&self) -> bool {
        self.tries == 0
    }

    fn word_guessed(&self) -> bool {
        self.word
            .chars()
            .all(|letter| self.guessed_letters.contains(&letter.to_string()))
    }
}

impl fmt::Display for HangMan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hangman_word_status = String::new();
        hangman_word_status.push_str("La palabra has el momento es: ");

        for character in self.word.chars() {
            if self.guessed_letters.contains(&character.to_string()) {
                hangman_word_status.push(character);
            } else {
                hangman_word_status.push('_');
            }
        }

        hangman_word_status.push('\n');
        hangman_word_status.push_str("Letras utilizadas: ");

        for character in &self.guessed_letters {
            hangman_word_status.push_str(character.as_str())
        }

        write!(f, "{}", hangman_word_status)
    }
}
