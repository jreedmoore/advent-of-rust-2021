mod puzzle {
    use std::collections::HashSet;

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct Point {
        pub x: u32,
        pub y: u32,
    }
    impl Point {
        fn parse(input: &str) -> Option<Point> {
            let xs: Vec<u32> = input
                .split(",")
                .map(|s| s.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()?;
            Some(Point { x: xs[0], y: xs[1] })
        }

        fn reflect_x(&mut self, x: u32) {
            let diff = self.x - x;
            self.x = x - diff;
        }

        fn reflect_y(&mut self, y: u32) {
            let diff = self.y - y;
            self.y = y - diff;
        }
    }

    pub struct OrigamiPaper {
        points: HashSet<Point>,
    }
    impl OrigamiPaper {
        fn parse_points(input: &[&str]) -> Option<OrigamiPaper> {
            let points: HashSet<Point> = input
                .iter()
                .map(|s| Point::parse(s))
                .collect::<Option<HashSet<Point>>>()?;
            Some(OrigamiPaper { points: points })
        }

        pub fn fold(&mut self, direction: &FoldDirection) {
            match *direction {
                FoldDirection::AlongX(x) => {
                    let to_fold: Vec<_> = self.points.drain_filter(|p| p.x > x).collect();
                    for mut p in to_fold {
                        p.reflect_x(x);
                        self.points.insert(p);
                    }
                }
                FoldDirection::AlongY(y) => {
                    let to_fold: Vec<_> = self.points.drain_filter(|p| p.y > y).collect();
                    for mut p in to_fold {
                        p.reflect_y(y);
                        self.points.insert(p);
                    }
                }
            }
        }

        pub fn count(&self) -> u64 {
            self.points.len() as u64
        }

        pub fn pretty_print(&self) -> String {
            let max_x = self.points.iter().map(|p| p.x).max().unwrap_or(0);
            let max_y = self.points.iter().map(|p| p.y).max().unwrap_or(0);
            let mut p = Point { x: 0, y: 0 };
            (0..max_y + 1)
                .map(|y| {
                    (0..max_x + 1)
                        .map(|x| {
                            p.x = x;
                            p.y = y;
                            if self.points.contains(&p) {
                                "#"
                            } else {
                                "."
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        }
    }

    pub enum FoldDirection {
        AlongX(u32),
        AlongY(u32),
    }
    impl FoldDirection {
        fn parse_direction(input: &str) -> Option<FoldDirection> {
            let parts: Vec<_> = input.split_whitespace().collect();
            let instructions: Vec<_> = parts[2].split("=").collect();
            let number = instructions[1].parse::<u32>().ok()?;
            match instructions[0] {
                "x" => Some(FoldDirection::AlongX(number)),
                "y" => Some(FoldDirection::AlongY(number)),
                _ => None,
            }
        }
        fn parse_directions(inputs: &[&str]) -> Option<Vec<FoldDirection>> {
            inputs
                .iter()
                .map(|s| FoldDirection::parse_direction(s))
                .collect()
        }
    }

    pub fn parse_input(input: &str) -> Option<(OrigamiPaper, Vec<FoldDirection>)> {
        let (points, directions): (Vec<&str>, Vec<&str>) = input
            .lines()
            .map(|s| s.trim())
            .filter(|s| s.len() != 0)
            .partition(|s| !s.contains("fold"));
        Some((
            OrigamiPaper::parse_points(&points)?,
            FoldDirection::parse_directions(&directions)?,
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_reflection() {
            let mut p = Point { x: 3, y: 0 };
            p.reflect_x(2);
            assert_eq!(p.x, 1);

            let mut p2 = Point { x: 0, y: 3 };
            p2.reflect_y(2);
            assert_eq!(p2.y, 1);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut paper, directions) = puzzle::parse_input(input)?;
    for direction in directions.iter().take(1) {
        paper.fold(direction);
    }
    Some(paper.count())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut paper, directions) = puzzle::parse_input(input)?;
    for direction in directions {
        paper.fold(&direction);
    }
    println!("{}", paper.pretty_print());
    Some(paper.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
  "#;

    #[test]
    fn test_part_one_examples() {
        assert_eq!(part_one(EXAMPLE), Some(17))
    }
}
