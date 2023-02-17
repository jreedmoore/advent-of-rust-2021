use std::collections::HashMap;

mod puzzle {

    pub struct GameState {
        pub player_one: GamePlayer,
        pub player_two: GamePlayer,
        pub die_rolls: u32,
        pub die_state: u32, // 0-99, NOT 1-100
        pub next_roller: PlayerId,
    }
    impl GameState {
        /// Takes 1-indexed player positions and creates a GameState
        pub fn new(player_one_pos: u32, player_two_pos: u32) -> GameState {
            GameState {
                player_one: GamePlayer {
                    position: player_one_pos - 1,
                    score: 0,
                },
                player_two: GamePlayer {
                    position: player_two_pos - 1,
                    score: 0,
                },
                die_rolls: 0,
                die_state: 0,
                next_roller: PlayerId::PlayerOne,
            }
        }

        pub fn roll_die(&mut self) -> u32 {
            let result = self.die_state + 1;
            self.die_state = (self.die_state + 1) % 100;
            self.die_rolls += 1;
            result
        }

        /// Advance the game one turn.
        /// Return true if the game continues, false if it has ended
        pub fn take_turn(&mut self) -> bool {
            let r1 = self.roll_die();
            let r2 = self.roll_die();
            let r3 = self.roll_die();

            print!(
                "{}",
                match self.next_roller {
                    PlayerId::PlayerOne => "P1: ",
                    PlayerId::PlayerTwo => "P2: ",
                }
            );
            match self.next_roller {
                PlayerId::PlayerOne => self.player_one.move_player(r1 + r2 + r3),
                PlayerId::PlayerTwo => self.player_two.move_player(r1 + r2 + r3),
            }

            self.next_roller = match self.next_roller {
                PlayerId::PlayerOne => PlayerId::PlayerTwo,
                PlayerId::PlayerTwo => PlayerId::PlayerOne,
            };

            self.player_one.score < 1000 && self.player_two.score < 1000
        }
    }
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct GamePlayer {
        pub position: u32, // 0-9 mapping to 1-10
        pub score: u32,
    }
    impl GamePlayer {
        pub fn move_player(&mut self, amount: u32) {
            let next_position = (self.position + amount) % 10;
            self.position = next_position;
            self.score += next_position + 1;
        }
    }
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub enum PlayerId {
        PlayerOne,
        PlayerTwo,
    }

    pub mod parser {
        use super::*;
        use nom::{
            bytes::complete::{tag, take_until},
            character::complete::multispace0,
            combinator::map,
            error::ParseError,
            sequence::{delimited, preceded, tuple},
            IResult,
        };
        /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
        /// trailing whitespace, returning the output of `inner`.
        fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
            inner: F,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where
            F: Fn(&'a str) -> IResult<&'a str, O, E>,
        {
            delimited(multispace0, inner, multispace0)
        }

        fn starting_position(input: &str) -> IResult<&str, u32> {
            preceded(
                tuple((take_until(":"), tag(":"))),
                ws(nom::character::complete::u32),
            )(input)
        }

        // Parses player positions as tuple
        pub fn player_positions(input: &str) -> IResult<&str, (u32, u32)> {
            tuple((ws(starting_position), ws(starting_position)))(input)
        }

        // creates game state
        pub fn puzzle_input(input: &str) -> IResult<&str, GameState> {
            map(player_positions, |(player_one, player_two)| {
                GameState::new(player_one, player_two)
            })(input)
        }
    }

    pub fn parse_input(input: &str) -> Option<GameState> {
        let (_, game) = parser::puzzle_input(input).ok()?;
        Some(game)
    }
}

// Board game simulation with "deterministic dice"
// Each turn takes 3 "rolls" of sequence (1.100)
// sum of rolls gives offset, wrapped into 1 to 10 which gives score for turn
// play until score >= 100
pub fn part_one(input: &str) -> Option<u64> {
    let mut game = puzzle::parse_input(input)?;

    while game.take_turn() {}

    // the loser will always be the player that would have rolled next
    let losing_score = match game.next_roller {
        puzzle::PlayerId::PlayerOne => game.player_one.score,
        puzzle::PlayerId::PlayerTwo => game.player_two.score,
    };

    println!("End of game {:?} / {:?}", game.player_one, game.player_two);
    println!("Losing {} rolls {}", losing_score, game.die_rolls);
    Some((losing_score * game.die_rolls) as u64)
}

// Same board game but now with a "non-deterministic die". Each die roll creates different universes to evaluate.
// Answer to puzzle is the number of universes the winning player wins in.
// Overall strategy will be to use a top-down dynamic programming strategy to evaluate the game space, since different paths will lead to the same game state.
// Since we roll the die 3 times we could model this as either 27 equally likely outcomes, or 6 differently weighted outcomes. Since 6 << 27 we'll use that strategy.
mod part_two {
    use std::collections::HashMap;

    use super::puzzle::{GamePlayer, PlayerId};

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct GameState {
        pub player_one: GamePlayer,
        pub player_two: GamePlayer,
        pub next_player: PlayerId,
    }
    impl GameState {
        pub fn new(player_one_pos: u32, player_two_pos: u32) -> GameState {
            GameState {
                player_one: GamePlayer {
                    position: player_one_pos - 1, // remember, positions are 0 indexed!
                    score: 0,
                },
                player_two: GamePlayer {
                    position: player_two_pos - 1,
                    score: 0,
                },
                next_player: PlayerId::PlayerOne,
            }
        }

        fn play_turn(&self, value: u32) -> GameState {
            let mut next_state = self.clone();
            match self.next_player {
                PlayerId::PlayerOne => next_state.player_one.move_player(value),
                PlayerId::PlayerTwo => next_state.player_two.move_player(value),
            };

            next_state.next_player = match self.next_player {
                PlayerId::PlayerOne => PlayerId::PlayerTwo,
                PlayerId::PlayerTwo => PlayerId::PlayerOne,
            };

            next_state
        }
    }

    #[derive(Clone)]
    pub struct PuzzleOutput {
        pub player_one_wins: u64,
        pub player_two_wins: u64,
    }

    pub struct DieRoll {
        pub value: u32,
        pub occurences: u64,
    }

    pub fn part_two_puzzle(
        start: &GameState,
        memo: &mut HashMap<GameState, PuzzleOutput>,
    ) -> PuzzleOutput {
        let rolls = vec![
            DieRoll {
                value: 3,
                occurences: 1,
            },
            DieRoll {
                value: 4,
                occurences: 3,
            },
            DieRoll {
                value: 5,
                occurences: 6,
            },
            DieRoll {
                value: 6,
                occurences: 7,
            },
            DieRoll {
                value: 7,
                occurences: 6,
            },
            DieRoll {
                value: 8,
                occurences: 3,
            },
            DieRoll {
                value: 9,
                occurences: 1,
            },
        ];

        if start.player_one.score >= 21 {
            PuzzleOutput {
                player_one_wins: 1,
                player_two_wins: 0,
            }
        } else if start.player_two.score >= 21 {
            PuzzleOutput {
                player_one_wins: 0,
                player_two_wins: 1,
            }
        } else {
            let mut acc = vec![];
            for roll in rolls {
                let successor_state = start.play_turn(roll.value);
                if let Some(output) = memo.get(&successor_state) {
                    acc.push((output.clone(), roll.occurences));
                } else {
                    let output = part_two_puzzle(&successor_state, memo);
                    memo.insert(successor_state, output.clone());
                    acc.push((output, roll.occurences));
                }
            }

            let mut player_one_wins = 0;
            let mut player_two_wins = 0;
            for (output, occurences) in acc {
                player_one_wins += output.player_one_wins * occurences;
                player_two_wins += output.player_two_wins * occurences;
            }

            PuzzleOutput {
                player_one_wins: player_one_wins,
                player_two_wins: player_two_wins,
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (one, two)) = puzzle::parser::player_positions(input).ok()?;
    let game_state = part_two::GameState::new(one, two);
    let output = part_two::part_two_puzzle(&game_state, &mut HashMap::new());

    Some(std::cmp::max(
        output.player_one_wins,
        output.player_two_wins,
    ))
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

    #[test]
    fn test_part_one_example_parser() {
        let example = r#"
    Player 1 starting position: 4
    Player 2 starting position: 8"#;

        let (_, (one, two)) = puzzle::parser::player_positions(example).unwrap();
        assert_eq!(one, 4);
        assert_eq!(two, 8);
    }

    #[test]
    fn test_part_two_example() {
        let example = r#"
    Player 1 starting position: 4
    Player 2 starting position: 8"#;

        assert_eq!(part_two(example), Some(444356092776315));
    }
}
