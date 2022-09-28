use rand::prelude::*;
use crate::words::*;

pub struct GameConfig {
  required_char: Option<char>,
  allowed_chars: Vec<char>
}

impl GameConfig {
  pub fn new() -> GameConfig {
    let mut rng = rand::thread_rng();
    let mut chars = random_pangram_chars();
    
    chars.shuffle(&mut rng);

    GameConfig {
      required_char: Some(chars[rng.gen_range(0..chars.len())]),
      allowed_chars: chars
    }
  }

  pub fn allowed_chars(&self) -> &Vec<char> {
    &self.allowed_chars
  }

  pub fn required_char(&self) -> Option<char> {
    self.required_char
  }

  pub fn is_required_char(&self, character: &char) -> bool {
    self.required_char()
      .and_then(|required| Some(&required == character))
      .unwrap_or(true)
  }

  pub fn valid_words(&self) -> Vec<String> {
    words()
      .into_iter()
      .filter(|word| self.word_validity(word).is_valid())
      .collect()
  }

  pub fn max_points(&self) -> u16 {
    points_for_all_words(&self.valid_words())
  }

  pub fn word_validity(&self, word: &str) -> WordValidity {
    if !word.len() < 4 {
      WordValidity::TooShort
    } else if !self.word_matches_allowed_chars(word) {
      WordValidity::BadChars
    } else if !self.contains_required_char(word) {
      WordValidity::MissingRequired
    } else if !is_word(word) {
      WordValidity::NotFound
    } else {
      WordValidity::Valid
    }
  }

  fn contains_required_char(&self, word: &str) -> bool {
    word.chars() 
      .any(|c| self.required_char
        .and_then(|required| Some(required == c))
        .unwrap_or(true)
      )
  }

  fn is_char_allowed(&self, c: &char) -> bool {
    self.allowed_chars()
      .into_iter()
      .any(|allowed| allowed == c)
  }

  fn word_matches_allowed_chars(&self, word: &str) -> bool {
    word.chars()
      .all(|c| self.is_char_allowed(&c))
  }
}

pub enum WordValidity {
  NotTried,
  Valid,
  BadChars,
  TooShort,
  MissingRequired,
  NotFound
}

impl WordValidity {
  pub fn is_valid(&self) -> bool {
    matches!(self, WordValidity::Valid)
  }

  pub fn feedback_message(&self) -> String {
    String::from(match self {
      WordValidity::NotTried => "Enter a guess below.",
      WordValidity::Valid => "Good guess!",
      WordValidity::TooShort => "That word is too short.",
      WordValidity::BadChars => "Your word must only use the characters above.",
      WordValidity::NotFound => "That word was not found in the dictionary.",
      WordValidity::MissingRequired => "Your guess must include the required character",
    })
  }
}

