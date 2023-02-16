mod puzzle {
    #[derive(Debug, PartialEq, Clone)]
    pub struct Vec2 {
        pub x: i32,
        pub y: i32,
    }
    impl Vec2 {
        fn plus(&mut self, other: &Vec2) {
            self.x = self.x + other.x;
            self.y = self.y + other.y;
        }
    }
    #[derive(Debug)]
    pub struct Probe {
        velocity: Vec2,
        position: Vec2,
    }
    impl Probe {
        pub fn with_velocity(velocity: Vec2) -> Probe {
            Probe {
                velocity: velocity,
                position: Vec2 { x: 0, y: 0 },
            }
        }
        pub fn step(&mut self) {
            self.position.plus(&self.velocity);
            self.velocity.x = if self.velocity.x > 0 {
                self.velocity.x - 1
            } else if self.velocity.x < 0 {
                self.velocity.x + 1
            } else {
                0
            };
            self.velocity.y = self.velocity.y - 1;
        }

        #[cfg(test)]
        pub fn hits_target(&mut self, target: &Target) -> bool {
            self.max_height(target).is_some()
        }

        pub fn max_height(&mut self, target: &Target) -> Option<u64> {
            let mut max_y = self.position.y;
            loop {
                max_y = std::cmp::max(max_y, self.position.y);

                if self.position.y < target.bottom_left.y {
                    return None;
                }
                if target.contains_point(&self.position) {
                    return Some(max_y as u64);
                }
                self.step();
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Target {
        pub bottom_left: Vec2,
        pub top_right: Vec2,
    }
    impl Target {
        fn contains_point(&self, point: &Vec2) -> bool {
            point.x >= self.bottom_left.x
                && point.y >= self.bottom_left.y
                && point.x <= self.top_right.x
                && point.y <= self.top_right.y
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_target_contains() {
            let target = Target {
                bottom_left: Vec2 { x: 20, y: -10 },
                top_right: Vec2 { x: 30, y: -5 },
            };
            assert!(target.contains_point(&Vec2 { x: 27, y: -8 }));
            assert!(target.contains_point(&Vec2 { x: 20, y: -10 }));
            assert!(target.contains_point(&Vec2 { x: 20, y: -5 }));
            assert!(target.contains_point(&Vec2 { x: 30, y: -5 }));
            assert!(target.contains_point(&Vec2 { x: 30, y: -10 }));
            assert!(!target.contains_point(&Vec2 { x: 0, y: 0 }));
            assert!(!target.contains_point(&Vec2 { x: 33, y: -9 }));
        }

        #[test]
        fn test_probe_hits_target() {
            let target = Target {
                bottom_left: Vec2 { x: 20, y: -10 },
                top_right: Vec2 { x: 30, y: -5 },
            };
            assert!(Probe::with_velocity(Vec2 { x: 7, y: 2 }).hits_target(&target));
            assert!(Probe::with_velocity(Vec2 { x: 6, y: 3 }).hits_target(&target));
            assert!(Probe::with_velocity(Vec2 { x: 9, y: 0 }).hits_target(&target));
            assert!(!Probe::with_velocity(Vec2 { x: 17, y: -4 }).hits_target(&target));
            assert!(Probe::with_velocity(Vec2 { x: 6, y: 9 }).hits_target(&target)); //maximum height
            assert!(!Probe::with_velocity(Vec2 { x: 6, y: 10 }).hits_target(&target));
        }

        #[test]
        fn test_probe_max_height() {
            let target = Target {
                bottom_left: Vec2 { x: 20, y: -10 },
                top_right: Vec2 { x: 30, y: -5 },
            };
            assert_eq!(
                Probe::with_velocity(Vec2 { x: 6, y: 9 }).max_height(&target),
                Some(45)
            ); //maximum height
        }
    }

    mod parser {
        use super::*;
        use nom::{
            bytes::complete::tag,
            character::complete::multispace0,
            combinator::fail,
            sequence::{delimited, preceded, terminated},
            IResult,
        };
        fn range_pair(input: &str) -> IResult<&str, (i32, i32)> {
            let (input, first) = nom::character::complete::i32(input)?;
            let (input, _) = tag("..")(input)?;
            let (input, second) = nom::character::complete::i32(input)?;

            Ok((input, (first, second)))
        }

        pub fn input_parser(input: &str) -> IResult<&str, Target> {
            let (input, _) = terminated(tag("target area:"), multispace0)(input)?;
            let (input, xs) =
                delimited(tag("x="), range_pair, terminated(tag(","), multispace0))(input)?;
            let (input, ys) = preceded(tag("y="), range_pair)(input)?;

            let (min_x, max_x) = match xs {
                (a, b) if a < b => (a, b),
                (a, b) if a > b => (b, a),
                _ => return fail(input),
            };
            let (min_y, max_y) = match ys {
                (a, b) if a < b => (a, b),
                (a, b) if a > b => (b, a),
                _ => return fail(input),
            };

            Ok((
                input,
                Target {
                    bottom_left: Vec2 { x: min_x, y: min_y },
                    top_right: Vec2 { x: max_x, y: max_y },
                },
            ))
        }
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_parse_targets() {
                assert_eq!(
                    input_parser("target area: x=20..30, y=-10..-5").unwrap().1,
                    Target {
                        bottom_left: Vec2 { x: 20, y: -10 },
                        top_right: Vec2 { x: 30, y: -5 }
                    }
                );
                assert_eq!(
                    input_parser("target area: x=236..262, y=-78..-57")
                        .unwrap()
                        .1,
                    Target {
                        bottom_left: Vec2 { x: 236, y: -78 },
                        top_right: Vec2 { x: 262, y: -57 }
                    }
                );
            }
        }
    }

    pub fn parse_input(input: &str) -> Option<Target> {
        parser::input_parser(input).ok().map(|t| t.1)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let target = puzzle::parse_input(input)?;
    let mut max_height: u64 = 0;
    for x in 1..(target.top_right.x) {
        for y in 0..100 {
            let velocity = puzzle::Vec2 { x: x, y: y };
            let mut probe = puzzle::Probe::with_velocity(velocity.clone());
            if let Some(height) = probe.max_height(&target) {
                println!("{:?} -> {}", velocity, height);
                max_height = std::cmp::max(max_height, height);
            }
        }
    }
    Some(max_height)
}

pub fn part_two(input: &str) -> Option<u64> {
    let target = puzzle::parse_input(input)?;
    let mut count: u64 = 0;
    for x in 1..(target.top_right.x + 10) {
        for y in -100..100 {
            let velocity = puzzle::Vec2 { x: x, y: y };
            let mut probe = puzzle::Probe::with_velocity(velocity.clone());
            if let Some(_height) = probe.max_height(&target) {
                count = count + 1
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one("target area: x=20..30, y=-10..-5"), Some(45));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two("target area: x=20..30, y=-10..-5"), Some(112));
    }
}
