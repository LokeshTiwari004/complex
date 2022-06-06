use complex::complex::ComplexNumber;

fn main() {
  let num = ComplexNumber::cartesian(0.0, 1.0);

  println!("{:#?}", ComplexNumber::exponentiation_of(&num, &num, true));
}
