// AoC 2021 Day 23
// Amiphod reshuffling
// We're given a starting arrangement of amiphods and are asked to sort them
// into their destination rooms. We have a set of rules that govern their
// movement and are asked to minimize their energy expenditure.

// This smells like a classic implicit graph search problem, so we'll attack it with more or less Dijkstra's algorithm.
pub mod puzzle {
    use std::{fmt, collections::HashMap};

    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
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
            for i in 0..self.rooms[0].len() {
                write!(
                    f,
                    "  #{}#{}#{}#{}#  \n",
                    self.rooms[0][i], self.rooms[1][i], self.rooms[2][i], self.rooms[3][i]
                )?;
            }
            write!(f, "  #########\n")
        }
    }
    impl fmt::Debug for BurrowState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self, f)
        }
    }
    impl BurrowState {

        pub(crate) fn to_hallway_moves(&self) -> Vec<(BurrowState, u64)> {
            let mut acc: Vec<(BurrowState, u64)> = vec![];

            let open_hallway_pos: Vec<usize> = vec![0, 1, 3, 5, 7, 9, 10]
                .into_iter()
                .filter(|idx| self.hall[*idx] == SpaceState::Empty)
                .collect();

            // generate moves from rooms into hallway
            for room_idx in 0..4 {
                if let Some((amphipod, pos)) = self.get_from_room(room_idx) {
                    let room_hall_pos = BurrowState::room_hall_pos(room_idx);
                    for hallway_pos in &open_hallway_pos {
                        if self.hall_unoccupied(room_hall_pos, *hallway_pos) {
                            let mut succ_state = self.clone();
                            succ_state.rooms[room_idx][pos] = SpaceState::Empty;
                            succ_state.hall[*hallway_pos] = SpaceState::Occupied(amphipod);
                            let moves = room_hall_pos.abs_diff(*hallway_pos) + 1 + pos;

                            acc.push((succ_state, moves as u64 * amphipod.move_cost()));
                        }
                    }
                }
            }

            acc
        }

        pub(crate) fn from_hallway_moves(&self) -> Vec<(BurrowState, u64)> {
            let mut acc: Vec<(BurrowState, u64)> = vec![];

            for (hallway_pos, amphipod) in
                self.hall
                    .iter()
                    .enumerate()
                    .filter_map(|(i, state)| match state {
                        SpaceState::Empty => None,
                        SpaceState::Occupied(amphipod) => Some((i, amphipod)),
                    })
            {
                let destination = amphipod.destination_room();
                let destination_available = self.rooms[destination].iter().all(|state| {
                    *state == SpaceState::Empty || *state == SpaceState::Occupied(*amphipod)
                });
                let pathable = self.hall_pathable(hallway_pos, BurrowState::room_hall_pos(destination));
                if destination_available
                    && self.hall_pathable(hallway_pos, BurrowState::room_hall_pos(destination))
                {
                    let pos = self.rooms[destination]
                        .iter()
                        .rposition(|state| *state == SpaceState::Empty)
                        .unwrap();

                    let mut succ_state = self.clone();
                    succ_state.hall[hallway_pos] = SpaceState::Empty;
                    succ_state.rooms[destination][pos] = SpaceState::Occupied(*amphipod);

                    let moves = BurrowState::room_hall_pos(destination).abs_diff(hallway_pos) + 1 + pos;

                    acc.push((succ_state, moves as u64 * amphipod.move_cost()))
                }
            }
            acc
        }

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
            

            // generate moves from hallway into room
            let mut to = self.to_hallway_moves();
            let mut from = self.from_hallway_moves();
            
            to.append(&mut from);

            to
        }

        fn room_hall_pos(room_idx: usize) -> usize {
            room_idx * 2 + 2
        }

        // Is the hall empty from and to indexes, inclusive.
        fn hall_unoccupied(&self, from: usize, to: usize) -> bool {
            if from <= to {
                (from..=to).all(|idx| self.hall[idx] == SpaceState::Empty)
            } else {
                (to..=from).all(|idx| self.hall[idx] == SpaceState::Empty)
            }
        }

        // Is the hall empty between indexes, EXCLUDING from.
        // Idea here is that if we're in the hallway we need to exclude our
        // current location, or we'll always think the way is blocked.
        fn hall_pathable(&self, from: usize, to: usize) -> bool {
            if from <= to {
                (from+1..=to).all(|idx| self.hall[idx] == SpaceState::Empty)
            } else {
                (to..=from-1).all(|idx| self.hall[idx] == SpaceState::Empty)
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
        // - Can only move if path out is open
        // - Never move if in destination and not in the way
        //   - Not in the way if I'm the only one (I should be in 0)
        //   - So equiv, all in room are in destination
        pub(crate) fn get_from_room(&self, room_idx: usize) -> Option<(Amphipod, usize)> {
            self.rooms[room_idx]
                .iter()
                .enumerate()
                .find_map(|(i, state)| match state {
                    &SpaceState::Empty => None,
                    &SpaceState::Occupied(amphipod) => 
                        Some((amphipod, i))
                })
                .filter(|(a, i)| {
                    // are all the amphipods in this room in their destination?
                    // (this would include the one under investigation)
                    !self.rooms[room_idx].iter().all(|&s| match s {
                        SpaceState::Occupied(a) => a.destination_room() == room_idx,
                        SpaceState::Empty => true,
                    })
                })
        }

        pub fn heuristic_cost(&self) -> u64 {
            let mut acc = 0u64;
            for (i, room) in self.rooms.iter().enumerate() {
                for (pos, state) in room.iter().enumerate() {
                    if let SpaceState::Occupied(amphipod) = state {
                        let curr_pos = BurrowState::room_hall_pos(i);
                        let dest_pos = BurrowState::room_hall_pos(amphipod.destination_room());
                        let should_move = !self.rooms[i].iter().all(|&s| match s {
                            SpaceState::Occupied(a) => a.destination_room() == i,
                            SpaceState::Empty => true,
                        });
                        if curr_pos != dest_pos || should_move {
                            let hallway_move = curr_pos.abs_diff(dest_pos) + 1;

                            acc += (hallway_move + pos*2) as u64 * amphipod.move_cost();
                        }
                    }
                }
            }

            for (i, state) in self.hall.iter().enumerate() {
                if let SpaceState::Occupied(amphipod) = state {
                    let curr_pos = i;
                    let dest_pos = BurrowState::room_hall_pos(amphipod.destination_room());
                    acc += (curr_pos.abs_diff(dest_pos) + 1) as u64 * amphipod.move_cost();
                }
            }

            acc
        }
    }

    #[cfg(test)]
    mod tests {
        use itertools::Itertools;

        use super::{*, parser::parse_input};

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

        #[test]
        fn test_get_from_room() {
        let example = r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
            "#;

        let after_move = r#"
#############
#.........D.#
###B#C#B#.###
  #A#D#C#A#
  #########
            "#;
        let dont_move_from_home = r#"
#############
#...B.......#
###.#C#B#D###
  #A#D#C#A#
  #########
            "#;

            assert_eq!(parser::parse_input(example).unwrap().get_from_room(3), Some((Amphipod::D, 0)));
            assert_eq!(parser::parse_input(after_move).unwrap().get_from_room(3), Some((Amphipod::A, 1)));
            assert_eq!(parser::parse_input(dont_move_from_home).unwrap().get_from_room(0), None);
        }

        #[test]
        fn test_heuristic_cost() {
let goal = r#"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
  
            "#;

            let example = r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
            "#;

            let moved = r#"
#############
#...B.......#
###.#C#B#D###
  #A#D#C#A#
  #########
            "#;

            assert_eq!(parser::parse_input(goal).unwrap().heuristic_cost(), 0);
            assert_eq!(parser::parse_input(example).unwrap().heuristic_cost(), 7369);
            assert_eq!(parser::parse_input(moved).unwrap().heuristic_cost(), 7359);
        }

        fn aux_test_successors_contains(input: &str, next: &str) -> usize {
            let input_state = parse_input(input).unwrap();
            let next_state = parse_input(next).unwrap();

            let successors = input_state.successors();
            assert!(input_state.successors().iter().find(|&(s, _)| *s == next_state).is_some());
            successors.len()
        }
        
        #[test]
        fn test_successor_examples() {
            // First state ripped out of test run, extrapolated to goal
            let sequence = vec![
r#"
#############
#D........D.#
###A#B#.#.###
  #A#B#C#C#
  #########
            "#,
r#"
#############
#D......C.D.#
###A#B#.#.###
  #A#B#C#.#
  #########
            "#,
r#"
#############
#D........D.#
###A#B#C#.###
  #A#B#C#.#
  #########
            "#,
r#"
#############
#D..........#
###A#B#C#.###
  #A#B#C#D#
  #########
            "#,
r#"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
            "#,

            ];

            let mut all_successors = 0;
            for (state, desired_next) in sequence.iter().tuple_windows() {
                all_successors += aux_test_successors_contains(state, desired_next);
            }
            println!("Added {} states", all_successors);
        }

        #[test]
        fn test_from_hallway_moves() {
            let example = r#"
#############
#...B.C.....#
###B#.#.#D###
  #A#D#C#A#
  #########
            "#;

            let optimal = r#"
#############
#...B.......#
###B#.#C#D###
  #A#D#C#A#
  #########
            "#;

            let moves = parser::parse_input(example).unwrap().from_hallway_moves();
            let optimal_state = parser::parse_input(optimal).unwrap();
            assert!(moves.iter().find(|(s, c)| (s, c) == (&optimal_state, &200)).is_some());
        }
    }
    mod parser {
        use super::*;

        fn parse_space(input: char) -> Option<SpaceState> {
            match input {
                'A' => Some(SpaceState::Occupied(Amphipod::A)),
                'B' => Some(SpaceState::Occupied(Amphipod::B)),
                'C' => Some(SpaceState::Occupied(Amphipod::C)),
                'D' => Some(SpaceState::Occupied(Amphipod::D)),
                '.' => Some(SpaceState::Empty),
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
                    hallway.push(parse_space(space)?);
                }
            }

            // 0 becomes rooms[0][0], 4 becomes rooms[0][1], and so on.
            let mut rooms_transposed: Vec<SpaceState> = vec![];
            for chr in (lines[2].to_owned() + lines[3]).chars() {
                if let Some(space) = parse_space(chr) {
                    rooms_transposed.push(space)
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

        pub fn mangle_to_part_two(state: &mut BurrowState) {
            // insert this
            //   #D#C#B#A#
            //   #D#B#A#C#

            state.rooms[0].insert(1, SpaceState::Occupied(Amphipod::D));
            state.rooms[0].insert(1, SpaceState::Occupied(Amphipod::D));

            state.rooms[1].insert(1, SpaceState::Occupied(Amphipod::B));
            state.rooms[1].insert(1, SpaceState::Occupied(Amphipod::C));
            
            state.rooms[2].insert(1, SpaceState::Occupied(Amphipod::A));
            state.rooms[2].insert(1, SpaceState::Occupied(Amphipod::B));

            state.rooms[3].insert(1, SpaceState::Occupied(Amphipod::C));
            state.rooms[3].insert(1, SpaceState::Occupied(Amphipod::A));
        }
    }

    fn search(initial: BurrowState) -> Option<(BurrowState, u64)> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut q = BinaryHeap::new();
        let mut dist: HashMap<BurrowState, u64> = HashMap::new();
        let mut counter: usize = 1;
        let mut last_counter: usize = 0;



        q.push(Reverse((initial.heuristic_cost(), 0, initial)));
        while let Some(Reverse((_, cost, next))) = q.pop() {
            if next.is_goal() {
                println!("Evaluted {} states at completion", counter);
                
                return Some((next, cost));
            }

            for (succ, cost_inc) in next.successors() {
                let new_cost = cost + cost_inc;
                if let Some(old_cost) = dist.get(&succ) {
                    if *old_cost > new_cost {
                        dist.insert(succ.clone(), new_cost);
                        q.retain(|Reverse((_, _, s))| *s != succ);
                        q.push(Reverse((succ.heuristic_cost() + new_cost, new_cost, succ)));
                    }
                } else {
                    dist.insert(succ.clone(), new_cost);
                    q.push(Reverse((succ.heuristic_cost() + new_cost, new_cost, succ)));
                }
                
                counter += 1;
                if counter - last_counter >= 100000 {
                    last_counter = counter;
                    println!("Evaluated 100k states, at {}, h(n) + g(n) = {}", counter, cost + next.heuristic_cost());
                    if let Some(Reverse((h, c, _))) = q.peek() {
                        println!("Next state h(n) + g(n) = {}", h);
                    }
                    println!("{}", next);
                }
            }
        }

        // no path found somehow
        println!("Evaluted {} states at failure", counter);
        return None;
    }

    pub mod part_one {
        use super::*;

        pub fn run(input: &str) -> Option<u64> {
            let initial = parser::parse_input(input)?;
            let (_, energy) = search(initial)?;
            Some(energy)
        }
    }

    pub mod part_two {
        use super::*;

        pub fn run(input: &str) -> Option<u64> {
            let mut initial = parser::parse_input(input)?;
            parser::mangle_to_part_two(&mut initial);
            println!("Initial\n {}", initial);
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

    #[test]
    fn test_part_two_example() {
        let example = r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
        "#;

        assert_eq!(puzzle::part_two::run(example), Some(44169))
    }
}
