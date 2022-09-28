use colored::*;
use game_config::*;
use game_state::*;

mod words;
mod game_config;
mod game_state;

fn main() {
  let mut game_state = GameState::new(GameConfig::new());

  // TODO: Move all of this terminal nonsense to its own module.
  //  Allow multiple frontends, not just terminal.
  loop {
    print!("\x1B[2J\x1B[1;1H");

    print_game_status(&game_state);
    accept_user_input(&mut game_state);
  }
}

fn print_game_status(game_state: &GameState) {
  let game_config = game_state.game_config();

  println!("{}", "┌────────────────────┐".truecolor(0, 255, 136));

  print!("  ");
  for character in game_config.allowed_chars() {
    let color = if game_config.is_required_char(character) {
      "yellow" 
    } else {
      "cyan"
    };

    print!("{} ", character.to_string().to_uppercase().color(color).bold());
  }
  println!();
  println!();

  let points = game_state.earned_points();
  let max_points = game_config.max_points();


  let completeness = ((points as f64 / max_points as f64) * 10.0).ceil() as u16;
  print!("  {}{}", "█".repeat(completeness as usize).green(), "░".repeat(10 - completeness as usize).green(),);

  println!(" {}/{}", points.to_string().green().bold(), max_points.to_string());
}

fn accept_user_input(game_state: &mut GameState) {
  println!("\n{}", "└────────────────────┘".yellow());

  println!("\n~ {}\n", game_state.last_validity().feedback_message());

  let mut input = String::new();        

  std::io::stdin()
    .read_line(&mut input)
    .expect("Internal error - this must be fixed");

  game_state.try_word(input.trim());
}
