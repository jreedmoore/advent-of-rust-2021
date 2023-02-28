// AoC 2021 Day 23
// Amiphod reshuffling
// We're given a starting arrangement of amiphods and are asked to sort them
// into their destination rooms. We have a set of rules that govern their
// movement and are asked to minimize their energy expenditure.

// This smells like a classic implicit graph search problem, so we'll attack it with more or less Dijkstra's algorithm.
pub mod puzzle {
    use std::fmt;


    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
    pub enum Amphipod {
        A,
        B,
        C,
        D,
    }
    impl Amphipod {
        pub fn move_cost(&self) -> u64 {
            match self {
                Amphipod::A => 1,
                Amphipod::B => 10,
                Amphipod::C => 100,
                Amphipod::D => 1000,
            }
        }

        pub fn destination_room(&self) -> usize {
            match self {
                Amphipod::A => 0,
                Amphipod::B => 1,
                Amphipod::C => 2,
                Amphipod::D => 3,
            }
        }
    }
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
    pub enum SpaceState {
        Occupied(Amphipod),
        Empty,
    }
    impl fmt::Display for SpaceState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SpaceState::Empty => write!(f, "."),
                SpaceState::Occupied(Amphipod::A) => write!(f, "A"),
                SpaceState::Occupied(Amphipod::B) => write!(f, "B"),
                SpaceState::Occupied(Amphipod::C) => write!(f, "C"),
                SpaceState::Occupied(Amphipod::D) => write!(f, "D"),
            }
        }
    }
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
    pub struct BurrowState {
        rooms: Vec<Vec<SpaceState>>,
        hall: Vec<SpaceState>,
    }
    // Goal state is this
    // #############
    // #...........#
    // ###A#B#C#D###
    //   #A#B#C#D#
    //   #########
    impl fmt::Display for BurrowState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "#############\n")?;
            write!(f, "#")?;
            for space in &self.hall {
                write!(f, "{}", space)?;
            }
            write!(f, "#\n")?;
            write!(f, "###{}#{}#{}#{}###\n", self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0])?;
            write!(f, "  #{}#{}#{}#{}#  \n", self.rooms[0][1], self.rooms[1][1], self.rooms[2][1], self.rooms[3][1])?;
            write!(f, "  #########\n")
        }
    }
    impl BurrowState {
        pub fn successors(&self) -> Vec<(BurrowState, u64)> {
            // Rules of moves, states as given are:
            // 1. Amphipods will never stop right outside of a room
            // 2. Amphipods will never move into a room unless it's their destination and it only contains the right type
            // 3. Amphipods will not move from their spot in the hallway until they move into their room

            // hall ranges from 0 to 10
            // rooms[0] connects to hall[2]
            // rooms[1] connects to hall[4]
            // rooms[2] connects to hall[6]
            // rooms[3] connects to hall[8]
            // (those spaces can't be ocupied so they're not really necesarry but w/e for now)

            // occupiable spaces hall[0,1,3,5,7,9,10]
            let mut acc: Vec<(BurrowState, u64)> = vec![];

            let open_hallway_pos: Vec<usize> = vec![0,1,3,5,7,9,10].into_iter().filter(|idx| self.hall[*idx] == SpaceState::Empty).collect();

            // generate moves from rooms into hallway
            for room_idx in 0..4 {
                if let Some((amphipod, pos)) = self.get_from_room(room_idx) {
                    let room_hall_pos = BurrowState::room_hall_pos(room_idx);
                    for hallway_pos in &open_hallway_pos {
                        if self.hall_unoccupied(room_hall_pos, *hallway_pos) {
                            let mut succ_state = self.clone();
                            succ_state.rooms[room_idx][pos] = SpaceState::Empty;
                            succ_state.hall[*hallway_pos] = SpaceState::Occupied(amphipod);
                            let moves = room_hall_pos.abs_diff(*hallway_pos) + pos;

                            acc.push((succ_state, moves as u64 * amphipod.move_cost()));
                        }
                    }
                }
            }

            // generate moves from hallway into room
            for (hallway_pos,amphipod) in self.hall.iter().enumerate().filter_map(|(i,state)| match state { SpaceState::Empty => None, SpaceState::Occupied(amphipod) => Some((i, amphipod))}) {
                let destination = amphipod.destination_room();
                let destination_available = self.rooms[destination].iter().all(|state| *state == SpaceState::Empty || *state == SpaceState::Occupied(*amphipod));
                if destination_available && self.hall_unoccupied(hallway_pos, BurrowState::room_hall_pos(destination)) {
                    let pos = self.rooms[destination].iter().position(|state| *state == SpaceState::Empty).unwrap();

                    let mut succ_state = self.clone();
                    succ_state.hall[hallway_pos] = SpaceState::Empty;
                    succ_state.rooms[destination][pos] = SpaceState::Occupied(*amphipod);

                    let moves = BurrowState::room_hall_pos(destination).abs_diff(hallway_pos) + pos;

                    acc.push((succ_state, moves as u64 * amphipod.move_cost()))
                }
            }

            acc
        }

        fn room_hall_pos(room_idx: usize) -> usize {
            room_idx*2 + 2
        }

        // Is the hall empty from and to indexes, inclusive.
        fn hall_unoccupied(&self, from: usize, to: usize) -> bool {
            if (from <= to) {
                (from..=to).all(|idx| self.hall[idx] == SpaceState::Empty)
            } else {
                (to..=from).all(|idx| self.hall[idx] == SpaceState::Empty)
            }
        }

        pub fn is_goal(&self) -> bool {
            self.rooms[0]
                .iter()
                .all(|s| *s == SpaceState::Occupied(Amphipod::A))
                && self.rooms[1]
                    .iter()
                    .all(|s| *s == SpaceState::Occupied(Amphipod::B))
                && self.rooms[2]
                    .iter()
                    .all(|s| *s == SpaceState::Occupied(Amphipod::C))
                && self.rooms[3]
                    .iter()
                    .all(|s| *s == SpaceState::Occupied(Amphipod::D))
        }

        // Return the movable amphipod and its position in the room if any
        pub(crate) fn get_from_room(&self, room_idx: usize) -> Option<(Amphipod, usize)> {
            self.rooms[room_idx].iter().enumerate().find_map(|(i, state)| match state { &SpaceState::Empty => None, &SpaceState::Occupied(amphipod) => Some((amphipod, i))})
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_goal_state() {
            let goal = r#"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
            "#;

            assert!(parser::parse_input(goal).unwrap().is_goal());
        }
    }
    mod parser {
        use super::*;

        fn parse_amphipod(input: char) -> Option<Amphipod> {
            match input {
                'A' => Some(Amphipod::A),
                'B' => Some(Amphipod::B),
                'C' => Some(Amphipod::C),
                'D' => Some(Amphipod::D),
                _ => None,
            }
        }

        pub fn parse_input(input: &str) -> Option<BurrowState> {
            let lines: Vec<&str> = input.trim().lines().collect();
            let mut hallway: Vec<SpaceState> = vec![];
            for space in lines[1].chars() {
                if space == '.' {
                    hallway.push(SpaceState::Empty);
                } else if space == '#' {
                    ()
                } else {
                    hallway.push(SpaceState::Occupied(parse_amphipod(space)?));
                }
            }

            // 0 becomes rooms[0][0], 4 becomes rooms[0][1], and so on.
            let mut rooms_transposed: Vec<SpaceState> = vec![];
            for space in (lines[2].to_owned() + lines[3]).chars() {
                if let Some(amphipod) = parse_amphipod(space) {
                    rooms_transposed.push(SpaceState::Occupied(amphipod))
                }
            }

            Some(BurrowState {
                rooms: vec![
                    vec![rooms_transposed[0], rooms_transposed[4]],
                    vec![rooms_transposed[1], rooms_transposed[5]],
                    vec![rooms_transposed[2], rooms_transposed[6]],
                    vec![rooms_transposed[3], rooms_transposed[7]],
                ],
                hall: hallway,
            })
        }
    }

    pub mod part_one {
        use super::*;

        fn search(initial: BurrowState) -> Option<(BurrowState, u64)> {
            use std::collections::BinaryHeap;
            use std::cmp::Reverse;

            let mut q = BinaryHeap::new();

            q.push(Reverse((0, initial)));
            while let Some(Reverse((cost, next))) = q.pop() {
                //println!("State cost = {}:\n{}", cost, next);
                if next.is_goal() {
                    return Some((next, cost));
                }

                for (succ, cost_inc) in next.successors() {
                    if cost + cost_inc <= 100000 {
                        q.push(Reverse((cost + cost_inc, succ)))
                    }
                }
            }

            // no path found somehow
            return None;
        }

        pub fn run(input: &str) -> Option<u64> {
            let initial = parser::parse_input(input)?;
            println!("{}", initial);
            let (_, energy) = search(initial)?;
            Some(energy)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let example = r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
        "#;

        assert_eq!(puzzle::part_one::run(example), Some(12521))
    }
}
