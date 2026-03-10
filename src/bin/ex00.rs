use std::vec;

use matrix;

fn main() {
  let new: matrix::Vector<i32> = matrix::Vector::new();
  println!("{:?}", new);

  let mut from: matrix::Vector<i32> = matrix::Vector::from(vec![1, 2]);
  println!("{:?}", from);

  from.push(4);
  println!("{:?}", from);

  from.pop();
  println!("{:?}", from);


  let mat_1: matrix::Matrix<i32> = matrix::Matrix::from(vec![
    vec![1, 2],
    vec![3, 4],
  ]);

  println!("{:?}", mat_1);

  let mat_2: matrix::Matrix<i32> = matrix::Matrix::from(vec![
    vec![5, 6],
    vec![7, 8]
  ]);

  
}
