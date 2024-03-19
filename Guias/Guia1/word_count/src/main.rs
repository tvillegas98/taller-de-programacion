use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

const PATH: &str = "words.txt";

fn read_file(path: &str) -> Result<HashMap<String, u64>, Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let mut words_occurrences: HashMap<String, u64> = HashMap::new();

    for line in buffer.lines() {
        let line = line?;
        words_occurrences
            .entry(line)
            .and_modify(|ocurrences| *ocurrences += 1)
            .or_insert(1);
    }
    Ok(words_occurrences)
}

fn show_words_ocurrences(words_occurrences: &HashMap<String, u64>) {
    let mut words_occurrences: Vec<(String, u64)> = words_occurrences
        .iter()
        .map(|(word, occurrences)| (word.to_owned(), occurrences.to_owned()))
        .collect();

    words_occurrences.sort_by(|(_, a), (_, b)| a.cmp(b));

    for (word, occurences) in words_occurrences{
        println!("{} -> {}", word, occurences);
    }
}

fn main() {
    let words_occurences = read_file(PATH);
    match words_occurences {
        Ok(words_occurrences) => show_words_ocurrences(&words_occurrences),
        Err(_err) => println!("Hubo un error al leer el archivo. El programa se cerrará"),
    }
}
