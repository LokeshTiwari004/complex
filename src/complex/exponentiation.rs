// This function takes a positive number and return its factorial.
fn factorial(number: u64) -> u64 {
  let mut answer = 1;
  for i in 1..(number + 1) {
    answer *= i;
  }
  answer
}

// This function calculates e raised to any real number between (0, 1]
pub fn e_raised_to(real_number: f64) -> f64 {
  let mut answer = 1.0;
  
  for i in 1..20 {
    answer += f64::powi(real_number, i) / factorial(i as u64) as f64;
  }

  answer
}