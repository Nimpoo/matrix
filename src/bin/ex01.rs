use matrix::linear_combination;
use matrix::Vector;

fn main() {
  let v1 = Vector::from(vec![1., 2., 3.]);
  let v2 = Vector::from(vec![0., 10., -100.]);

  let res: Option<Vector<f32>> = linear_combination(&[v1, v2], &[10., -2.]);

  println!("Testing: {:?}", res);
}
