mod puzzle {
  use std::cmp::Ordering;
  use std::collections::{BinaryHeap, HashMap};

  #[derive(Debug)]
  pub struct CaveGraph {
    properties: Vec<LocationProperties>,
    width: usize,
    height: usize
  }
  impl CaveGraph {
    pub fn top_left(&self) -> CaveLocation {
      CaveLocation { row: 0, column: 0 }
    }
    pub fn bottom_right(&self) -> CaveLocation {
      CaveLocation { row: self.width-1, column: self.height-1 }
    }
    pub fn get_neighbors(&self, of: CaveLocation) -> Vec<CaveLocation> {
      let ir = of.row as i64;
      let ic = of.column as i64;
      // TODO: is there a less messy way to do my bounds checking?
      vec![
        (ir-1,ic)
      , (ir,ic-1)
      , (ir+1,ic)
      , (ir,ic+1)
      ].iter()
       .filter(|(r,c)| *r >= 0 && *c >= 0 && *r < self.height.try_into().unwrap() && *c < self.width.try_into().unwrap())
       .map(|(r,c)| CaveLocation { row: *r as usize, column: *c as usize })
       .collect()
    }
    pub fn len(&self) -> usize {
      self.properties.len()
    }
    pub fn get(&self, at: CaveLocation) -> &LocationProperties {
      &self.properties[self.idx(at)]
    }
    pub fn idx(&self, of: CaveLocation) -> usize {
      of.column + of.row * self.width
    }
    pub fn parse(input: &str) -> Option<CaveGraph> {
      let lines = 
        input.lines()
          .map(|x| x.trim())
          .filter(|x| x.len() != 0)
          .collect::<Vec<&str>>();
      if lines.len() == 0 {
        None
      } else {
        let width = lines[0].len();
        let risk_level: Option<Vec<u64>> = 
          lines.iter()
            .flat_map(|l| l.chars())
            .map(|c| c.to_string().parse::<u64>().ok())
            .collect();
        risk_level.map (|rl| {
          let properties: Vec<LocationProperties> = rl.iter().map(|r| LocationProperties { risk_level: *r }).collect();
          let height = properties.len() / width;
          CaveGraph { properties: properties, width: width, height: height }
        })
      }
    }

    pub fn add_and_wrap(level: u64, plus: u64) -> u64 {
      let wrapped_risk = level + plus;
      let new_risk = if wrapped_risk > 9 { wrapped_risk - 9 } else { wrapped_risk };
      new_risk
    }

    pub fn parse_and_expand(input: &str) -> Option<CaveGraph> {
      let lines = 
        input.lines()
          .map(|x| x.trim())
          .filter(|x| x.len() != 0)
          .collect::<Vec<&str>>();
      if lines.len() == 0 {
        None
      } else {
        let width = lines[0].len() * 5;
        let height = lines.len() * 5;
        let risk_level_lines = 
          lines.iter()
            .map(|l| l.chars().map(|c| c.to_string().parse::<u64>().ok()).collect())
            .collect::<Option<Vec<Vec<u64>>>>()?;
        let mut risk_levels: Vec<u64> = Vec::with_capacity(height * width);
        for row_ex in 0..5 {
          risk_level_lines.iter().for_each(|line| {
            for col_ex in 0..5 {
              line.iter().for_each(|r| risk_levels.push(CaveGraph::add_and_wrap(*r, row_ex + col_ex)))
            }
          })
        }
        let properties: Vec<LocationProperties> = risk_levels.iter().map(|r| LocationProperties { risk_level: *r }).collect();
        Some(CaveGraph { properties: properties, width: width, height: height })
      }
    }
  }

  pub struct ExpandedCaveGraph {
    inner: CaveGraph
  }
  impl ExpandedCaveGraph {
    pub fn width(&self) -> usize {
      self.inner.width * 5
    }
    pub fn height(&self) -> usize {
      self.inner.height * 5
    }
    pub fn top_left(&self) -> CaveLocation {
      CaveLocation { row: 0, column: 0 }
    }
    pub fn bottom_right(&self) -> CaveLocation {
      CaveLocation { row: self.width()-1, column: self.height()-1 }
    }
    pub fn get_neighbors(&self, of: CaveLocation) -> Vec<CaveLocation> {
      let ir = of.row as i64;
      let ic = of.column as i64;
      // TODO: is there a less messy way to do my bounds checking?
      vec![
        (ir-1,ic)
      , (ir,ic-1)
      , (ir+1,ic)
      , (ir,ic+1)
      ].iter()
       .filter(|(r,c)| *r >= 0 && *c >= 0 && *r < self.height().try_into().unwrap() && *c < self.width().try_into().unwrap())
       .map(|(r,c)| CaveLocation { row: *r as usize, column: *c as usize })
       .collect()
    }
    pub fn len(&self) -> usize {
      self.inner.properties.len()
    }
    pub fn get(&self, at: CaveLocation) -> LocationProperties {
      let base_loc = CaveLocation { row: at.row % self.inner.width, column: at.column % self.inner.height };
      let base_idx = self.inner.idx(base_loc);
      let base_prop = &self.inner.properties[base_idx];
      
      let row_dup = (at.row / self.inner.width) as u64;
      let col_dup = (at.column / self.inner.height) as u64;

      let new_risk = CaveGraph::add_and_wrap(base_prop.risk_level, row_dup + col_dup);

      LocationProperties { risk_level: new_risk }
    }
    pub fn parse(input: &str) -> Option<ExpandedCaveGraph> {
      Some(ExpandedCaveGraph { inner: CaveGraph::parse(input)? })
    }
  }

  #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
  pub struct CaveLocation {
    pub row: usize,
    pub column: usize
  }
  #[derive(Debug)]
  pub struct LocationProperties {
    pub risk_level: u64
  }


  #[derive(Copy, Clone, Eq, PartialEq, Debug)]
  struct State {
      cost: u64,
      position: CaveLocation,
  }

  impl Ord for State {
      fn cmp(&self, other: &Self) -> Ordering {
          // Notice that the we flip the ordering on costs. (turn max-heap into min-heap)
          // In case of a tie we compare positions - this step is necessary
          // to make implementations of `PartialEq` and `Ord` consistent.
          other.cost.cmp(&self.cost)
              .then_with(|| self.position.cmp(&other.position))
      }
  }

  // `PartialOrd` needs to be implemented as well.
  impl PartialOrd for State {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
          Some(self.cmp(other))
      }
  }

  pub fn sparse_lowest_cost_between<F: Fn(&LocationProperties) -> u64>(graph : ExpandedCaveGraph, cost_fun : F, start: CaveLocation, end: CaveLocation) -> Option<u64> {
    let mut dist: HashMap<CaveLocation, u64> = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
      // println!("{:?} {:?} {:?}", cost, dist.get(&position), position);
      if position == end { return Some(cost); }

      // ignore more expensive path (if dist relaxed)
      if cost > *dist.get(&position).unwrap_or(&u64::MAX) { continue; }

      for neighbor in graph.get_neighbors(position) {
        let next = State { cost: cost + cost_fun(&graph.get(neighbor)), position: neighbor };

        if next.cost < *dist.get(&next.position).unwrap_or(&u64::MAX) {
          heap.push(next);
          dist.insert(next.position, next.cost);
        }
      }
    }

    None
  }
  pub fn lowest_cost_between<F: Fn(&LocationProperties) -> u64>(graph : CaveGraph, cost_fun : F, start: CaveLocation, end: CaveLocation) -> Option<u64> {
    let mut dist: Vec<u64> = (0..graph.len()).map(|_| u64::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[graph.idx(start)] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
      // println!("{:?} {:?} {:?}", cost, dist[graph.idx(position)], position);
      if position == end { return Some(cost); }

      // ignore more expensive path (if dist relaxed)
      if cost > dist[graph.idx(position)] { continue; }

      for neighbor in graph.get_neighbors(position) {
        let next = State { cost: cost + cost_fun(graph.get(neighbor)), position: neighbor };

        if next.cost < dist[graph.idx(next.position)] {
          heap.push(next);
          dist[graph.idx(next.position)] = next.cost;
        }
      }
    }

    None
  }

  pub fn parse_input(input: &str) -> Option<CaveGraph> {
    CaveGraph::parse(input)
  }
}
pub fn part_one(input: &str) -> Option<u64> {
  use puzzle::*;

  let cave = parse_input(input)?;
  let top_left = cave.top_left();
  let bottom_right = cave.bottom_right();

  lowest_cost_between(cave, |prop| prop.risk_level, top_left, bottom_right)
}

pub fn part_two(input: &str) -> Option<u64> {
  use puzzle::*;

  // was easier to debug this brute force approach, and also faster! ~3x
  // if I had to guess the slowness was caused by using the HashMap for the dist
  // map even though the graph was actually sparse (just implicit)
  let cave = CaveGraph::parse_and_expand(input)?;
  let top_left = cave.top_left();
  let bottom_right = cave.bottom_right();

  lowest_cost_between(cave, |prop| prop.risk_level, top_left, bottom_right)
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

  #[test]
  fn test_example_part_two() {
    assert_eq!(part_two(EXAMPLE), Some(315));
  }

  #[test]
  fn test_hand_worked_example() {
    let example = r#"
    155
    155
    111
    "#;
    assert_eq!(part_one(example), Some(4));
  }

  #[test]
  fn test_hand_worked_part_two_examples() {
    assert_eq!(part_two("1"), Some(2+3+4+5+6+7+8+9));
    assert_eq!(part_two("2"), Some(3+4+5+6+7+8+9+1));
    assert_eq!(part_two("8"), Some(9+1+2+3+4+5+6+7));
    assert_eq!(part_two("11\n11"), Some(1+1+(2+3+4+5+6+7+8+9)*2));
  }

  #[test]
  fn test_properties_part_two() {
    let example = "1";
    let cave = puzzle::ExpandedCaveGraph::parse(example).unwrap();
    assert_eq!(cave.get(puzzle::CaveLocation { row: 0, column: 0 }).risk_level, 1);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 1, column: 0 }).risk_level, 2);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 0, column: 1 }).risk_level, 2);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 0, column: 2 }).risk_level, 3);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 0, column: 3 }).risk_level, 4);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 0, column: 4 }).risk_level, 5);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 1, column: 4 }).risk_level, 6);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 2, column: 4 }).risk_level, 7);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 3, column: 4 }).risk_level, 8);
    assert_eq!(cave.get(puzzle::CaveLocation { row: 4, column: 4 }).risk_level, 9);

    let overflow_cave = puzzle::ExpandedCaveGraph::parse("2").unwrap();
    assert_eq!(overflow_cave.get(puzzle::CaveLocation { row: 4, column: 4 }).risk_level, 1);
  }

  #[test]
  fn test_add_and_wrap() {
    let wrapped = (1..10).map(|i| puzzle::CaveGraph::add_and_wrap(i, 1)).collect::<Vec<_>>();
    assert_eq!(wrapped, vec![2,3,4,5,6,7,8,9,1]);
  }
}