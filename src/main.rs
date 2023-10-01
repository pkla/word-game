use std::collections::HashSet;
use std::fs;
use std::env;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

const MAX_TRIES: usize = 5;

type Word = [u8; 5];

fn char_to_u8(c: char) -> u8 {
    (c as u8) - ('a' as u8)
}

fn u8_to_char(byte: u8) -> char {
    (byte + ('a' as u8)) as char
}

fn string_to_word(s: &str) -> Word {
    let mut word = [0; 5];
    for (i, c) in s.chars().enumerate() {
        word[i] = char_to_u8(c);
    }
    word
}

fn get_words() -> Vec<Word> {
    let filename = "wordle_words.txt";
    let text = fs::read_to_string(filename).expect("Unable to read the file");
    let words: HashSet<Word> = text
        .to_lowercase()
        .lines()
        .map(|line| string_to_word(line.trim()))
        .collect();
    let mut words: Vec<Word> = words.into_iter().collect();
    words.sort();
    words
}

fn evolve(letter_sets: Vec<HashSet<u8>>, word: &Word, guess: &Word) -> Vec<HashSet<u8>> {
    let mut new_letter_sets = letter_sets.clone();

    for i in 0..MAX_TRIES {
        if guess[i] == word[i] {
            new_letter_sets[i] = [word[i]].iter().cloned().collect();
        } else if !word.contains(&guess[i]) {
            for new_set in new_letter_sets.iter_mut() {
                new_set.remove(&guess[i]);
            }
        } else {
            new_letter_sets[i].remove(&guess[i]);
        }
    }

    new_letter_sets
}

fn reduce(words: &[Word], letter_sets: &[HashSet<u8>]) -> Vec<Word> {
    words
        .iter()
        .filter(|&word| {
            word.iter().enumerate().all(|(i, &c)| letter_sets[i].contains(&c))
        })
        .cloned()
        .collect()
}

fn reduce_count(words: &[Word], letter_sets: &[HashSet<u8>]) -> usize {
    words
        .iter()
        .filter(|&word| {
            word.iter().enumerate().all(|(i, &c)| letter_sets[i].contains(&c))
        })
        .count()
}

fn expected_reduction(guess: &Word, words: &[Word], letter_sets: &[HashSet<u8>]) -> f64 {
    let g = reduce(words, letter_sets);
    let n = g.len() as f64;
    let e: f64 = g.iter()
        .map(|word| reduce_count(&g, &evolve(letter_sets.to_vec(), &word, guess)) as f64)
        .sum::<f64>()
        / g.len() as f64;
    n - e
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected a word as the first argument");
    }

    let word_str = &args[1];
    let word = string_to_word(word_str);
    let words = get_words();
    let mut letter_sets: Vec<HashSet<u8>> = (0..MAX_TRIES)
        .map(|i| words.iter().map(|w| w[i]).collect())
        .collect();

    if args.len() > 2 {
        for guess_str in args[2..].iter() {
            let guess = string_to_word(guess_str);
            letter_sets = evolve(letter_sets, &word, &guess);
            let g = reduce(&words, &letter_sets);
            println!("Guess: {} ({} words remaining)", guess_str, g.len());
        }
    }

    let g = reduce(&words, &letter_sets);
    if g.len() == 1 {
        println!("Solution: {}", word_str);
        return;
    }

    for _ in 0..MAX_TRIES {
        let pb = ProgressBar::new(words.len() as u64);
        pb.set_style(ProgressStyle::default_bar());
        let reductions: Vec<f64> = words.par_iter()
            .map(|guess| {
                pb.inc(1);
                expected_reduction(guess, &words, &letter_sets)
            })
            .collect();
        pb.finish_and_clear();

        let max_reduction = reductions.iter().cloned().fold(0. / 0., f64::max); // NaN as initial value to handle empty vec
        let best_guess_index = reductions.iter().position(|&r| r == max_reduction).unwrap();
        let best_guess = words[best_guess_index];
        let best_guess_str: String = best_guess.iter().map(|&c| u8_to_char(c)).collect();

        letter_sets = evolve(letter_sets, &word, &best_guess);
        let g = reduce(&words, &letter_sets);

        println!("Guess: {} ({} words remaining) - auto", best_guess_str, g.len());

        if g.len() == 1 {
            let solution = g[0];
            let solution_str: String = solution.iter().map(|&c| u8_to_char(c)).collect();
            println!("Solution: {}", solution_str);
            break;
        }
    }
}
