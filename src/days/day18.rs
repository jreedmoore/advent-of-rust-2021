mod puzzle {
  #[derive(PartialEq, Debug, Clone)]
  pub enum SnailfishNum {
    Pair(Box<SnailfishNum>, Box<SnailfishNum>),
    Literal(u64)
  }
  impl std::fmt::Display for SnailfishNum {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
      match self {
        SnailfishNum::Literal(v) => write!(f, "{}", v),
        SnailfishNum::Pair(l, r) => write!(f, "[{},{}]", l, r),
      }
    }
  }
  impl SnailfishNum {
    fn parse(input: &str) -> Option<SnailfishNum> {
      parser::snailfish_num(input).ok().map(|(_,num)| *num)
    }
    fn magnitude(&self) -> u64 {
      match self {
        SnailfishNum::Pair(l, r) => 3*l.magnitude() + 2*r.magnitude(),
        SnailfishNum::Literal(n) => n.clone()
      }
    }
    fn add(&self, other: &SnailfishNum) -> SnailfishNum {
      SnailfishNum::Pair(Box::new(self.clone()), Box::new(other.clone()))
    }
    fn explode(&self) -> Option<SnailfishNum> {
      fn run(num: &SnailfishNum, level: usize) -> Option<(SnailfishNum, Option<u64>)> {
        let res = if level >= 3 {
          match num {
            SnailfishNum::Pair(l, r) => {
              match (*l.clone(), *r.clone()) {
                (SnailfishNum::Literal(l), SnailfishNum::Pair(ll, rr)) => {
                  match (*ll.clone(), *rr.clone()) {
                    (SnailfishNum::Literal(ll), SnailfishNum::Literal(rr)) => Some((
                      SnailfishNum::Pair(
                        Box::new(SnailfishNum::Literal(l+ll)), 
                        Box::new(SnailfishNum::Literal(0))
                      ),
                      Some(rr)
                      )),
                    _ => None
                  }
                }
                (SnailfishNum::Pair(ll, rr), SnailfishNum::Literal(r)) => {
                  match (*ll.clone(), *rr.clone()) {
                    (SnailfishNum::Literal(ll), SnailfishNum::Literal(rr)) => Some((
                      SnailfishNum::Pair(
                        Box::new(SnailfishNum::Literal(0)), 
                        Box::new(SnailfishNum::Literal(r+rr))
                      ),
                      Some(ll)
                    )),
                    _ => None
                  }
                }
                _ => None
              }
            }
            _ => None
          }
        } else {
          match num {
            SnailfishNum::Literal(_) => {
              None
            }
            SnailfishNum::Pair(l, r) => {
              let lo = run(l, level+1);
              let ro = run(r, level+1);
              match (lo, ro) {
                (None, None) => None,
                (Some((l,_)), Some(_)) => Some((SnailfishNum::Pair(Box::new(l), r.clone()), None)), // only take left-most change
                (None, Some((r,_))) => Some((SnailfishNum::Pair(l.clone(), Box::new(r)), None)),
                (Some((l,_)), None) => Some((SnailfishNum::Pair(Box::new(l), r.clone()), None)),
              }
            }
          }  
        };
        println!("{} {:?} => {:?}", level, num, res);
        res
      }
      run(self, 0).map(|t| t.0)
    }

    fn half_up(x: u64) -> u64 {
      if x&1 != 0 {
        x / 2 + 1
      } else {
        x / 2
      }
    }
    fn half_down(x: u64) -> u64 {
      x / 2
    }
    fn split(&self) -> Option<SnailfishNum> {
      match self {
        SnailfishNum::Literal(v) => {
          if *v >= 10 {
            Some(SnailfishNum::Pair(
              Box::new(SnailfishNum::Literal(SnailfishNum::half_down(*v))),
              Box::new(SnailfishNum::Literal(SnailfishNum::half_up(*v))),
            ))
          } else {
            None
          }
        }
        SnailfishNum::Pair(l, r) => {
          let lo = l.split();
          let ro = r.split();
          match (lo, ro) {
            (None, None) => None,
            (Some(l), Some(r)) => Some(SnailfishNum::Pair(Box::new(l), Box::new(r))),
            (None, Some(r)) => Some(SnailfishNum::Pair(l.clone(), Box::new(r))),
            (Some(l), None) => Some(SnailfishNum::Pair(Box::new(l), r.clone())),
          }
        }
      } 
    }
    fn reduce(&mut self) -> SnailfishNum {
      let mut acc = self.clone();
      loop {
        if let Some(explode) = acc.explode() {
          acc = explode;
          continue;
        }
        if let Some(split) = acc.split() {
          acc = split;
          continue
        }
        return acc;
      }
    }
    fn add_and_reduce(&self, other: &SnailfishNum) -> SnailfishNum {
      self.add(other).reduce()
    }
  }
  mod parser {
    use super::*;
    use nom::{
      IResult,
      branch::alt,
      combinator::map,
      sequence::{delimited, separated_pair},
      bytes::complete::tag
    };

    fn snailfish_elem(input: &str) -> IResult<&str, Box<SnailfishNum>> {
      alt((
        map(nom::character::complete::u64, |n| Box::new(SnailfishNum::Literal(n))),
        snailfish_num
      ))(input)
    }

    pub fn snailfish_num(input: &str) -> IResult<&str, Box<SnailfishNum>> {
      map(
        delimited(
          tag("["), 
          separated_pair(snailfish_elem, tag(","), snailfish_elem), 
          tag("]")
        ),
        |(l,r)| Box::new(SnailfishNum::Pair(l, r))
      )(input)
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

    #[test]
    fn test_explode() {
      fn test(pre: &str, post: &str) {
        let pre = SnailfishNum::parse(pre).unwrap();
        let post = SnailfishNum::parse(post).unwrap();
        println!("exploding\n{}=>{}", pre, post);
        assert_eq!(pre.explode(), Some(post.clone()), "exploding\n{} =>\n{}", pre, post);
      }
      test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"); //no regular number left
      test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"); //no regular number right
      test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"); //normal
      test("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"); // [3,2] unaffected, not left most
      test("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]","[[3,[2,[8,0]]],[9,[5,[7,0]]]]"); //next step
    }

    #[test]
    fn test_split() {
      fn test(pre: &str, post: &str) {
        let pre = SnailfishNum::parse(pre).unwrap();
        let post = SnailfishNum::parse(post).unwrap();
        assert_eq!(pre.split(), Some(post));
      }
      test("[10,0]", "[[5,5],0]");
      test("[11,0]", "[[5,6],0]");
      test("[12,0]", "[[6,6],0]");
    }

    #[test]
    fn test_reduce() {
      assert_eq!(
        SnailfishNum::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap().reduce(), 
        SnailfishNum::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap()
      )
    }

    #[test]
    fn test_add_and_reduce() {
      let a = SnailfishNum::parse("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
      let b = SnailfishNum::parse("[1,1]").unwrap();
      assert_eq!(
        a.add_and_reduce(&b),
        SnailfishNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()
      )
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
