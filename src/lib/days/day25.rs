pub mod puzzle {
    use crate::util::grid::Grid;
    use std::fmt::Display;

    #[derive(Clone, PartialEq)]
    pub enum Occupancy {
        East,
        South,
        Empty,
    }
    impl Occupancy {
        pub fn from_char(c: char) -> Occupancy {
            match c {
                '>' => Occupancy::East,
                'v' => Occupancy::South,
                '.' => Occupancy::Empty,
                _ => panic!("Unexpected character"),
            }
        }
    }
    impl Display for Occupancy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Occupancy::East => f.write_str(">"),
                Occupancy::South => f.write_str("v"),
                Occupancy::Empty => f.write_str("."),
            }
        }
    }

    pub fn advance_herds(map: &mut Grid<Occupancy>) -> bool {
        // East moves, simultaneously
        // South moves, simultaneously

        let mut moves_made = false;
        let mut east_moves = vec![];
        for i in 0..map.height() {
            for j in 0..map.width() {
                if *map.wrapped_get(i, j) == Occupancy::East
                    && *map.wrapped_get(i, j + 1) == Occupancy::Empty
                {
                    east_moves.push((i, j))
                }
            }
        }
        if !east_moves.is_empty() {
            moves_made = true;
        }
        for (i, j) in east_moves {
            *map.wrapped_get_mut(i, j) = Occupancy::Empty;
            *map.wrapped_get_mut(i, j + 1) = Occupancy::East;
        }

        let mut south_moves = vec![];
        for i in 0..map.height() {
            for j in 0..map.width() {
                if *map.wrapped_get(i, j) == Occupancy::South
                    && *map.wrapped_get(i + 1, j) == Occupancy::Empty
                {
                    south_moves.push((i, j))
                }
            }
        }
        if !south_moves.is_empty() {
            moves_made = true;
        }
        for (i, j) in south_moves {
            *map.wrapped_get_mut(i, j) = Occupancy::Empty;
            *map.wrapped_get_mut(i + 1, j) = Occupancy::South;
        }

        moves_made
    }

    mod parser {
        use itertools::Itertools;

        use super::*;

        pub fn parse_input(input: &str) -> Grid<Occupancy> {
            Grid::from_rows(
                input
                    .trim()
                    .lines()
                    .map(|l| l.trim())
                    .map(|l| l.chars().map(|c| Occupancy::from_char(c)).collect_vec())
                    .collect_vec(),
            )
        }
    }

    pub mod part_one {
        use super::{advance_herds, parser};

        pub fn run(input: &str) -> Option<u64> {
            let mut map = parser::parse_input(input);
            let mut moves = 0;
            while advance_herds(&mut map) {
                moves += 1;
            }
            Some(moves + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::puzzle::*;

    #[test]
    fn test_day_one_examples() {
        let example = r#"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
        "#;

        assert_eq!(part_one::run(example), Some(58));
    }
}
