use std::vec;

use matrix::{Vector, Matrix};

fn main() {
  println!("|-------- Vector tests --------|");

  println!();

  println!("~ Basic functions");

  println!();

  println!("1) `let vector_1: Vector<i32> = Vector::new()`");
  let vector_1: Vector<i32> = Vector::new();
  println!("variable `vector_1` = [ {:?} ]", vector_1);

  println!();

  println!("2) `let mut vector_1: Vector<i32> = Vector::from(4)`");
  let mut vector_1: Vector<i32> = Vector::from(vec![4, 2]);
  println!("variable `vector_1` = [ {:?} ]", vector_1);

  println!();

  println!("3) `vector_1.push(1)`");
  vector_1.push(1);
  println!("Value `1` pushed in variable `vector_1` = [ {:?} ]", vector_1);

  println!();

  println!("4) `vector_1.pop()`");
  vector_1.pop();
  println!("Last value poped in variable `vector_1` = [ {:?} ]", vector_1);

  println!();

  println!("~ Mandatory functions");

  println!();

  println!("1) `vector_1.add(&vector_2)`");
  let vector_2: Vector<i32> = Vector::from(vec![1, 1]);
  let addition_vector_1: Result<(), &str> = vector_1.add(&vector_2);
  if addition_vector_1.unwrap() == {
      // TODO: do a match-case here for this test
  }
}
