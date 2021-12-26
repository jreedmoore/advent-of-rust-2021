extern crate itertools;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

mod puzzle {
  use std::collections::VecDeque;

  #[derive(PartialEq, Eq, Clone, Hash, Debug)]
  pub struct Element {
    letter : char
  }

  pub struct Polymer {
    pub elements : Vec<Element>
  }
  impl Polymer {
    pub fn parse(input: &str) -> Option<Polymer> {
      Some(Polymer { elements: input.chars().map(|c| Element { letter: c }).collect() })
    }
    pub fn pretty_print(&self) -> String {
      self.elements.iter().map(|e| e.letter).collect()
    }

    pub fn apply_rules(&mut self, rules : &[PairInsertionRule]) {
      let mut insertions : VecDeque<(usize, Element)> = VecDeque::new();
      for i in 0..self.elements.len() - 1 {
        let l = &self.elements[i];
        let r = &self.elements[i+1];
        if let Some(rule) = rules.iter().find(|rule| rule.left == *l && rule.right == *r) {
          insertions.push_back((i, rule.insert.clone()))
        }
      }
      let mut next_elements : Vec<Element> = Vec::with_capacity(self.elements.len() + insertions.len());
      let mut insert_iter = insertions.iter();
      let mut insert_peek = insert_iter.next();
      for i in 0..self.elements.len() {
        next_elements.push(self.elements[i].clone());
        while let Some((idx, elem)) = insert_peek {
          if *idx == i {
            next_elements.push(elem.clone());
            insert_peek = insert_iter.next();
          } else {
            break;
          }
        }
      }
      self.elements = next_elements;
    }
  }

  pub struct PairInsertionRule {
    left : Element,
    right : Element,
    insert : Element
  }
  impl PairInsertionRule {
    pub fn new(lr : (char, char), insert: char) -> PairInsertionRule {
      let (l, r) = lr;
      PairInsertionRule {
        left: Element { letter: l },
        right: Element { letter: r },
        insert: Element { letter: insert }
      }
    }

    pub fn parse(input: &str) -> Option<PairInsertionRule> {
      let parts : Vec<&str> = input.split_whitespace().collect();
      let pair : Vec<char> = parts[0].chars().collect();
      let insert : char = parts[2].chars().next()?;

      Some(PairInsertionRule { 
        left: Element { letter: pair[0]}, 
        right: Element{ letter: pair[1]}, 
        insert: Element { letter: insert} 
      })
    }
  }
  
  pub fn parse_input(input: &str) -> Option<(Polymer, Vec<PairInsertionRule>)> {
    let mut lines_iter = 
      input.lines()
        .map(|s| s.trim())
        .filter(|s| s.len() != 0);

    let start_polymer = Polymer::parse(lines_iter.next()?)?;
    let rules = lines_iter.map(|s| PairInsertionRule::parse(s)).collect::<Option<Vec<PairInsertionRule>>>()?;

    Some((start_polymer, rules))
  }
}

pub fn step_and_min_max(input: &str, steps: u32) -> Option<u64> {
  let (mut start_polymer, rules) = puzzle::parse_input(input)?;

  for _i in 0..steps {
    println!("{}", _i);
    start_polymer.apply_rules(&rules);
  }
  if let MinMax(min, max) =
    start_polymer.elements.iter()
      .map(|k| (k,1 as usize))
      .into_grouping_map()
      .sum()
      .values()
      .minmax() {

      Some((max-min) as u64)
    } else {
      None
    }

}

pub fn part_one(input: &str) -> Option<u64> {
  step_and_min_max(input, 10)
}

/*
- pair insertion rule only needs to know about pairs, decompose polymer into pairs on parse
- pair insertion rule output only depends on rule, not adjacent elements

- parse polymer into a HashMap<(Element, Element), u64>
- rule application is elements.contains((l,r))
- rule production is li + count, ir + count
*/
pub fn part_two(input: &str) -> Option<u64> {
  step_and_min_max(input, 40)
}

#[cfg(test)]
mod tests {
  use super::*;
  use puzzle::*;

  const EXAMPLE: &'static str = r#"
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
  "#;
  #[test] 
  fn test_example_part_one() {
    assert_eq!(part_one(EXAMPLE), Some(1588));
  }

  #[test] 
  fn test_example_part_two() {
    assert_eq!(part_two(EXAMPLE), Some(2188189693529));
  }

  #[test]
  fn test_first_step() {
    let mut polymer = Polymer::parse("NNCB").unwrap();
    let rules : Vec<PairInsertionRule> = vec![
      PairInsertionRule::new(('N', 'N'), 'C')
    , PairInsertionRule::new(('N', 'C'), 'B')
    , PairInsertionRule::new(('C', 'B'), 'H')
    ];
    polymer.apply_rules(&rules);
    assert_eq!(polymer.pretty_print(), "NCNBCHB");
  }
}