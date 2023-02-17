// Day 22 is a spatial problem, we're trying to count the number of cubes
// enabled given a set of instructions to turn on and off regions of cubes

// Part 1 restricts the problem to a set of a million cubes, so no big deal to
// fit a naive dense representation in memory

// The puzzle input contains a bunch of instructions outside of the bounds of
// Part 1 so we will definitely need a sparse representation and some way to
// efficiently query it.

pub mod puzzle {
    struct CoordRange {
        low: i64,
        high: i64,
    }
    impl CoordRange {
        fn new(low: i64, high: i64) -> CoordRange {
            if low > high {
                panic!("Invalid CoordRange, low > high; {} > {}", low, high)
            }
            CoordRange {
                low: low,
                high: high,
            }
        }
    }
    struct BoundingBox {
        x: CoordRange,
        y: CoordRange,
        z: CoordRange,
    }
    impl BoundingBox {
        fn new(
            x_low: i64,
            x_high: i64,
            y_low: i64,
            y_high: i64,
            z_low: i64,
            z_high: i64,
        ) -> BoundingBox {
            BoundingBox {
                x: CoordRange::new(x_low, x_high),
                y: CoordRange::new(y_low, y_high),
                z: CoordRange::new(z_low, z_high),
            }
        }
    }

    #[derive(Clone)]
    enum CommandState {
        ON,
        OFF,
    }
    struct Command {
        state: CommandState,
        bbox: BoundingBox,
    }
    mod parser {
        use super::*;

        use crate::util::nom_helpers::ws;
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::alpha1,
            combinator::{map, value},
            multi::{many1, separated_list1},
            sequence::{preceded, separated_pair, tuple},
            IResult,
        };

        fn state(input: &str) -> IResult<&str, CommandState> {
            ws(alt((
                value(CommandState::ON, tag("on")),
                value(CommandState::OFF, tag("off")),
            )))(input)
        }

        fn range(input: &str) -> IResult<&str, CoordRange> {
            map(
                preceded(
                    preceded(alpha1, tag("=")),
                    separated_pair(
                        nom::character::complete::i64,
                        tag(".."),
                        nom::character::complete::i64,
                    ),
                ),
                |(low, high)| CoordRange::new(low, high),
            )(input)
        }

        fn command(input: &str) -> IResult<&str, Command> {
            map(
                tuple((state, separated_list1(tag(","), range))),
                |(state, mut ranges)| {
                    let z = ranges.remove(2);
                    let y = ranges.remove(1);
                    let x = ranges.remove(0);
                    Command {
                        state: state,
                        bbox: BoundingBox { x: x, y: y, z: z },
                    }
                },
            )(input)
        }

        pub(super) fn parse_input(input: &str) -> Option<Vec<Command>> {
            Some(many1(command)(input).ok()?.1)
        }
    }

    pub mod part_one {
        use super::*;
        pub fn run(input: &str) -> Option<u64> {
            let commands = parser::parse_input(input)?;

            Some(0)
        }
    }

    pub mod part_two {
        pub fn run(input: &str) -> Option<u64> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let example = r#"
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
        "#;

        assert_eq!(puzzle::part_one::run(example), Some(590784))
    }
}
