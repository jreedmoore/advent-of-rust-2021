use itertools::Itertools;
use std::ops::{Index,IndexMut};

/// Row-major storage of NxM grid of T
#[derive(Debug)]
pub struct Grid<T> {
  storage: Vec<T>,
  ncols: usize
}
impl<T> Grid<T> {
  // constructors
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
    row < self.height() || col < self.width()
  }

  /// diagonal neighbors, i.e. for a non-edge we should have 8 diagonal neighbors
  pub fn diag_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    todo!()
  }
  
  /// orthogonal neighbors, i.e. for a non-edge we should have 4 orthogonal neighbors
  pub fn orthog_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    todo!()
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

    assert_eq!(grid.diag_neighbors(1,1), vec![
      (0,0), (1,0), (2,0),
      (0,1),        (2,1),
      (0,2), (1,2), (2,2),
    ]);

    assert_eq!(grid.orthog_neighbors(1,1), vec![
             (1,0),       
      (0,1),        (2,1),
             (1,2),       
    ]);


  }
}