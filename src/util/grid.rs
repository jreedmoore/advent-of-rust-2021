use itertools::Itertools;
use std::ops::{Index,IndexMut};
use std::fmt::Display;

/// Row-major storage of NxM grid of T
/// (0,0) is top-left, (n,m) is bottom-right
#[derive(Debug)]
pub struct Grid<T> {
  storage: Vec<T>,
  ncols: usize
}
impl<T> Grid<T> {
  // constructors
  #[allow(dead_code)]
  pub fn from_data(ncols: usize, data: Vec<T>) -> Grid<T> {
    Grid { storage: data, ncols: ncols }
  }
    
  pub fn width(&self) -> usize {
    self.ncols
  }

  pub fn height(&self) -> usize {
    self.storage.len() / self.ncols
  }

  pub fn in_bounds(&self, row: usize, col: usize) -> bool {
    row < self.height() && col < self.width()
  }

  fn checked_offset_pos(&self, row: usize, col: usize, row_off: i32, col_off: i32) -> Option<(usize, usize)> {
    if (row as i32 + row_off < 0) || (col as i32 + col_off < 0) {
      None
    } else {
      let r = ((row as i32) + row_off) as usize;
      let c = ((col as i32) + col_off) as usize;
      if self.in_bounds(r, c) {
        Some((r, c))
      } else {
        None
      }
    }
  }

  // all neighbor functions should be top-left to bottom-right
  /// diagonal neighbors, i.e. for a non-edge we should have 8 diagonal neighbors
  #[allow(dead_code)]
  pub fn diag_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    vec![
      self.checked_offset_pos(row, col, -1, -1),
      self.checked_offset_pos(row, col, -1, 0),
      self.checked_offset_pos(row, col, -1, 1),
      self.checked_offset_pos(row, col, 0, -1),
      // no self
      self.checked_offset_pos(row, col, 0, 1),
      self.checked_offset_pos(row, col, 1, -1),
      self.checked_offset_pos(row, col, 1, 0),
      self.checked_offset_pos(row, col, 1, 1),
    ].into_iter().flatten().collect_vec()
  }

  /// diagonal neighbors and self, ibid, including (row, col)
  #[allow(dead_code)]
  pub fn diag_neighbors_self(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    vec![
      self.checked_offset_pos(row, col, -1, -1),
      self.checked_offset_pos(row, col, -1, 0),
      self.checked_offset_pos(row, col, -1, 1),
      self.checked_offset_pos(row, col, 0, -1),
      self.checked_offset_pos(row, col, 0, 0), // could be out of bounds!
      self.checked_offset_pos(row, col, 0, 1),
      self.checked_offset_pos(row, col, 1, -1),
      self.checked_offset_pos(row, col, 1, 0),
      self.checked_offset_pos(row, col, 1, 1),
    ].into_iter().flatten().collect_vec()
  }
  
  /// orthogonal neighbors, i.e. for a non-edge we should have 4 orthogonal neighbors
  #[allow(dead_code)]
  pub fn orthog_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    vec![
      self.checked_offset_pos(row, col, -1, 0),
      self.checked_offset_pos(row, col, 0, -1),
      self.checked_offset_pos(row, col, 0, 1),
      self.checked_offset_pos(row, col, 1, 0),
    ].into_iter().flatten().collect_vec()
  }

  pub fn diag_offsets(&self, row: i32, col: i32) -> Vec<(i32, i32)> {
    vec![
      (row-1, col-1),
      (row-1, col),
      (row-1, col+1),
      (row, col-1),
      (row, col),
      (row, col+1),
      (row+1, col-1),
      (row+1, col),
      (row+1, col+1),
    ]
  }

  fn index(&self, row: usize, col: usize) -> usize {
    row*self.width() + col
  }

  pub fn get(&self, row: usize, col: usize) -> Option<&T> {
    if self.in_bounds(row, col) {
      Some(&self.storage[self.index(row, col)])
    } else {
      None
    }
  }

  /// Get with possibly negative integers
  pub fn get_int(&self, row: i32, col: i32) -> Option<&T> {
    if row < 0 || col < 0 {
      None
    } else {
      self.get(row as usize, col as usize)
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.storage.iter()
  }
}
impl<T: Clone> Grid<T> {
  pub fn from_rows(rows: Vec<Vec<T>>) -> Grid<T> {
    let ncols = rows[0].len();
    for row in &rows {
      if row.len() != ncols {
        panic!("Mismatched row lengths");
      }
    }
    let data = rows.iter().flatten().cloned().collect_vec();
    Grid { storage: data, ncols: ncols }
  }

  pub fn fill(ncols: usize, nrows: usize, elem: T) -> Grid<T> {
    let mut data: Vec<T> = Vec::new();
    data.resize(ncols * nrows, elem);
    Grid { storage: data, ncols: ncols }
  }
}
impl<T> Index<(usize,usize)> for Grid<T> {
  type Output = T;

  fn index(&self, index: (usize, usize)) -> &T {
    let (r,c) = index;
    if self.in_bounds(r, c) {
      self.get(r, c).unwrap()
    } else {
      panic!("(row, col) ({}, {}) out of bounds", r, c);
    }
  }
}
impl<T> IndexMut<(usize,usize)> for Grid<T> {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
    let (r,c) = index;
    let idx = self.index(r, c);
    if self.in_bounds(r, c) {
      &mut self.storage[idx]
    } else {
      panic!("(row, col) ({}, {}) out of bounds", r, c);
    }
  }
}
impl<T: Display> Display for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
    for col in 0..self.height() {
      for row in 0..self.width() {
        write!(f, "{}", self[(row,col)])?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // take a 3x3 grid and validate that we can manipulate it
  fn test_valid_access(grid: Grid<bool>) {
    //assert_eq!(grid.in_bounds(0, 0));
    let mut grid = grid;

    for row in 0..3 {
      for col in 0..3 {
        assert_eq!(grid.get(row, col), Some(&false), "({},{})", row, col);
        assert_eq!(grid[(row, col)], false);
      }
    }

    grid[(0,0)] = true;
    assert_eq!(grid[(0,0)], true);
    assert_eq!(grid[(0,1)], false);

    grid[(1,1)] = true;
    assert_eq!(grid[(1,1)], true);
    assert_eq!(grid[(1,2)], false);
    
    grid[(2,2)] = true;
    assert_eq!(grid[(2,2)], true);
    assert_eq!(grid[(2,1)], false);
  }

  #[test]
  fn test_from_data() {
    test_valid_access(Grid::from_data(3, vec![
      false, false, false,
      false, false, false,
      false, false, false
    ]))
  }

  #[test]
  fn test_from_rows() {
    test_valid_access(Grid::from_rows(vec![
      vec![false, false, false],
      vec![false, false, false],
      vec![false, false, false],
    ]));
  }

  #[test]
  fn test_fill_constructor() {
    let grid: Grid<bool> = Grid::fill(3, 3, false);
    test_valid_access(grid);
  }

  #[test]
  fn test_neighbors() {
    let grid: Grid<bool> = Grid::fill(3, 3, false);

    assert_eq!(grid.diag_neighbors_self(1,1), vec![
      (0,0), (0,1), (0,2),
      (1,0), (1,1), (1,2),
      (2,0), (2,1), (2,2),
    ]);

    assert_eq!(grid.diag_neighbors(1,1), vec![
      (0,0), (0,1), (0,2),
      (1,0),        (1,2),
      (2,0), (2,1), (2,2),
    ]);

    assert_eq!(grid.orthog_neighbors(1,1), vec![
             (0,1),       
      (1,0),        (1,2),
             (2,1),       
    ]);

    assert_eq!(grid.diag_neighbors(0,0), vec![
                    (0,1),
             (1,0), (1,1),
    ]);

    assert_eq!(grid.diag_neighbors(2,2), vec![
      (1,1), (1,2),
      (2,1),       
    ]);

    assert_eq!(grid.diag_neighbors_self(3, 3), vec![(2,2)]);
  }

  #[test]
  fn test_offsets() {
    let grid: Grid<bool> = Grid::from_data(3, vec![
      false, true, false,
      true, false, true,
      false, true, false
    ]);

    assert_eq!(grid.diag_offsets(1,1), vec![
      (0,0), (0,1), (0,2),
      (1,0), (1,1), (1,2),
      (2,0), (2,1), (2,2),
    ]);

    assert_eq!(
      grid.diag_offsets(0, 0).into_iter().map(|(r,c)| grid.get_int(r,c).map(|rf| rf.clone())).collect_vec(),
      vec![
        None, None,        None,
        None, Some(false), Some(true),
        None, Some(true),  Some(false)
      ]);
  }
}