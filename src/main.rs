extern crate ndarray;

use ndarray::Array3;
use std::time::Instant;
use complex::complex::ComplexNumber;
use image::RgbImage;

fn main() {
  let mut array = Array3::<u8>::zeros((1080, 1920, 3));
  // let mut image = [[[0_u8; 3];1920]; 1080];

  let start = Instant::now();
  create(&mut array);
  array_to_image(array).save("mandelbrot.png").expect("opps!");
  let duration = start.elapsed();
  println!("Time taken to create mandelbrot is {:?}", &duration);
}

fn iterate(z: &mut ComplexNumber, c: &ComplexNumber) {
  z.squared();
  z.add(c);
}

fn create(array: &mut Array3<u8>) {
  let max_iterations = 100;

  for x in 0..1080 {
    for y in 0..1920 {
      let c = ComplexNumber::cartesian(3.0 * (((x as f64) - 540.0)/1080.0), 3.0 * ((960.0 - (y as f64))/1920.0));
      let mut z = ComplexNumber::cartesian(0.0, 0.0);

      let mut num_of_iter = max_iterations;
      for i in 0..max_iterations {
        iterate(&mut z, &c);
        if z.modulus() > 20.0 {
          num_of_iter = i ;
        }
      };

      let factor = ((num_of_iter as f64) / (max_iterations as f64)).powf(0.5);
      let color = 255 - (factor * 255.0) as u8;
      array[[x, y, 0]] = color;
      array[[x, y, 1]] = color;
      array[[x, y, 2]] = color;
    }
  }
}


fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}