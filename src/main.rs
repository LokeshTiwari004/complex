use std::time::Instant;

use complex::complex::ComplexNumber;

fn main() {
  let mut num1 = ComplexNumber::cartesian(21.0, 19.0);
  let num2 = ComplexNumber::cartesian(10.0, 17.0);

  let start1 = Instant::now();
  let i_i1 = ComplexNumber::exponentiation_of(&num1, &num1);
  let duration1 = start1.elapsed();

  let start3 = Instant::now();
  num1.exponentiation(&num2);
  let duration3 = start3.elapsed();
  


  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i1, duration1);
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", &num1, duration3);
}
