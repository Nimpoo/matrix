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

impl<K: std::ops::Add<Output = K> + Copy> Vector<K> {
  pub fn add(&mut self, v: &Vector<K>) -> Result<(), &str> {

    if v.data.len() > 2 {
      return Err("Cannot add 2 vectors themselves: one of them has more than 2 coordinate points.");
    }

    let v1 = self.data[0] + v.data[0];
    let v2 = self.data[1] + v.data[1];

    self.data[0] = v1;
    self.data[1] = v2;

    Ok(())
  }
}

impl<K: std::ops::Sub<Output = K> + Copy> Vector<K> {
  pub fn sub(&mut self, v: &Vector<K>) -> Result<(), &str> {

    if v.data.len() > 2 {
      return Err("Cannot substract 2 vectors themselves: one of them has more than 2 coordinate points.");
    }

    let v1 = self.data[0] - v.data[0];
    let v2 = self.data[1] - v.data[1];

    self.data[0] = v1;
    self.data[1] = v2;

    Ok(())
  }
}

impl<K: std::ops::Mul<Output = K> + Copy> Vector<K> {
  pub fn scl(&mut self, a: K) -> Result<(), &str> {

    if self.data.len() > 2 {
      return Err("Cannot scale a vector with more than 2 coordinate points.");
    }

    self.data[0] = self.data[0] * a;
    self.data[1] = self.data[0] * a;

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

  pub fn add(&mut self, _v: &Matrix<K>) {
    
  }
}
