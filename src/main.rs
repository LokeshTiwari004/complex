use std::time::Instant;

use complex::complex::ComplexNumber;

fn main() {
  let num3 = ComplexNumber::cartesian(0.0, 1.0);
  let start3 = Instant::now();
  let i_i3 = ComplexNumber::exp_of(&num3, &num3);
  let duration3 = start3.elapsed();
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i3, duration3);

  let num1 = ComplexNumber::cartesian(0.0, 1.0);
  let start1 = Instant::now();
  let i_i1 = ComplexNumber::exp_of(&num1, &num1);
  let duration1 = start1.elapsed();
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i1, duration1);
}
