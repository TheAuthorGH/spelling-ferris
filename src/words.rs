use std::sync::Mutex;
use std::cmp;
use std::fs;
use lazy_static::lazy_static;
use itertools::Itertools;
use rand::prelude::*;

pub fn words() -> Vec<String> {
  lazy_static! {
    static ref WORDS: Mutex<Option<Vec<String>>> = Mutex::new(None);
  }

  let mut words_value = WORDS.lock().unwrap();

  match &*words_value {
    None => {
      let new_words = read_words_from_file();
      *words_value = Some(new_words.clone());

      new_words
    },
    Some(value) => value.clone()
  }
}

pub fn pangrams() -> Vec<String> {
  words()
    .into_iter()
    .filter(|word| chars_in_word(word) == 7) 
    .collect()
}

pub fn random_pangram_chars() -> Vec<char> {
  let mut rng = rand::thread_rng();

  let pangrams = pangrams(); 
  let pangram = &pangrams[rng.gen_range(0..pangrams.len())];

  pangram.chars()
    .unique()
    .collect()
}

pub fn chars_in_word(word: &str) -> u8 {
  word.chars()
    .unique()
    .count() as u8
}

pub fn points_for_word(word: &str) -> u16 {
  cmp::max(1, (word.chars().count() as u16) - 3)
}

pub fn points_for_all_words(words: &Vec<String>) -> u16 {
  words
    .into_iter()
    .map(|word| points_for_word(&word))
    .reduce(|points, word| points + word)
    .unwrap_or(0)
}

pub fn is_word(word: &str) -> bool {
  words()
    .into_iter()
    .any(|w| w.eq(word))
}

fn read_words_from_file() -> Vec<String> {
  let contents = fs::read_to_string("./src/words.txt")
    .expect("Internal error - This must be fixed.");

  contents.split("\n")
    .map(|s| s.to_owned())
    .unique()
    .collect()
}
