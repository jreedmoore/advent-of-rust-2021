extern crate itertools;

mod puzzle {
  use itertools::Itertools;
  #[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
  pub enum CaveType {
    Start,
    End,
    Big(String),
    Small(String)
  }
  impl CaveType {
    fn parse(input: &str) -> Option<CaveType> {
      match input {
        "start" => Some(CaveType::Start),
        "end" => Some(CaveType::End),
        big if big.chars().all(|c| char::is_uppercase(c)) => Some(CaveType::Big(big.to_string())),
        small if small.chars().all(|c| char::is_lowercase(c)) => Some(CaveType::Small(small.to_string())),
        _ => None
      }
    }
  }

  // undirected edge
  #[derive(Debug, PartialEq)]
  pub struct Connection {
    a : CaveType,
    b : CaveType
  }
  impl Connection {
    fn parse(input : &str) -> Option<Connection> {
      let v: Vec<&str> = input.split("-").collect();
      let a = v[0];
      let b = v[1];
      Some(Connection { a: CaveType::parse(a)?, b: CaveType::parse(b)? })
    }
  }

  #[derive(Debug)]
  pub struct Path {
    caves : Vec<CaveType>
  }
  impl Path {
    pub fn start() -> Path {
      Path { caves : vec![CaveType::Start] }
    }
    pub fn append(&self, v : &CaveType) -> Path {
      let mut new_caves = self.caves.to_vec();
      new_caves.push(v.clone());
      Path { caves: new_caves }
    }

    pub fn entries(&self, v: &CaveType) -> usize {
      self.caves.iter().filter(|c| v == *c).count()
    }

    pub fn max_small_entries(&self) -> Option<usize> {
      self.caves.iter()
        .filter(|v| match v { CaveType::Small(_) => true, _ => false })
        .map(|k| (k,1 as usize))
        .into_grouping_map()
        .sum()
        .values()
        .max()
        .map(|x| x.clone())
    }

    pub fn pretty_print(&self) -> String {
      self.caves.iter().map(|v| {
        match v {
          CaveType::Start => "start",
          CaveType::End => "end",
          CaveType::Big(b) => &b[..],
          CaveType::Small(s) => &s[..]
        }
      }).collect::<Vec<&str>>().join(",")
    }
  }

  pub struct CaveSystem {
    connections : Vec<Connection>
  }
  impl CaveSystem {
    pub fn valid_paths_count<F : Fn(&Path, &CaveType) -> bool>(&self, f : F) -> u32 {
      let mut unfinished : Vec<Path> = vec![Path::start()];
      let mut finished : u32 = 0;
      while let Some(path) = unfinished.pop() {
        let last = path.caves.last().unwrap();
        self.next_caves(last).iter()
          .filter(|v| {
            match v {
              CaveType::Small(_) => f(&path, v),
              CaveType::Start => false,
              _ => true
            }
          })
          .for_each(|v| {
            if *v == CaveType::End {
              // println!("{}", path.append(v).pretty_print());
              finished += 1;
            } else {
              unfinished.push(path.append(v))
            }
          })
      }
      finished
    }

    fn next_caves(&self, from: &CaveType) -> Vec<CaveType> {
      self.connections.iter()
        .flat_map(|conn| {
          if conn.a == *from {
            Some(conn.b.clone())
          } else if conn.b == *from {
            Some(conn.a.clone())
          } else {
            None
          }
        })
        .collect()
    }
  }

  pub fn parse_input(input: &str) -> Option<CaveSystem> {
    let connections: Option<Vec<Connection>> = input.lines().map(|l| l.trim()).filter(|l| l.len() != 0).map(|l| Connection::parse(l)).collect();
    Some(CaveSystem { connections: connections? })
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use CaveType::*;

    #[test]
    fn test_connection_parse() {
      assert_eq!(Connection::parse("start-A"), Some(Connection {a: Start, b: Big("A".to_string())}));
      assert_eq!(Connection::parse("start-b"), Some(Connection {a: Start, b: Small("b".to_string())}));
      assert_eq!(Connection::parse("A-b"), Some(Connection {a: Big("A".to_string()), b: Small("b".to_string())}));
      assert_eq!(Connection::parse("b-end"), Some(Connection {a: Small("b".to_string()), b: End}));
    }
  }
}
pub fn part_one(input: &str) -> Option<u32> {
  let system = puzzle::parse_input(input)?;
  let paths = system.valid_paths_count(|path: &puzzle::Path, v| path.entries(v) < 1);
  Some(paths)
}

pub fn part_two(input: &str) -> Option<u32> {
  let system = puzzle::parse_input(input)?;
  let paths = system.valid_paths_count(|path: &puzzle::Path, v| {
    if path.max_small_entries().unwrap_or(0) >= 2 {
      path.entries(v) < 1
    } else {
      path.entries(v) < 2
    }
  });
  Some(paths)
}

#[cfg(test)] 
mod tests {
  use super::*;

  // bad: start,b,A,c,A,c,A,b,A,end
/*
GOOD:
start,A,c,A,c,A,b,A,end
BAD:
start,A,c,A,c,A,b,A,b,end
start,A,c,A,c,A,b,A,b,A,end
*/
  const EXAMPLE_1 : &'static str = r#"
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
  "#;

  const EXAMPLE_2: &'static str = r#"
    dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
  "#;

  const EXAMPLE_3: &'static str = r#"
    fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
  "#;

  #[test]
  fn test_part_one_examples() {
    assert_eq!(part_one(EXAMPLE_1), Some(10));
    assert_eq!(part_one(EXAMPLE_2), Some(19));
    assert_eq!(part_one(EXAMPLE_3), Some(226));
  }

  #[test]
  fn test_part_two_examples() {
    assert_eq!(part_two(EXAMPLE_1), Some(36));
    assert_eq!(part_two(EXAMPLE_2), Some(103));
    assert_eq!(part_two(EXAMPLE_3), Some(3509));
  }
}