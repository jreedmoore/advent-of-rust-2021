mod puzzle {

}

fn part_one(input: &str) -> Option<u32> {
  todo!()
}

#[cfg(test)]
mod tests {
  use super::*;

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
}