#[derive(Debug)]
pub struct Vector<K> {
    data: Vec<K>
}

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

impl<K> Matrix<K> {
    pub fn new() -> Matrix<K> {
        Matrix {
            data: Vec::<Vec<K>>::new(),
            row: 0,
            col: 0
        }
    }

    pub fn from(src: Vec<Vec<K>>) -> Matrix<K> {
        Matrix {
            data: Vec::<Vec<K>>::from(src),
            col: src.len(),
            row: src[0].len()
        }
    }

    pub fn push() {
        
    }

    pub fn pop() {
        
    }
}
