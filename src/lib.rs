#[derive(Debug)]
pub struct Vector<K> {
  data: Vec<K>
}

#[derive(Debug)]
pub struct Matrix<K> {
  data: Vec<Vec<K>>,
  col: usize,
  row: usize
}

impl<K> Vector<K> {
  pub fn new() -> Vector<K> {
    Vector {
      data: Vec::<K>::new()
    }
  }

  pub fn from(src: Vec<K>) -> Vector<K> {
    Vector {
      data: Vec::<K>::from(src)
    }
  }

  pub fn push(&mut self, value: K) {
    self.data.push(value);
  }

  pub fn pop(&mut self) {
    self.data.pop();
  }
}

impl<K> Vector<K>
  where
    // Bounds from impl:
    K: std::ops::AddAssign<K> + Copy
{
  pub fn add(&mut self, v: &Vector<K>) -> Result<(), &str> {

    if v.data.len() > 2 {
      return Err("Cannot add 2 vectors themselves: one of them has more than 2 coordinate points.");
    }

    self.data[0] += v.data[0];
    self.data[1] += v.data[1];

    Ok(())
  }
}

impl<K> Vector<K>
  where
    // Bounds from impl:
    K: std::ops::SubAssign<K> + Copy
{
  pub fn sub(&mut self, v: &Vector<K>) -> Result<(), &str> {

    if v.data.len() > 2 {
      return Err("Cannot substract 2 vectors themselves: one of them has more than 2 coordinate points.");
    }

    self.data[0] -= v.data[0];
    self.data[1] -= v.data[1];

    Ok(())
  }
}

impl<K> Vector<K>
  where
    // Bounds from impl:
    K: std::ops::MulAssign<K> + Copy
{
  pub fn scl(&mut self, a: K) -> Result<(), &str> {

    if self.data.len() > 2 {
      return Err("Cannot scale a vector with more than 2 coordinate points.");
    }

    self.data[0] *= a;
    self.data[1] *= a;

    Ok(())
  }
}

impl<K> Matrix<K> {
  pub fn new() -> Matrix<K> {
    Matrix {
      data: Vec::<Vec<K>>::new(),
      row: 0,
      col: 0
    }
  }

  pub fn from(src: Vec<Vec<K>>) -> Matrix<K> {
    let col_src = src[0].len();
    let row_src = src.len();
    Matrix {
      data: Vec::<Vec<K>>::from(src),
      col: col_src,
      row: row_src
    }
  }
}

impl<K> Matrix<K>
  where
    // Bounds from impl:
    K: std::ops::AddAssign + Copy
{
  pub fn add(&mut self, v: &Matrix<K>) -> Result<(), &str> {
    if self.col != v.col {
      return Err("Cannot add 2 matrices with different number of columns.");
    }
    if self.row != v.row {
      return Err("Cannot add 2 matrices with different number of rows.");
    }

    for r in 0..self.row {
      for c in 0..self.col {
        self.data[r][c] += v.data[r][c];
      }
    } 

    Ok(())
  }
}
