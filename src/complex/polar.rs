use super::cartesian::CartesianFormat;
use std::f64::consts::E;
use std::f64::consts::PI;

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
        if set_with.signum() == 1.0 {
            self.modulus = set_with;
        } else {
            if 0.0 < self.argument && self.argument <= PI {
                self.argument -= PI;
            } else {
                self.argument += PI;
            }

            self.modulus = -set_with;
        }
    }

    // This updates the value of Argument.
    pub fn set_argument(&mut self, mut set_with: f64) {
        if set_with.abs() > PI {
            set_with %= PI;
        } else if set_with == -PI {
            set_with = PI;
        }

        self.argument = set_with;
    }
}

impl PolarFormat {
    pub fn add(&mut self, add_with: &PolarFormat) {
        let answer =
            CartesianFormat::new(self.real() + add_with.real(), self.imag() + add_with.imag());
        self.modulus = answer.modulus();
        self.argument = answer.argument();
    }

    pub fn reduce(&mut self, reduce_by: &PolarFormat) {
        let answer = CartesianFormat::new(
            self.real() - reduce_by.real(),
            self.imag() - reduce_by.imag(),
        );
        self.modulus = answer.modulus();
        self.argument = answer.argument();
    }

    pub fn multiply(&mut self, multiply_with: &PolarFormat) {
        self.modulus *= multiply_with.modulus;
        self.argument += multiply_with.argument;

        if self.argument.abs() > PI {
            self.argument %= PI;
        } else if self.argument == -PI {
            self.argument = PI;
        }
    }
}

impl PolarFormat {
    pub fn real(&self) -> f64 {
        self.modulus * self.argument.cos()
    }

    pub fn imag(&self) -> f64 {
        self.modulus * self.argument.sin()
    }

    pub fn transform(&self) -> CartesianFormat {
        CartesianFormat::new(self.real(), self.imag())
    }

    pub fn exponentiation(&mut self, index: &PolarFormat) {
        let answer = PolarFormat::new(
            (index.real() * self.modulus.log(E) - self.argument * index.imag()).exp(),
            self.argument * index.real() + index.imag() * self.modulus.log(E),
        );

        self.modulus = answer.modulus;
        self.argument = answer.argument;
    }
}

// Associated Functions for { creating Instances, defining Complex Number Operations }
impl PolarFormat {
    pub fn new(modulus: f64, mut argument: f64) -> PolarFormat {
        if argument.abs() > PI {
            argument %= PI;
        } else if argument == -PI {
            argument = PI;
        }

        if modulus.signum() == 1.0 {
            return PolarFormat { modulus, argument };
        } else {
            if 0.0 < argument && argument <= PI {
                argument -= PI;
            } else {
                argument += PI;
            }

            return PolarFormat {
                modulus: -modulus,
                argument,
            };
        }
    }

    pub fn addition_of(num1: &PolarFormat, num2: &PolarFormat) -> PolarFormat {
        let answer = CartesianFormat::new(num1.real() + num2.real(), num1.imag() + num2.imag());

        PolarFormat::new(answer.modulus(), answer.argument())
    }

    pub fn subtraction_of(num1: &PolarFormat, num2: &PolarFormat) -> PolarFormat {
        let answer = CartesianFormat::new(num1.real() - num2.real(), num1.imag() - num2.imag());

        PolarFormat::new(answer.modulus(), answer.argument())
    }

    pub fn multiplication_of(num1: &PolarFormat, num2: &PolarFormat) -> PolarFormat {
        PolarFormat::new(num1.modulus * num2.modulus, num1.argument + num2.argument)
    }

    pub fn exponentiation_of(base: &PolarFormat, power: &PolarFormat) -> PolarFormat {
        PolarFormat::new(
            (power.real() * base.modulus.log(E) - base.argument * power.imag()).exp(),
            base.argument * power.real() + power.imag() * base.modulus.log(E),
        )
    }
}
