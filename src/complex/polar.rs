#[derive(Debug, Clone, Copy)]
pub struct PolarFormat {
    modulus: f64,
    argument: f64,
}

// getters and setters
impl PolarFormat {
  // Returns the value of modulus of a Complex Number in PolarFormat.
  pub fn modulus(&self) -> f64 {
      self.modulus
  }

  // Returns the value of Argument of a Complex Number in PolarFormat.
  pub fn argument(&self) -> f64 {
      self.argument
  }

  // This updates the value of Modulus.
  pub fn set_modulus(&mut self, set_with: f64) {
      self.modulus = set_with;
  }

  // This updates the value of Argument.
  pub fn set_argument(&mut self, set_with: f64) {
      self.argument = set_with;
  }
}