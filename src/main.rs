use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::env;
use std::fs;

const MAX_TRIES: usize = 6;

type Word = [u8; 5];

fn string_to_u8(word: &str) -> Word {
    let mut arr = [0u8; 5];
    for (i, c) in word.chars().enumerate() {
        arr[i] = (c as u8) - ('a' as u8);
    }
    arr
}

fn u8_to_string(word: &Word) -> String {
    word.iter().map(|&b| (b + ('a' as u8)) as char).collect()
}

fn get_words() -> Vec<String> {
    let filename = "wordle_words.txt";
    let text = fs::read_to_string(filename).expect("Unable to read the file");
    let mut words: Vec<String> = text.lines().map(|s| s.trim().to_lowercase()).collect();
    words.sort();
    words
}

fn evolve(l: &[[bool; 5]; 26], word: &Word, guess: &Word) -> [[bool; 5]; 26] {
    let mut new_l = *l;

    for i in 0..5 {
        if guess[i] == word[i] {
            for j in 0..26 {
                new_l[j][i] = false;
            }
            new_l[word[i] as usize][i] = true;
        } else if !word.contains(&guess[i]) {
            for j in 0..5 {
                new_l[guess[i] as usize][j] = false;
            }
        } else {
            new_l[guess[i] as usize][i] = false;
        }
    }

    new_l
}

fn reduce(w: &Vec<Word>, l: &[[bool; 5]; 26]) -> Vec<Word> {
    w.iter()
        .filter(|&&word| {
            word.iter()
                .enumerate()
                .all(|(i, &letter)| l[letter as usize][i])
        })
        .cloned()
        .collect()
}

fn reduce_len(w: &Vec<Word>, l: &[[bool; 5]; 26]) -> usize {
    w.iter()
        .filter(|&&word| {
            word.iter()
                .enumerate()
                .all(|(i, &letter)| l[letter as usize][i])
        })
        .count()
}

fn expected_reduction(guess: &Word, w: &Vec<Word>, l: &[[bool; 5]; 26]) -> f64 {
    let g = reduce(w, l);
    let e: f64 = g.iter()
        .map(|&word| reduce_len(&g, &evolve(l, &word, guess)) as f64)
        .sum::<f64>()
        / g.len() as f64;

    g.len() as f64 - e
}

fn optimal_guess(w: &Vec<Word>, l: &[[bool; 5]; 26]) -> Word {
    let style = ProgressStyle::default_bar();
    let reductions: Vec<f64> = w
        .par_iter()
        .progress_with_style(style)
        .map(|&guess| expected_reduction(&guess, w, l))
        .collect();

    let max_index = reductions
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    w[max_index]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: program_name word [guesses...]");
        return;
    }

    let word_str = &args[1];
    let word = string_to_u8(word_str);
    let w_str = get_words();
    let w: Vec<Word> = w_str.iter().map(|s| string_to_u8(s)).collect();

    if !w_str.contains(word_str) {
        println!("Word {} not in dictionary.", word_str);
        return;
    }

    let mut l = [[true; 5]; 26];

    let mut g = w.clone();

    if args.len() > 2 {
        for (i, guess_str) in args[2..].iter().enumerate() {
            if guess_str.len() != 5 {
                println!("All guesses must be 5 letters long.");
                return;
            }

            let len_g = g.len();
            let guess = string_to_u8(guess_str);
            l = evolve(&l, &word, &guess);
            g = reduce(&w, &l);

            println!(
                "Guess {}/{}: {} ({} words remaining) reduction: {}",
                i + 1,
                MAX_TRIES,
                guess_str,
                g.len(),
                len_g - g.len()
            );
        }
    }

    if g.len() == 1 {
        println!("Solution: {}", u8_to_string(&g[0]));
        return;
    }

    for i in (args.len() - 2)..(MAX_TRIES) {
        let len_g = g.len();
        let best_guess = optimal_guess(&w, &l);
        let expected = expected_reduction(&best_guess, &w, &l);

        l = evolve(&l, &word, &best_guess);
        g = reduce(&w, &l);
        let actual = len_g - g.len();

        println!(
            "Guess {}/{}: {} ({} words remaining) reduction: {} (expected {:.2})",
            i + 1,
            MAX_TRIES,
            u8_to_string(&best_guess),
            g.len(),
            actual,
            expected
        );

        if (g.len() == 1) && (i < MAX_TRIES - 1) {
            println!("Solution {}/{}: {}", i + 2, MAX_TRIES, u8_to_string(&g[0]));
            break;
        }
    }

    if g.len() > 1 {
        println!("Failed.");
    }
}
