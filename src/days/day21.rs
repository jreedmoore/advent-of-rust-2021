// Board game simulation with "deterministic dice"
// Each turn takes 3 "rolls" of sequence (1.100)
// sum of rolls gives offset, wrapped into 1 to 10 which gives score for turn
// play until score >= 100
mod puzzle {

  pub struct GameState {
    pub player_one: GamePlayer,
    pub player_two: GamePlayer,
    pub die_rolls: u32,
    pub next_roller: PlayerId
  }
  impl GameState {
    pub fn take_turn<I: Iterator<Item = u8>>(&mut self, die: &mut I) -> bool {
      todo!()
    }
  }
  pub struct GamePlayer {
    pub position: u8,
    pub score: u32,
  }
  pub enum PlayerId {
    PlayerOne,
    PlayerTwo,
  }

  pub fn deterministic_die() -> impl Iterator<Item = u8> {
    0..=100u8
  }

  pub fn parse_input(input: &str) -> Option<GameState> {
    todo!()
  }
}

pub fn part_one(input: &str) -> Option<u64> {
  let mut game = puzzle::parse_input(input)?;
  let mut die = puzzle::deterministic_die();

  while !game.take_turn(&mut die) {
    ()
  }

  let losing_score = match game.next_roller {
    puzzle::PlayerId::PlayerOne => game.player_one.score,
    puzzle::PlayerId::PlayerTwo => game.player_two.score,
  };

  Some((losing_score * game.die_rolls) as u64)
 }

 #[cfg(test)]
mod tests {
  use super::*;

 #[test]
 fn test_part_one_example() {
   let example = r#"
   Player 1 starting position: 4
   Player 2 starting position: 8"#;

   assert_eq!(part_one(example), Some(739785));
 }
}