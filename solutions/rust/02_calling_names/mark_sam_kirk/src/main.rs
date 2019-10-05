use std::fs;
use rand::prelude::*;

fn main() {
    let nouns = fs::read_to_string("nouns.txt").expect("Something went wrong");
    let verbs = fs::read_to_string("verbs.txt").expect("Something went wrong");

    let rng = rand::thread_rng();
    let first_noun_list: Vec<&str> = shuffled_words(&nouns, rng);
    let second_noun_list: Vec<&str> = shuffled_words(&nouns, rng);
    let verb_list: Vec<&str> = shuffled_words(&verbs, rng);

    let zipped = first_noun_list.iter()
        .zip(verb_list.iter())
        .map(|t| format!("{ } { }", t.0, t.1))
        .zip(second_noun_list.iter())
        .map(|t| format!("{ } { }", t.0, t.1));

    for item in zipped {
        println!("{ }", item);
    }
}

fn shuffled_words (words: &str, mut rng: impl RngCore) -> Vec<&str> {
    let mut word_vect: Vec<&str> = words.lines().collect();
    word_vect.shuffle(&mut rng);
    return word_vect;
}
