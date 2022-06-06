use std::time::Instant;

use complex::complex::{ComplexNumber, cartesian::CartesianFormat, polar::PolarFormat};
use complex::complex1::ComplexNumber as cn;

fn main() {
  let num = ComplexNumber::cartesian(0.0, 1.0);
  let start = Instant::now();
  let i_i = ComplexNumber::exponentiation_of(&num, &num, true);
  let duration = start.elapsed();
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i, duration);


  let num1 = CartesianFormat::new(0.0, 1.0);
  let start1 = Instant::now();
  let i_i1 = CartesianFormat::exponentiation_of(&num1, &num1);
  let duration1 = start1.elapsed();
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i1, duration1);


  let num2 = PolarFormat::new(1.0, std::f64::consts::PI / 2.0);
  let start2 = Instant::now();
  let i_i2 = PolarFormat::exponentiation_of(&num2, &num2);
  let duration2 = start2.elapsed();
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i2.transform(), duration2);


  let num3 = cn::cartesian(0.0, 1.0);
  let start3 = Instant::now();
  let i_i3 = cn::exponentiation_of(&num3, &num3);
  let duration3 = start3.elapsed();
  println!("i^i is {:#?}. \nTime Elapsed = {:?}", i_i3, duration3);
}
