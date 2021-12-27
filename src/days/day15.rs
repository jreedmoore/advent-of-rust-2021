mod puzzle {
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
  }
  impl CaveGraph {
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
  }

  #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
  pub struct CaveLocation {
    row: usize,
    column: usize
  }
  pub struct LocationProperties {
    pub risk_level: u64
  }

  use std::cmp::Ordering;
  use std::collections::BinaryHeap;

  #[derive(Copy, Clone, Eq, PartialEq)]
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
  fn test_hand_worked_example() {
    let example = r#"
    155
    155
    111
    "#;
    assert_eq!(part_one(example), Some(4));
  }
}