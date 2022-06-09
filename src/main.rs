extern crate ndarray;

use std::time::Instant;
use ndarray::Array3;
use std::f64::consts::PI;
use complex::complex::ComplexNumber;
use image::RgbImage;

fn main() {
  // let start = Instant::now();
  // for i in 1..10 {
  //   create_frames(0.01 * i as f64, format!("00{}", i).as_str());
  // }
  // for i in 10..100 {
  //   create_frames(0.01 * i as f64, format!("0{}", i).as_str());
  // }
  // for i in 100..201 {
  //   create_frames(0.01 * i as f64, format!("{}", i).as_str());
  // }
  // let duration = start.elapsed();
  // println!("Time taken to generate image sequence is {:?}", &duration);
  create_frames(0.5, "001");
}

fn iterate(z: &mut ComplexNumber, c: &ComplexNumber) {
  z.squared();
  z.add(c);
}

fn create_julia_set(array: &mut Array3<u8>, factor: f64) {
  let max_iterations = (100.0 * factor.powf(0.25)) as i32;

  for y in 0..800 {
    for x in 0..800 {
      let mut z = ComplexNumber::cartesian(1.65 * (((x as f64) - 400.0)/400.0) , 1.65 * ((400.0 - (y as f64))/400.0));
      // let c = ComplexNumber::polar(0.7885, PI * factor);
      let c = ComplexNumber::polar(0.79, PI * factor);

      let mut num_of_iter = max_iterations;
      for i in 0..max_iterations {
        iterate(&mut z, &c);
        if z.modulus() > 80.0 {
          num_of_iter = i;
        }
      };

      if num_of_iter < 11 {
        array[[y, x, 0]] = 17;
        array[[y, x, 1]] = 45;
        array[[y, x, 2]] = 77;
      } else if num_of_iter < 21 {
        num_of_iter -= 10;
        let factor = (num_of_iter as f64) / 10.0;
        array[[y, x, 0]] = 19 + (factor * 17.0) as u8;
        array[[y, x, 1]] = 70 + (factor * 70.0) as u8;
        array[[y, x, 2]] = 94 + (factor * 95.0) as u8;
      } else if num_of_iter < 51 {
        num_of_iter -= 20;
        let factor = (num_of_iter as f64) / 30.0;
        array[[y, x, 0]] = 41 + (factor * 201.0) as u8;
        array[[y, x, 1]] = 31 + (factor * 151.0) as u8;
        array[[y, x, 2]] = 7 + (factor * 29.0) as u8;
      } else if num_of_iter < 71 {
        num_of_iter -= 50;
        let factor = (num_of_iter as f64) / 20.0;
        array[[y, x, 0]] = 110 + (factor * 102.0) as u8;
        array[[y, x, 1]] = 26 + (factor * 23.0) as u8;
        array[[y, x, 2]] = 16 + (factor * 14.0) as u8;
      } else if num_of_iter < 81 {
        num_of_iter -= 70;
        let factor = (num_of_iter as f64) / 10.0;
        array[[y, x, 0]] = 103 + (factor * 60.0) as u8;
        array[[y, x, 1]] = 138 + (factor * 81.0) as u8;
        array[[y, x, 2]] = 22 + (factor * 11.0) as u8;
      } else {
        array[[y, x, 0]] = 0;
        array[[y, x, 1]] = 0;
        array[[y, x, 2]] = 0;
      }
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

fn create_frames(factor: f64, number: &str) {
  let mut array = Array3::<u8>::zeros((800, 800, 3));

  create_julia_set(&mut array, factor);
  array_to_image(array).save(format!("../img_seq1/julia_set.{}.png", number)).expect("opps!");
}