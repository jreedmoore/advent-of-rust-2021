// Day 22 is a spatial problem, we're trying to count the number of cubes
// enabled given a set of instructions to turn on and off regions of cubes

// Part 1 restricts the problem to a set of a million cubes, so no big deal to
// fit a naive dense representation in memory

// The puzzle input contains a bunch of instructions outside of the bounds of
// Part 1 so we will definitely need a sparse representation and some way to
// efficiently query it.

pub mod puzzle {
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

        fn is_overlapping(&self, other: &CoordRange) -> bool {
            (self.low < other.low && self.high > other.low)
                || (self.low < other.high && self.high > other.high)
                || (other.low < self.low && other.high > self.low)
                || (other.low < self.high && other.high > self.high)
        }

        fn contains(&self, other: &CoordRange) -> bool {
            self.low <= other.low && self.high >= other.high
        }

        fn size(&self) -> u64 {
            (self.high - self.low) as u64 + 1
        }
    }
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub(super) struct BoundingBox {
        x: CoordRange,
        y: CoordRange,
        z: CoordRange,
    }
    impl BoundingBox {
        pub fn new(
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

        // project the problem into 2D for test cases
        pub fn on_z(x_low: i64, x_high: i64, y_low: i64, y_high: i64) -> BoundingBox {
            BoundingBox {
                x: CoordRange::new(x_low, x_high),
                y: CoordRange::new(y_low, y_high),
                z: CoordRange::new(1, 1),
            }
        }

        pub fn is_overlapping(&self, other: &BoundingBox) -> bool {
            self.x.is_overlapping(&other.x)
                && self.y.is_overlapping(&other.y)
                && self.z.is_overlapping(&other.z)
        }

        pub fn size(&self) -> u64 {
            self.x.size() * self.y.size() * self.z.size()
        }

        // Produce vec of bounding boxes which do NOT contain other.
        fn subdivide_on(&self, other: &BoundingBox) -> Vec<BoundingBox> {
            let mut acc = vec![];

            if self.x.contains(&other.x) {
                acc.push(BoundingBox { x: CoordRange::new(other.x.high + 1, self.x.high), y: self.y.clone(), z: self.z.clone() });
            }
            acc
        }
    }
    #[cfg(test)]
    mod tests {
        use super::BoundingBox;

        fn subdivided_size(target: &BoundingBox, other: &BoundingBox) -> u64 {
            target.subdivide_on(other).iter().map(|bb|bb.size()).sum()
        }

        #[test]
        fn test_size() {
            assert_eq!(BoundingBox::on_z(1,3,1,3).size(), 9);
            assert_eq!(BoundingBox::on_z(2,3,1,3).size(), 6);
            assert_eq!(BoundingBox::on_z(1,1,1,1).size(), 1);
        }

        #[test]
        fn test_subivide_on() {
            let three_by_three = BoundingBox::on_z(1,3,1,3);
            let left = BoundingBox::on_z(1,1,1,3);
            let right = BoundingBox::on_z(3,3,1,3);
            let top = BoundingBox::on_z(1,3,3,3);
            let bottom = BoundingBox::on_z(1,3,1,1);

            let center = BoundingBox::on_z(2,2,2,2);

            let top_left = BoundingBox::on_z(1,1,3,3);
            let top_right = BoundingBox::on_z(3,3,3,3);
            let bottom_left = BoundingBox::on_z(1,1,1,1);
            let bottom_right = BoundingBox::on_z(3,3,1,1);

            let left_center = BoundingBox::on_z(1,1,2,2);
            let right_center = BoundingBox::on_z(3,3,2,2);
            let bottom_center = BoundingBox::on_z(2,2,1,1);
            let top_center = BoundingBox::on_z(2,2,3,3);

            assert_eq!(subdivided_size(&three_by_three, &left), 6);
            assert_eq!(subdivided_size(&three_by_three, &right), 6);
            assert_eq!(subdivided_size(&three_by_three, &top), 6);
            assert_eq!(subdivided_size(&three_by_three, &bottom), 6);

            assert_eq!(subdivided_size(&three_by_three, &center), 8);

            assert_eq!(subdivided_size(&three_by_three, &top_left), 8);
            assert_eq!(subdivided_size(&three_by_three, &top_right), 8);
            assert_eq!(subdivided_size(&three_by_three, &bottom_left), 8);
            assert_eq!(subdivided_size(&three_by_three, &bottom_right), 8);
            
            assert_eq!(subdivided_size(&three_by_three, &left_center), 8);
            assert_eq!(subdivided_size(&three_by_three, &right_center), 8);
            assert_eq!(subdivided_size(&three_by_three, &bottom_center), 8);
            assert_eq!(subdivided_size(&three_by_three, &top_center), 8);
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    enum CommandState {
        ON,
        OFF,
    }
    #[derive(Debug)]
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

    // operate over a restricted set region, so we take a naive approach and represent the reactor as a dense matrix, and directly apply the commands on to that region
    pub mod part_one {
        use itertools::Itertools;

        use super::*;

        #[derive(Clone, Copy, PartialEq)]
        enum CubeState {
            ON,
            OFF,
        }
        struct DenseCubes {
            cubes: Vec<CubeState>,
        }
        impl DenseCubes {
            fn in_bounds(v: i64) -> bool {
                v <= 50 && v >= -50
            }
            fn index(x: i64, y: i64, z: i64) -> Option<usize> {
                if Self::in_bounds(x) && Self::in_bounds(y) && Self::in_bounds(z) {
                    let xx = (x + 50) as usize;
                    let yy = (y + 50) as usize;
                    let zz = (z + 50) as usize;
                    Some(xx + yy * 100 + zz * 100 * 100)
                } else {
                    None
                }
            }
            fn set(&mut self, x: i64, y: i64, z: i64, v: CubeState) -> bool {
                Self::index(x, y, z)
                    .map(|idx| self.cubes[idx] = v)
                    .is_some()
            }
            fn new() -> DenseCubes {
                DenseCubes {
                    cubes: vec![CubeState::OFF; 1000000],
                }
            }
        }
        pub fn run(input: &str) -> Option<u64> {
            let commands = parser::parse_input(input)?;
            let reactor_bbox = BoundingBox::new(-50, 50, -50, 50, -50, 50);
            let mut reactor = DenseCubes::new();

            println!("Command in {:?}", commands[0]);
            //let filtered_commands = commands.into_iter().filter(|cmd| cmd.bbox.is_overlapping(&reactor_bbox)).collect_vec();

            for command in commands {
                if command.bbox.is_overlapping(&reactor_bbox) {
                    for x in command.bbox.x.low..=command.bbox.x.high {
                        for y in command.bbox.y.low..=command.bbox.y.high {
                            for z in command.bbox.z.low..=command.bbox.z.high {
                                let cube_state = match command.state {
                                    CommandState::ON => CubeState::ON,
                                    CommandState::OFF => CubeState::OFF,
                                };
                                reactor.set(x, y, z, cube_state);
                            }
                        }
                    }
                }
            }

            Some(
                reactor
                    .cubes
                    .iter()
                    .filter(|s| **s == CubeState::ON)
                    .count() as u64,
            )
        }
    }

    // Part Two allows a much larger range for the reactor region, which isn't feasible to naively store in memory, and further the runtime of naively executing operations is way too large
    // instead we'll directly represent the on regions of the instructions as bounding boxes in 3D space and then subdivide those regions if subsequent on or off instructions overlap them.
    pub mod part_two {
        use std::collections::HashSet;

        use super::*;
        pub fn run(input: &str) -> Option<u64> {
            let commands = parser::parse_input(input)?;
            
            let mut on_regions: HashSet<BoundingBox> = HashSet::new();
            for command in commands {
                let command_region = command.bbox.clone();

                // this will be quadratic slow, but that's okay for now
                let overlapping: Vec<BoundingBox> = on_regions.drain_filter(|existing| existing.is_overlapping(&command_region)).collect();
                for overlapping_region in overlapping {
                    for new_region in overlapping_region.subdivide_on(&command_region) {
                        on_regions.insert(new_region);
                    }
                }

                if command.state == CommandState::ON {
                    on_regions.insert(command_region);
                }
            }

            Some(on_regions.iter().map(|region| region.size()).sum())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small_example() {
        let example = r#"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
        "#;

        assert_eq!(puzzle::part_one::run(example), Some(39))
    }
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

    fn test_part_two_example() {
        let example = r#"
on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
        "#;

        assert_eq!(puzzle::part_one::run(example), Some(474140));
        assert_eq!(puzzle::part_two::run(example), Some(2758514936282235))
    }

    #[test]
    fn test_bounding_box_overlap() {
        use super::puzzle::BoundingBox;
        let containing = BoundingBox::new(-50, 50, -50, 50, -50, 50);
        let smaller = BoundingBox::new(10, 12, 10, 12, 10, 12);

        assert!(containing.is_overlapping(&smaller));
        assert!(smaller.is_overlapping(&containing));
    }
}
