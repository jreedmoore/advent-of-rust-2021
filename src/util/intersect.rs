pub fn count_intersect<T: Ord>(a: &[T], b: &[T]) -> usize {
  let mut a_i = 0;
  let mut b_i = 0;
  let mut count = 0;
  while a_i < a.len() && b_i < b.len() {
    if a[a_i] == b[b_i] {
      count += 1;
      a_i += 1;
      b_i += 1;
    } else if a[a_i] < b[b_i] {
      a_i += 1;
    } else if a[a_i] > b[b_i] {
      b_i += 1;
    }
  }
  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_intersect() {
    let a = vec![1,2,3];
    assert_eq!(count_intersect(&a, &a), 3);
    assert_eq!(count_intersect(&vec![0,1,2,3], &vec![1,2,3]), 3);
    assert_eq!(count_intersect(&vec![1,2,3], &vec![0,1,2,3]), 3);
    assert_eq!(count_intersect(&vec![1,2,3], &vec![1,2,3,4]), 3);
    assert_eq!(count_intersect(&vec![1,2,3], &vec![4,5,6]), 0);
  }
}