use std::f64::consts::PI;
use std::f64::consts::E;

#[derive(Debug, Clone, Copy)]
pub struct ComplexNumber {
    real: f64,
    imag: f64
}

impl ComplexNumber {
    pub fn cartesian(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber {
            real,
            imag
        }
    }

    pub fn polar(modulus: f64, argument: f64) -> ComplexNumber {
        ComplexNumber {
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
}

impl ComplexNumber {
    pub fn set_real(&mut self, set_with: f64) {
      self.real = set_with;
    }

    pub fn set_imag(&mut self, set_with: f64) {
      self.imag = set_with;
    }
}


impl ComplexNumber {
  pub fn modulus(&self) -> f64 {
    (self.real.powi(2) + self.imag.powi(2)).sqrt()
  }

  pub fn argument(&self) -> f64 {
    let mut argument = (self.imag / self.real).atan();
    if self.real.signum() == -1.0 {
        if argument.signum() == 1.0 {
          argument -= PI;
        } else {
          argument += PI;
        }
    };
    argument
  }

  
  pub fn add(&mut self, add_with: &ComplexNumber) {
    self.real += add_with.real;
    self.imag += add_with.imag;
  }
  
  pub fn subtract(&mut self, subtract_with: &ComplexNumber) {
    self.real -= subtract_with.real;
    self.imag -= subtract_with.imag;
  }

  pub fn multiply(&mut self, multiply_with: &ComplexNumber) {
    let real = self.real * multiply_with.real - self.imag * multiply_with.imag;
    let imag = self.real * multiply_with.imag + self.imag * multiply_with.real;

    self.real = real;
    self.imag = imag;
  }

  pub fn log(&mut self, base: f64) {
    let argument = self.argument();
    let modulus = self.modulus();

    self.real = modulus.log(base);
    self.imag = argument * E.log(base);
  }

  pub fn exp(&mut self, index: &ComplexNumber) {
    let imag = self.argument();
    let real = self.modulus().log(E);

    let ans_modulus = (index.real * real - imag * index.imag).exp();
    let ans_argument = index.real * imag + real * index.imag;

    self.real = ans_modulus * ans_argument.cos();
    self.imag = ans_modulus * ans_argument.sin();

    // self.log(E);
    // self.multiply(index);

    // let ans_mod = self.real.exp();
    // let ans_argument = self.imag;

    // self.real = ans_mod * ans_argument.cos();
    // self.imag = ans_mod * ans_argument.sin();
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

    pub fn exp_of(base: &ComplexNumber, power: &ComplexNumber) -> ComplexNumber {
      let mut ans = base.clone();
      ans.exp(power);
      ans
    }

    pub fn log_of(num: &ComplexNumber, base: f64) -> ComplexNumber {
      let mut ans = num.clone();
      ans.log(base);
      ans
    }
}