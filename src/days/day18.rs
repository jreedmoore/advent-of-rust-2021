mod puzzle {
  #[derive(PartialEq, Debug)]
  pub enum SnailfishNum {
  }
  impl SnailfishNum {
    fn parse(input: &str) -> Option<SnailfishNum> {
      todo!()
    }
    fn magnitude(&self) -> u64 {
      todo!()
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_num_magnitude() {
      assert_eq!(SnailfishNum::parse("[9,1]").unwrap().magnitude(), 29);
      assert_eq!(SnailfishNum::parse("[1,9]").unwrap().magnitude(), 21);
      assert_eq!(SnailfishNum::parse("[[9,1],[1,9]]").unwrap().magnitude(), 129);
      assert_eq!(SnailfishNum::parse("[[1,2],[[3,4],5]]").unwrap().magnitude(), 143);
      assert_eq!(SnailfishNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().magnitude(), 1384);
      assert_eq!(SnailfishNum::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().magnitude(), 445);
      assert_eq!(SnailfishNum::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().magnitude(), 791);
      assert_eq!(SnailfishNum::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().magnitude(), 1137);
      assert_eq!(SnailfishNum::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap().magnitude(), 3488);
    }
  }
}
pub fn part_one(input: &str) -> Option<u64> {
  todo!()
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &'static str = r#"
    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
  "#;

  const EXAMPLE_SUM:  &'static str = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";

  const EXAMPLE_2: &'static str = r#"
    [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]] 
  "#;

  #[test]
  fn test_part_one_example() {
    assert_eq!(part_one("[1,1]\n[2,2]\n[3,3]\n[4,4]"), Some(445));
    assert_eq!(part_one("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]"), Some(791));
    assert_eq!(part_one("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]"), Some(1137));
    assert_eq!(part_one(EXAMPLE_2), Some(3488));
    assert_eq!(part_one(EXAMPLE), Some(4140));
  }
}
