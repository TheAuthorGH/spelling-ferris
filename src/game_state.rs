use colored::*;
use crate::words::*;
use crate::game_config::*;

pub struct GameState {
  game_config: GameConfig,
  found_words: Vec<String>,
  last_validity: WordValidity
}

impl GameState {
  pub fn new(game_config: GameConfig) -> GameState {
    GameState {
      game_config,
      found_words: Vec::new(),
      last_validity: WordValidity::NotTried
    }
  }

  pub fn game_config(&self) -> &GameConfig {
    &self.game_config
  }

  pub fn found_words(&self) -> &Vec<String> {
    &self.found_words
  }

  pub fn last_validity(&self) -> &WordValidity {
    &self.last_validity
  }

  pub fn earned_points(&self) -> u16 {
    points_for_all_words(self.found_words())
  }

  pub fn try_word(&mut self, word: &str) {
    self.last_validity = self.game_config().word_validity(word);

    if self.last_validity.is_valid() {
      self.found_words.push(word.to_owned());
    }
  }
}

