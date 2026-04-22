use matrix::linear_combination;
use matrix::Vector;

fn main() {
  let v1 = Vector::from(vec![1., 2., 3.]);
  let v2 = Vector::from(vec![0., 10., -100.]);

  println!("Testing: {:?}", linear_combination(&[v1, v2], &[10., -2.]));
}
