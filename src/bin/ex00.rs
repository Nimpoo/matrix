use std::vec;

use matrix::{Vector, Matrix};

fn main() {

  // ? VECTOR TESTS
  println!("|-------- Vector tests --------|");

  println!();

  // * ***************************************************************************
  // * VECTOR - BASIC FUNCTIONS
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
  // * ***************************************************************************

  // * VECTOR - MANADATORY FUNCTIONS
  println!("~ Mandatory functions");

  println!();

  println!("1.1) `vector_1.add(&vector_2)`: Right example");
  println!("`vector_1` = {:?}", vector_1);

  let vector_2: Vector<i32> = Vector::from(vec![1, 2]);
  println!("`vector_2` = {:?}", vector_2);

  let addition_vector_1: Result<(), &str> = vector_1.add(&vector_2);
  println!();

  match addition_vector_1 {
    Ok(()) => {
      println!("The `vector_2` is correctly added to the `vector_1`.");
      println!("variable `vector_1` is now = [ {:?} ]", vector_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("1.2) `vector_1.add(&vector_incorrect)`: Erraneous example");
  println!("`vector_1` = {:?}", vector_1);

  let vector_incorrect: Vector<i32> = Vector::from(vec![1, 2, 3]);
  println!("`vector_incorrect` = {:?}", vector_incorrect);
  
  let addition_vector_1: Result<(), &str> = vector_1.add(&vector_incorrect);
  println!();

  match addition_vector_1 {
    Ok(()) => {
      println!("The `vector_incorrect` is correctly added to the `vector_1`.");
      println!("variable `vector_1` is now = [ {:?} ]", vector_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("2.1) `vector_1.sub(&vector_2)`: Right example");
  println!("`vector_1` = {:?}", vector_1);

  println!("`vector_2` = {:?}", vector_2);

  let substraction_vector_1: Result<(), &str> = vector_1.sub(&vector_2);
  println!();

  match substraction_vector_1 {
    Ok(()) => {
      println!("The `vector_2` is correctly substracted to the `vector_1`.");
      println!("variable `vector_1` is now = [ {:?} ]", vector_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("2.2) `vector_1.sub(&vector_incorrect)`: Erraneous example");
  println!("`vector_1` = {:?}", vector_1);

  let vector_incorrect: Vector<i32> = Vector::from(vec![1, 2, 3]);
  println!("`vector_incorrect` = {:?}", vector_incorrect);
  
  let addition_vector_1: Result<(), &str> = vector_1.sub(&vector_incorrect);
  println!();

  match addition_vector_1 {
    Ok(()) => {
      println!("The `vector_incorrect` is correctly substracted to the `vector_1`.");
      println!("variable `vector_1` is now = [ {:?} ]", vector_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("3.1) `vector_1.scl(int)`: Right example");
  println!("`vector_1` = {:?}", vector_1);

  let int: i32 = 2;
  println!("`int` = [ {} ]", int);

  let scalar_vector_1: Result<(), &str> = vector_1.scl(int);

  println!();

  match scalar_vector_1 {
    Ok(()) => {
      println!("The `vector_1` is correctly scaled to `int` (`int` = {}).", int);
      println!("variable `vector_1` is now = [ {:?} ]", vector_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("3.2) `vector_incorrect.scl(int)`: Erraneous example");
  let mut vector_incorrect: Vector<i32> = Vector::from(vec![1, 2, 3]);
  println!("`vector_incorrect` = {:?}", vector_incorrect);
  
  println!("`int` = [ {} ]", int);

  let scalar_vector_incorrect: Result<(), &str> = vector_incorrect.scl(int);
  println!();

  match scalar_vector_incorrect {
    Ok(()) => {
      println!("The `vector_incorrect` is correctly scaled to `int` (`int` = {}).", int);
      println!("variable `vector_1` is now = [ {:?} ]", vector_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();
  println!();
  // * ***************************************************************************

  // ? MATRIX TESTS
  println!("|-------- Matrix tests --------|");

  println!();

  // * MATRIX - BASIC FUNCTIONS
  println!("~ Basic functions");

  println!();

  println!("1) `let matrix_1: Matrix<i32> = Matrix::new();`");
  let matrix_1: Matrix<i32> = Matrix::new();
  println!("variable `matrix_1` = {:?}", matrix_1);

  println!("2) `let matrix_1: Matrix<i32> = Matrix::from(vec![\n      vec![1, 2],\n      vec![3, 4],\n    ]);`");
  let matrix_1: Matrix<i32> = Matrix::from(vec![
    vec![1, 2],
    vec![3, 4],
  ]);
  println!("variable `matrix_1` = {:?}", matrix_1);

  println!();
  // * ***************************************************************************

  // * MATRIX - MANADATORY FUNCTIONS
  println!("~ Mandatory functions");

  println!();

  println!("1.1) `matrix_1.add(&matrix_2)`: Right example");
  let mut matrix_1: Matrix<i32> = Matrix::from(vec![
    vec![1, 2],
    vec![3, 4],
  ]);
  println!("variable `matrix_1` = {:?}", matrix_1);

  let matrix_2: Matrix<i32> = Matrix::from(vec![
    vec![1, 1],
    vec![1, 1],
  ]);
  println!("variable `matrix_2` = {:?}", matrix_2);

  let addition_matrix_1: Result<(), &str> = matrix_1.add(&matrix_2);
  println!();

  match addition_matrix_1 {
    Ok(()) => {
      println!("The `matrix_2` is correctly added to `matrix_1`.");
      println!("The variable `matrix_1` is now = [ {:?} ]", matrix_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("1.2) `matrix_1.add(&matrix_erranous)`: Erraneous example");
  println!("variable `matrix_1` = {:?}", matrix_1);

  let matrix_erranous: Matrix<i32> = Matrix::from(vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
  ]);
  println!("variable `matrix_erranous` = {:?}", matrix_erranous);

  let addition_matrix_1: Result<(), &str> = matrix_1.add(&matrix_erranous);
  println!();

  match addition_matrix_1 {
    Ok(()) => {
      println!("The `matrix_erranous` is correctly added to `matrix_1`.");
      println!("The variable `matrix_1` is now = [ {:?} ]", matrix_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("2.1) `matrix_1.sub(&matrix_2)`: Right example");
  println!("variable `matrix_1` = {:?}", matrix_1);

  println!("variable `matrix_2` = {:?}", matrix_2);

  let substraction_matrix_1: Result<(), &str> = matrix_1.sub(&matrix_2);
  println!();

  match substraction_matrix_1 {
    Ok(()) => {
      println!("The `matrix_2` is correctly substracted to `matrix_1`.");
      println!("The variable `matrix_1` is now = [ {:?} ]", matrix_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("2.2) `matrix_1.sub(&matrix_erranous)`: Erraneous example");
  println!("variable `matrix_1` = [ {:?} ]", matrix_1);

  println!("variable `matrix_erranous` = [ {:?} ]", matrix_erranous);

  let substraction_matrix_1: Result<(), &str> = matrix_1.sub(&matrix_erranous);
  println!();

  match substraction_matrix_1 {
    Ok(()) => {
      println!("The `matrix_erranous` is correctly substracted to `matrix_1`.");
      println!("The variable `matrix_1` is now = [ {:?} ]", matrix_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("3.1) `matrix_1.scl(int)`: Right example");
  println!("variable `matrix_1` = [ {:?} ]", matrix_1);

  let int: i32 = 2;
  println!("`int` = [ {} ]", int);

  let scalar_matrix_1: Result<(), &str> = matrix_1.scl(int);
  println!();

  match scalar_matrix_1 {
    Ok(()) => {
      println!("The `matrix_1` is correctly scaled to `int` (`int` = {}).", int);
      println!("The variable `matrix_1` is now = [ {:?} ]", matrix_1);
    },
    Err(msg) => println!("{}", msg),
  }
  println!();

  println!("3.2) `erraneous_matrix.scl(int)`: Erraneous example");
  let mut erraneous_matrix: Matrix<i32> = Matrix::from(vec![
    vec![1, 2],
    vec![],
  ]);
  println!("variable `erraneous_matrix` = [ {:?} ]", erraneous_matrix);

  let scalar_matrix_erraneous: Result<(), &str> = erraneous_matrix.scl(int);
  println!();

  match scalar_matrix_erraneous {
    Ok(()) => {
      println!("The `erraneous_matrix` is correctly scaled to `int` (`int` = {}).", int);
      println!("The variable `erraneous_matrix` is now = [ {:?} ]", erraneous_matrix);
    },
    Err(msg) => println!("{}", msg),
  }
  // * ***************************************************************************
}
