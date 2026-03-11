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

impl<K: std::ops::Add<Output = K> + Copy> Vector<K> {
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

  pub fn add(&mut self, v: &Vector<K>) {
      let v1 = self.data[0] + v.data[0];
      let v2 = self.data[1] + v.data[1];
      
      self.data[0] = v1;
      self.data[1] = v2;
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
