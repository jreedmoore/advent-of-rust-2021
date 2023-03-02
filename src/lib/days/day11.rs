mod puzzle {
    //use std::fmt::{Debug, Formatter};

    #[derive(Debug)]
    pub struct Octopus {
        pub energy: u8,
        pub flashed: bool,
    }

    #[derive(Debug)]
    pub struct Cave {
        octopi: Vec<Octopus>,
        pub width: u8,
        pub flashes: u64,
    }
    impl Cave {
        pub fn height(&self) -> u8 {
            (self.octopi.len() / self.width as usize) as u8
        }
        pub fn get_at(&mut self, row: u8, col: u8) -> &mut Octopus {
            let idx = (row * self.width) + col;
            &mut self.octopi[idx as usize]
        }

        pub fn get_neighbors(&self, row: u8, col: u8) -> Vec<(u8, u8)> {
            let mut res = vec![];
            let ir = row as i16;
            let ic = col as i16;
            for r in ir - 1..ir + 2 {
                for c in ic - 1..ic + 2 {
                    if r >= 0 && r < self.width.into() && c >= 0 && c < self.height().into() {
                        if (r, c) != (row.into(), col.into()) {
                            res.push((r as u8, c as u8));
                        }
                    }
                }
            }
            res
        }

        pub fn step(&mut self) {
            for octopus in self.octopi.iter_mut() {
                octopus.energy += 1;
            }

            let mut cont = true;
            while cont {
                cont = false;
                for col in 0..self.width {
                    for row in 0..self.height() {
                        let octopus = self.get_at(row, col);
                        if octopus.energy > 9 && !octopus.flashed {
                            octopus.flashed = true;
                            self.flashes += 1;
                            for (r, c) in self.get_neighbors(row, col) {
                                let mut neighbor = self.get_at(r, c);
                                neighbor.energy += 1;
                            }
                            cont = true;
                        }
                    }
                }
            }

            for octopus in self.octopi.iter_mut() {
                if octopus.flashed {
                    octopus.flashed = false;
                    octopus.energy = 0;
                }
            }
        }

        pub fn all_zero(&self) -> bool {
            self.octopi.iter().all(|o| o.energy == 0)
        }

        #[allow(dead_code)]
        pub fn pretty_print(&mut self) -> String {
            (0..self.height())
                .map(|row| {
                    (0..self.width)
                        .map(|col| self.get_at(row as u8, col as u8).energy.to_string())
                        .collect()
                })
                .collect::<Vec<String>>()
                .join("\n")
        }
    }

    pub fn parse_input(input: &str) -> Option<Cave> {
        let lines = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| x.len() != 0)
            .collect::<Vec<&str>>();
        if lines.len() == 0 {
            None
        } else {
            let width = lines[0].len() as u8;
            let energy: Option<Vec<u8>> = lines
                .iter()
                .flat_map(|l| l.chars())
                .map(|c| c.to_string().parse::<u8>().ok())
                .collect();
            energy.map(|es| {
                let octopi: Vec<Octopus> = es
                    .iter()
                    .map(|e| Octopus {
                        energy: *e,
                        flashed: false,
                    })
                    .collect();
                Cave {
                    octopi: octopi,
                    width: width,
                    flashes: 0,
                }
            })
        }
    }
}

pub fn step_cave(input: &str, steps: usize) -> Option<u64> {
    let mut cave = puzzle::parse_input(input)?;
    //println!("staring config\n{}", cave.pretty_print());
    for _n in 0..steps {
        cave.step();
        //println!("step {}\n{}", n+1, cave.pretty_print());
    }
    return Some(cave.flashes.clone());
}

pub fn step_until_synchronized(cave: &mut puzzle::Cave) -> u64 {
    let mut n: u64 = 0;
    loop {
        cave.step();
        n += 1;
        if cave.all_zero() {
            break;
        }
    }
    n
}

pub fn part_one(input: &str) -> Option<u64> {
    step_cave(input, 100)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(step_until_synchronized(&mut puzzle::parse_input(input)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_EXAMPLE: &'static str = r#"
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
  "#;
    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one(FULL_EXAMPLE), Some(1656));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two(FULL_EXAMPLE), Some(195));
    }

    #[test]
    fn test_small_example() {
        let example = r#"
      11111
      19991
      19191
      19991
      11111
    "#;
        assert_eq!(step_cave(example, 2), Some(9));
    }

    #[test]
    fn test_neighbors() {
        let example = r#"
      11111
      19991
      19191
      19991
      11111
    "#;
        let mut cave = puzzle::parse_input(example).unwrap();
        // I think I should be able to add a test module next to the cave impl rather than doing this
        assert_eq!(cave.width, 5);
        assert_eq!(cave.height(), 5);
        assert_eq!(cave.get_at(0, 0).energy, 1);
        assert_eq!(cave.get_at(2, 2).energy, 1);
        assert_eq!(
            cave.get_neighbors(2, 2),
            vec![
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3)
            ]
        );
        assert_eq!(cave.get_neighbors(0, 0), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(cave.get_neighbors(4, 4), vec![(3, 3), (3, 4), (4, 3)]);
    }
}
