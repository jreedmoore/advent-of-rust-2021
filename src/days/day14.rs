extern crate itertools;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

mod puzzle {
  use itertools::Itertools;
  use std::collections::HashMap;

  #[derive(PartialEq, Eq, Clone, Hash, Debug)]
  pub struct Element {
    pub letter : char
  }

  #[derive(PartialEq, Eq, Clone, Hash, Debug)]
  pub struct ElementPair {
    l : Element,
    r : Element
  }
  impl ElementPair {
    pub fn new(l: char, r: char) -> ElementPair {
      ElementPair {
        l: Element{ letter: l },
        r: Element{ letter: r }
      }
    }
    pub fn of(s: &str) -> ElementPair {
      let cs : Vec<char> = s.chars().collect();
      ElementPair::new(cs[0], cs[1])
    }
  }

  pub struct Polymer {
    pub pairs : HashMap<ElementPair, usize>,
    pub elements : HashMap<Element, usize>
  }
  impl Polymer {
    pub fn parse(input: &str) -> Option<Polymer> {
      let pairs = 
        input
          .as_bytes()
          .windows(2)
          .map(|pair| (ElementPair::new(pair[0] as char, pair[1] as char), 1))
          .into_grouping_map()
          .sum();

      let elements = 
        input
          .chars()
          .map(|c| (Element {letter: c},1 as usize))
          .into_grouping_map()
          .sum();

      Some(Polymer { pairs: pairs, elements: elements })
    }

    pub fn apply_rules(&mut self, rules : &[PairInsertionRule]) {
      let applied_rules : Vec<(&PairInsertionRule, usize)>= 
        rules.iter().filter_map(|rule| {
          self.pairs
            .get(&rule.pair())
            //TODO: understand flat_map trait bounds and drop applications with count = 0
            .map(|count| (rule, count.clone()))
        }).collect();
      
      for (rule, count) in applied_rules {
        let output = rule.output_pairs();
        *self.pairs.entry(rule.pair()).or_insert(0) -= count;
        *self.pairs.entry(output[0].clone()).or_insert(0) += count;
        *self.pairs.entry(output[1].clone()).or_insert(0) += count;

        *self.elements.entry(rule.insert.clone()).or_insert(0) += count;
      }
    }

    pub fn elements(&self) -> &HashMap<Element, usize> {
      &self.elements
    }

    pub fn element_count(&self, element: Element) -> usize {
      self.elements().get(&element).map(|n| n.clone()).unwrap_or(0)
    }
  }

  #[derive(Debug)]
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

    pub fn pair(&self) -> ElementPair {
      ElementPair {l: self.left.clone(), r: self.right.clone() }
    }
    
    pub fn output_pairs(&self) -> Vec<ElementPair> {
      vec![
        ElementPair {l: self.left.clone(), r: self.insert.clone()}
      , ElementPair {l: self.insert.clone(), r: self.right.clone()}
      ]
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
  if let MinMax(min, max) = start_polymer.elements().values().minmax() {
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
    // "NCNBCHB"
    assert_eq!(polymer.element_count(Element { letter: 'N'}), 2);
    assert_eq!(polymer.element_count(Element { letter: 'C'}), 2);
    assert_eq!(polymer.element_count(Element { letter: 'B'}), 2);
    assert_eq!(polymer.element_count(Element { letter: 'H'}), 1);
  }

  #[test]
  fn test_parse_puzzle() {
    let polymer = Polymer::parse("PFVKOBSHPSPOOOCOOHBP").unwrap();

    assert_eq!(*polymer.pairs.get(&ElementPair::of("OO")).unwrap(), 3);
    assert_eq!(*polymer.elements.get(&Element {letter: 'O'}).unwrap(), 6);
  }
}