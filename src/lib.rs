#[derive(Debug)]
pub struct Vector<K> {
  data: Vec<K>
}

#[derive(Debug)]
pub struct Matrix<K> {
  data: Vec<Vec<K>>,
  col: usize,
  row: usize,
  is_rectangular: bool
}

impl<K> Vector<K> {
  pub fn new() -> Self {
    Vector {
      data: Vec::<K>::new()
    }
  }

  pub fn from(src: Vec<K>) -> Self {
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

    if self.data.len() != v.data.len() {
      return Err("Cannot add 2 vectors themselves: they have different number of dimension.");
    }

    for i in 0..self.data.len() {
      self.data[i] += v.data[i];
    }

    Ok(())
  }
}

impl<K> Vector<K>
  where
    // Bounds from impl:
    K: std::ops::SubAssign<K> + Copy
{
  pub fn sub(&mut self, v: &Vector<K>) -> Result<(), &str> {

    if self.data.len() != v.data.len() {
      return Err("Cannot substract 2 vectors themselves: they have different number of dimension.");
    }

    for i in 0..self.data.len() {
      self.data[i] -= v.data[i];
    }

    Ok(())
  }
}

impl<K> Vector<K>
  where
    // Bounds from impl:
    K: std::ops::MulAssign<K> + Copy
{
  pub fn scl(&mut self, a: K) -> Result<(), &str> {

    // if self.data.len() != 2 {
    //   return Err("Cannot scale a vector with more than 2 coordinate points.");
    // }

    for i in 0..self.data.len() {
      self.data[i] *= a;
    }

    Ok(())
  }
}

impl<K> Matrix<K> {
  pub fn new() -> Matrix<K> {
    Matrix {
      data: Vec::<Vec<K>>::new(),
      row: 0,
      col: 0,
      is_rectangular: true
    }
  }

  pub fn from(src: Vec<Vec<K>>) -> Self {
    let col_src = src[0].len();
    let row_src = src.len();
    let mut rect: bool = true;

    for i in &src {
      if i.len() != col_src {
        rect = false;
      }
    }

    Matrix {
      data: Vec::<Vec<K>>::from(src),
      col: col_src,
      row: row_src,
      is_rectangular: rect
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

    if self.is_rectangular == false || v.is_rectangular == false {
      return Err("Cannot add a matrice that's not rectangular.");
    }

    for r in 0..self.row {
      for c in 0..self.col {
        self.data[r][c] += v.data[r][c];
      }
    } 

    Ok(())
  }
}

impl<K> Matrix<K>
  where
    // Bounds from impl:
    K: std::ops::SubAssign + Copy
{
  pub fn sub(&mut self, v: &Matrix<K>) -> Result<(), &str> {
    if self.col != v.col {
      return Err("Cannot add 2 matrices with different number of columns.");
    }
    if self.row != v.row {
      return Err("Cannot add 2 matrices with different number of rows.");
    }

    if self.is_rectangular == false || v.is_rectangular == false {
      return Err("Cannot substracte a matrice that's not rectangular.");
    }

    for r in 0..self.row {
      for c in 0..self.col {
        self.data[r][c] -= v.data[r][c];
      }
    } 

    Ok(())
  }
}

impl<K> Matrix<K>
  where
  // Bounds from impl:
    K: std::ops::MulAssign + Copy
{
  pub fn scl(&mut self, a: K) -> Result<(), &str> {
    if self.is_rectangular == false {
      return Err("Cannot scale a matrice that's not rectangular.");
    }

    for r in 0..self.row {
      for c in 0..self.col {
        self.data[r][c] *= a;
      }
    }

    Ok(())
  }
}

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Option<Vector<K>>
  where
    K: std::ops::Mul<Output = K> + Copy + std::fmt::Debug + std::ops::Add<Output = K>
{
  if u.len() != coefs.len() {
    return None;
  }

  let lenght_test: usize = u[0].data.len();

  for i in u {
    if lenght_test != i.data.len() {
      return None;
    }
  }

  let mut result: Vector<K> = Vector::from(Vec::with_capacity(u[0].data.len()));

  for i in 0..u[0].data.len() {
    let mut sum = coefs[0] * u[0].data[i];

    for j in 1..u.len() {
      sum = sum + coefs[j] * u[j].data[i];
    }
    result.push(sum);
  }

  Some(result)
}
