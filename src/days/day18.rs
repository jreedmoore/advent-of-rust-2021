mod puzzle {
  #[derive(PartialEq, Debug, Clone)]
  pub struct Elem {
    depth: u8,
    value: u8
  }
  #[derive(PartialEq, Debug, Clone)]
  pub struct SnailfishNum {
    nums: Vec<Elem>
  }
  impl SnailfishNum {
    pub fn parse(input: &str) -> Option<SnailfishNum> {
      parser::parse(input).ok().map(|t| t.1)
    }
    // taken from tim visee, and I don't understand this implementation very well...
    // https://github.com/timvisee/advent-of-code-2021/blob/master/day18a/src/main.rs#L42
    pub fn magnitude(&self) -> u64 {
      fn run(idx: &mut usize, depth: u8, num: &SnailfishNum) -> u64 {
        3 * if num.nums[*idx].depth == depth {
          *idx += 1;
          num.nums[*idx - 1].value as u64
        } else {
          run(idx, depth + 1, num)
        } +
        2 * if num.nums[*idx].depth == depth {
          *idx += 1;
          num.nums[*idx - 1].value as u64
        } else {
          run(idx, depth + 1, num)
        }
      }
      run(&mut 0, 1, self)
    }
    fn add(&mut self, other: &mut SnailfishNum) {
      self.nums.append(&mut other.nums);
      for mut elem in self.nums.iter_mut() {
        elem.depth += 1;
      }
    }
    fn explode(&mut self) -> bool {
      for i in 0..self.nums.len() - 1 {
        let l = self.nums[i].clone();
        if l.depth == 5 {
          let r = self.nums[i+1].clone();
          self.nums[i].depth -= 1;
          self.nums[i].value = 0;
          self.nums.remove(i+1);
          if i > 0 {
            self.nums[i-1].value += l.value;
          }
          if i < self.nums.len() - 1 {
            self.nums[i+1].value += r.value;
          }
          return true;
        }
      }
      false
    }

    fn half_up(x: u8) -> u8 {
      if x&1 != 0 {
        x / 2 + 1
      } else {
        x / 2
      }
    }
    fn half_down(x: u8) -> u8 {
      x / 2
    }
    fn split(&mut self) -> bool {
      for i in 0..self.nums.len() {
        let v = self.nums[i].clone();
        if v.value >= 10 {
          self.nums[i] = Elem { value: SnailfishNum::half_up(v.value), depth: v.depth + 1 };
          self.nums.insert(i, Elem { value: SnailfishNum::half_down(v.value), depth: v.depth + 1});
          return true;
        }
      }
      false
    }
    fn reduce(&mut self) {
      loop {
        if self.explode() { continue; }
        if self.split() { continue; }
        break;
      }
    }
    pub fn add_and_reduce(&mut self, other: &mut SnailfishNum) {
      self.add(other);
      self.reduce();
    }
  }
  mod parser {
    use super::*;
    use nom::{
      IResult,
      branch::alt,
      combinator::{map, value},
      character::complete::char
    };

    #[derive(Clone)]
    enum Action {
      Inc,
      Dec,
      Push(u8),
      NoOp
    }
    pub fn parse(input: &str) -> IResult<&str, SnailfishNum> {
      let mut d = 0;
      let mut input_c : &str = input.clone();
      let mut elems: Vec<Elem> = Vec::new();
      while !input_c.is_empty() {
        let (n, action) : (&str, Action) = 
          alt((
            value(Action::Inc, char('[')),
            value(Action::Dec, char(']')),
            value(Action::NoOp, char(',')),
            map(nom::character::complete::u8, |v| Action::Push(v)),
          ))(input_c)?;
        input_c = n;

        match action {
          Action::Inc => d += 1,
          Action::Dec => d -= 1,
          Action::Push(v) => elems.push(Elem { depth: d, value: v }),
          Action::NoOp => ()
        }
      }
      Ok(("", SnailfishNum { nums: elems }))
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
        let mut pre = SnailfishNum::parse(pre).unwrap();
        let post = SnailfishNum::parse(post).unwrap();
        assert!(pre.explode());
        assert_eq!(pre, post.clone(), "exploding\n{:?} =>\n{:?}", pre, post);
      }
      test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"); //no regular number left
      test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"); //no regular number right
      test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"); //normal
      test("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"); // [3,2] unaffected, not left most
      test("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]","[[3,[2,[8,0]]],[9,[5,[7,0]]]]"); //next step
    }

    #[test]
    fn test_split() {
      fn test(pre_s: &str, post_s: &str) {
        let mut pre = SnailfishNum::parse(pre_s).unwrap();
        let post = SnailfishNum::parse(post_s).unwrap();
        assert!(pre.split(), "splitting {} {}", pre_s, post_s);
        assert_eq!(pre, post, "splitting {} {}", pre_s, post_s);
      }
      test("[10,0]", "[[5,5],0]");
      test("[11,0]", "[[5,6],0]");
      test("[12,0]", "[[6,6],0]");
    }

    #[test]
    fn test_reduce() {
      let mut a = SnailfishNum::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
      a.reduce();
      assert_eq!(
        a,
        SnailfishNum::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap()
      )
    }

    #[test]
    fn test_add_and_reduce() {
      let mut a = SnailfishNum::parse("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
      let mut b = SnailfishNum::parse("[1,1]").unwrap();
      a.add_and_reduce(&mut b);
      assert_eq!(
        a,
        SnailfishNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()
      )
    }
  }
}
pub fn part_one(input: &str) -> Option<u64> {
  let mut nums = 
    input.lines()
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .map(|l| puzzle::SnailfishNum::parse(l))
      .collect::<Option<Vec<puzzle::SnailfishNum>>>()?;

  nums.iter_mut().reduce(|a,b| {a.add_and_reduce(b); a }).map(|n| n.magnitude())
}

use itertools::Itertools;

pub fn part_two(input: &str) -> Option<u64> {
  let nums = 
    input.lines()
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .map(|l| puzzle::SnailfishNum::parse(l))
      .collect::<Option<Vec<puzzle::SnailfishNum>>>()?;

  nums.iter().cartesian_product(nums.iter())
    .flat_map(|(a,b)| {
      let mut ab = a.clone();
      ab.add_and_reduce(&mut b.clone());
      let mut ba = b.clone();
      ba.add_and_reduce(&mut a.clone());
      vec![ab, ba]
    })
    .map(|n| n.magnitude())
    .max()
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

  #[test]
  fn test_part_two_example() {
    //assert_eq!(part_one("[1,1]\n[2,2]\n[3,3]\n[4,4]"), Some(445));
    //assert_eq!(part_one("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]"), Some(791));
    //assert_eq!(part_one("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]"), Some(1137));
    //assert_eq!(part_one(EXAMPLE_2), Some(3488));
    assert_eq!(part_two(EXAMPLE), Some(3993));
  }

}
