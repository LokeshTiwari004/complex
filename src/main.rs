use complex::complex::ComplexNumber;

fn main() {
  let mut z1 = ComplexNumber::new(2.0, 1.0);
  let z2 = ComplexNumber::new(2.0, 1.0);

  println!("z is {:?}", ComplexNumber::addition_of(&z1, &z2));

  z1.add(&z2);
  println!("z is {:?}", z1);

}
