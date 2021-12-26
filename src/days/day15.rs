mod puzzle {
  pub struct CaveGraph;
  impl CaveGraph {
    pub fn top_left(&self) -> CaveLocation {
      todo!()
    }
    pub fn bottom_right(&self) -> CaveLocation {
      todo!()
    }
  }
  pub struct CaveLocation;

  pub fn lowest_cost_between(graph : CaveGraph, start: CaveLocation, end: CaveLocation) -> Option<u64> {
    todo!()
  }

  pub fn parse_input(input: &str) -> Option<CaveGraph> {
    todo!()
  }
}
pub fn part_one(input: &str) -> Option<u64> {
  use puzzle::*;

  let cave = parse_input(input)?;
  let top_left = cave.top_left();
  let bottom_right = cave.bottom_right();

  lowest_cost_between(cave, top_left, bottom_right)
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &'static str = r#"
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
  "#;
  #[test]
  fn test_example_part_one() {
    assert_eq!(part_one(EXAMPLE), Some(40));
  }
}