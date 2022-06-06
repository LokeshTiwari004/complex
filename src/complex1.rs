use std::f64::consts::PI;
use std::f64::consts::E;

#[derive(Debug, Clone, Copy)]
pub struct ComplexNumber {
    real: f64,
    imag: f64,
    modulus: f64,
    argument: f64,
}

impl ComplexNumber {
    pub fn cartesian(real: f64, imag: f64) -> ComplexNumber {
        let mut argument = (imag / real).atan();
        if real.signum() == -1.0 {
            if argument.signum() == 1.0 {
              argument -= PI;
            } else {
              argument += PI;
            }
        };

        ComplexNumber {
            real,
            imag,
            modulus: (real.powi(2) + imag.powi(2)).sqrt(),
            argument,
        }
    }

    pub fn polar(modulus: f64, argument: f64) -> ComplexNumber {
        ComplexNumber {
            modulus,
            argument,
            real: modulus * argument.cos(),
            imag: modulus * argument.sin(),
        }
    }
}

impl ComplexNumber {
    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }

    pub fn modulus(&self) -> f64 {
        self.modulus
    }

    pub fn argument(&self) -> f64 {
        self.argument
    }
}

impl ComplexNumber {
    pub fn set_real(&mut self, set_with: f64) {
      self.real = set_with;
      self.modulus = (self.imag.powi(2) + self.real.powi(2)).sqrt();

      let mut argument = (self.imag / self.real).atan();
      if self.real.signum() == -1.0 {
          if argument.signum() == 1.0 {
            argument -= PI;
          } else {
            argument += PI;
          }
      };
      self.argument = argument;
    }

    pub fn set_imag(&mut self, set_with: f64) {
      self.imag = set_with;
      self.modulus = (self.imag.powi(2) + self.real.powi(2)).sqrt();
      
      let mut argument = (self.imag / self.real).atan();
      if self.real.signum() == -1.0 {
          if argument.signum() == 1.0 {
            argument -= PI;
          } else {
            argument += PI;
          }
      };
      self.argument = argument;
    }

    pub fn set_modulus(&mut self, set_with: f64) {
      self.modulus = set_with;
      self.real = self.modulus * self.argument.cos();
      self.imag = self.modulus * self.argument.sin()
    }
    
    pub fn set_argument(&mut self, set_with: f64) {
      self.argument = set_with;
      self.real = self.modulus * self.argument.cos();
      self.imag = self.modulus * self.argument.sin()
    }
}


impl ComplexNumber {
  pub fn add(&mut self, add_with: &ComplexNumber) {
    self.real += add_with.real;
    self.imag += add_with.imag;
    
    let mut argument = (self.imag / self.real).atan();
    if self.real.signum() == -1.0 {
        if argument.signum() == 1.0 {
          argument -= PI;
        } else {
          argument += PI;
        }
    };
    self.argument = argument;
    self.modulus = (self.imag.powi(2) + self.real.powi(2)).sqrt();
  }
  
  pub fn subtract(&mut self, subtract_with: &ComplexNumber) {
    self.real -= subtract_with.real;
    self.imag -= subtract_with.imag;
    
    let mut argument = (self.imag / self.real).atan();
    if self.real.signum() == -1.0 {
        if argument.signum() == 1.0 {
          argument -= PI;
        } else {
          argument += PI;
        }
    };
    self.argument = argument;
    self.modulus = (self.imag.powi(2) + self.real.powi(2)).sqrt();
  }

  pub fn multiply(&mut self, multiply_with: &ComplexNumber) {
    self.modulus *= multiply_with.modulus;
    self.argument += multiply_with.argument;

    self.real = self.modulus * self.argument.cos();
    self.imag = self.modulus * self.argument.sin();
  }

  pub fn exponentiation(&mut self, index: &ComplexNumber) {
    let argument = if self.argument.abs() > PI {
        self.argument % PI 
      } else {
        PI
      };
    
    let r = self.modulus.log(E);

    self.modulus = (index.real * r - argument * index.imag).exp();
    self.argument = index.real * argument + r * index.imag;

    self.real = self.modulus * self.argument.cos();
    self.imag = self.modulus * self.argument.sin();
  }
}

impl ComplexNumber {
    pub fn addition_of(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
      let mut ans = a.clone();
      ans.add(b);
      ans
    }

    pub fn subtraction_of(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
      let mut ans = a.clone();
      ans.subtract(b);
      ans
    }
    
    pub fn multiplication_of(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
      let mut ans = a.clone();
      ans.multiply(b);
      ans
    }

    pub fn exponentiation_of(base: &ComplexNumber, power: &ComplexNumber) -> ComplexNumber {
      let mut ans = base.clone();
      ans.exponentiation(power);
      ans
    }
}