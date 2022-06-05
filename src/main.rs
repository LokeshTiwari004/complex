use complex::complex::cartesian::CartesianFormat;

fn main() {
  let num = CartesianFormat::new(0.0, 1.0);

  println!("{}", num.argument());
}
