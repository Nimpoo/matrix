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
}
