use hangman::{HangMan, HangManError, HangManEvent, HangManGameState};
use rand::Rng;
use std::{fs::read_to_string, io, vec};

mod hangman;

const DEFAULT_TRIES: u64 = 10;

pub enum GameError {
    FileCouldNotBeRead,
}

const PATH: &str = "words.txt";

fn read_letter() -> String {
    println!("Ingresa una letra: ");

    let mut letter = String::new();

    io::stdin()
        .read_line(&mut letter)
        .expect("Error leyendo la linea.");

    dbg!(&letter);
    letter.trim().to_string()
}

fn get_random_word_from_file(path: &str) -> Result<String, GameError> {
    let mut words: Vec<String> = vec![];
    match read_to_string(path) {
        Ok(string) => string
            .lines()
            .for_each(|word: &str| words.push(word.to_owned())),
        Err(_err) => (),
    };

    if words.is_empty() {
        Err(GameError::FileCouldNotBeRead)
    } else {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen::<usize>() % words.len();
        Ok(words[random_index].to_owned())
    }
}

fn start_game(word: &str) {
    dbg!(word);
    let mut hangman_game = HangMan::new(word, DEFAULT_TRIES + word.len() as u64);

    while hangman_game.game_still_going() {
        let guess_letter = read_letter();

        match hangman_game.play_turn(&guess_letter) {
            Ok(HangManEvent::LetterFound) => println!("¡La letra {} es correcta!", guess_letter),
            Err(HangManError::LetterAlreadyGuessed) => {
                println!("¡La letra {} ya se ha utlizado!", guess_letter)
            }
            Err(HangManError::LetterNotFound) => {
                println!("¡La letra {} no se encuentra en la palabra!", guess_letter)
            }
            Err(HangManError::OutOfTries) => {
                println!("Has agotado la cantidad de intentos")
            }
        }
        println!("{}", hangman_game);
    }

    match hangman_game.game_state() {
        HangManGameState::WordGuessed => println!("¡Has Ganado! Felicitaciones"),
        HangManGameState::OutOfTries => {
            println!("Has perdido. Has agotado la cantidad de intentos posibles...")
        }
        _ => (),
    }
}

fn main() {
    println!("Welcome to FIUBA's Hangman!");

    match get_random_word_from_file(PATH) {
        Ok(random_word) => start_game(&random_word),
        Err(GameError::FileCouldNotBeRead) => {
            println!("¡Hubo un error al leer el archivo de palabras!")
        }
    }
}
