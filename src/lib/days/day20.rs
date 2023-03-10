mod puzzle {
    use crate::util::grid::Grid;
    use itertools::Itertools;
    use std::fmt::Display;
    // build some kind of Grid<T> data structure, maybe back it by an nalgebra DMatrix under the hood.
    // Grid<T>::neighbors() -> Vec<(usize, usize)> giving us the 9 neighbors
    // index into the "image enhancement string" based on

    // since output is "infinite" we need to compute a larger area than input size
    // -3 to n + 3 (I think)
    #[derive(Debug, Clone, PartialEq)]
    pub enum PixelValue {
        Dark,
        Light,
    }
    impl PixelValue {
        pub fn is_lit(&self) -> bool {
            match self {
                PixelValue::Light => true,
                _ => false,
            }
        }

        pub fn parse(c: char) -> Option<PixelValue> {
            match c {
                '#' => Some(PixelValue::Light),
                '.' => Some(PixelValue::Dark),
                _ => None,
            }
        }

        pub fn parse_pattern(s: &str) -> Option<Vec<PixelValue>> {
            s.chars()
                .map(|c| PixelValue::parse(c))
                .collect::<Option<Vec<PixelValue>>>()
        }
    }
    impl Display for PixelValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            match self {
                PixelValue::Dark => write!(f, "."),
                PixelValue::Light => write!(f, "#"),
            }
        }
    }

    pub fn parse_input(input: &str) -> Option<(Vec<PixelValue>, Grid<PixelValue>)> {
        let lines = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect_vec();
        let enhancement = PixelValue::parse_pattern(&lines[0])?;
        let rows = lines[1..lines.len()]
            .iter()
            .map(|l| PixelValue::parse_pattern(l))
            .collect::<Option<Vec<Vec<PixelValue>>>>()?;

        let grid = Grid::from_rows(rows);

        Some((enhancement, grid))
    }

    pub fn pixel_pattern_to_index(pattern: &[PixelValue]) -> usize {
        let mut index = 0;
        for i in 0..pattern.len() {
            index = index << 1;
            if pattern[i] == PixelValue::Light {
                index = index | 1;
            }
        }
        index
    }

    pub fn pixel_pattern_at(
        image: &Grid<PixelValue>,
        default: &PixelValue,
        row: i32,
        col: i32,
    ) -> Vec<PixelValue> {
        let offsets = image.diag_offsets(row, col);
        offsets
            .iter()
            .map(|(r, c)| image.get_int(*r, *c).unwrap_or(default).clone())
            .collect_vec()
    }

    const DIM_INCREASE: usize = 2;
    const DIM_OFFSET: i32 = (DIM_INCREASE / 2) as i32;
    pub fn apply_enhancement(
        enhancement: &[PixelValue],
        image: Grid<PixelValue>,
        oob_value: &PixelValue,
    ) -> Grid<PixelValue> {
        let mut new_image = Grid::fill(
            image.height() + DIM_INCREASE,
            image.width() + DIM_INCREASE,
            PixelValue::Dark,
        );
        for col in -DIM_OFFSET..image.width() as i32 + DIM_OFFSET {
            for row in -DIM_OFFSET..image.width() as i32 + DIM_OFFSET {
                let pattern = pixel_pattern_at(&image, oob_value, row, col);
                let idx = pixel_pattern_to_index(&pattern);
                let r = (row + DIM_OFFSET) as usize;
                let c = (col + DIM_OFFSET) as usize;
                new_image[(r, c)] = enhancement[idx].clone();
            }
        }
        new_image
    }

    pub fn repeatedly_enhance(
        enhancement: &[PixelValue],
        image: Grid<PixelValue>,
        steps: usize,
    ) -> Grid<PixelValue> {
        let mut acc = apply_enhancement(enhancement, image, &PixelValue::Dark);
        for _i in 1..steps {
            let oob_value = if _i & 1 != 0 {
                &enhancement[0]
            } else {
                &PixelValue::Dark
            };
            acc = apply_enhancement(enhancement, acc, oob_value);
        }
        acc
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pixel_pattern_to_index() {
            let pattern = &PixelValue::parse_pattern("...#...#.").unwrap();
            println!("{:?}", pattern);
            assert_eq!(pixel_pattern_to_index(pattern), 0b000100010);
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let (enhancement, image) = puzzle::parse_input(input)?;
    let output = puzzle::repeatedly_enhance(&enhancement, image, 2);
    Some(output.iter().filter(|p| p.is_lit()).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (enhancement, image) = puzzle::parse_input(input)?;

    let output = puzzle::repeatedly_enhance(&enhancement, image, 50);

    //println!("in:\n{}", image);
    println!("out:\n{}", output);
    Some(output.iter().filter(|p| p.is_lit()).count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &'static str = include_str!("examples/day20-full.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one(EXAMPLE), Some(35));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two(EXAMPLE), Some(3351));
    }
}
