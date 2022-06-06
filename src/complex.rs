mod cartesian;
mod polar;

use self::{cartesian::CartesianFormat, polar::PolarFormat};


#[derive(Debug, Clone, Copy)]
pub enum ComplexNumber {
    CF(cartesian::CartesianFormat),
    PF(polar::PolarFormat),
}

impl ComplexNumber {
    pub fn cartesian(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber::CF(cartesian::CartesianFormat::new(real, imag))
    }

    pub fn polar(modulus: f64, argument: f64) -> ComplexNumber {
        ComplexNumber::PF(polar::PolarFormat::new(modulus, argument))
    }

    pub fn convert(&self) -> ComplexNumber {
        match self {
            ComplexNumber::CF(number) => {
                ComplexNumber::PF(number.transform())
            }
            ComplexNumber::PF(number) => {
                ComplexNumber::CF(number.transform())
            }
        }
    }

    pub fn add(&mut self, add_with: &ComplexNumber) {
        match self {
            ComplexNumber::CF(number1) => {
                match add_with {
                    ComplexNumber::CF(number2) => number1.add(number2),
                    ComplexNumber::PF(number2) => number1.add(&number2.transform()),
                }
            },
            ComplexNumber::PF(number1) => {
                match add_with {
                    ComplexNumber::CF(number2) => number1.add(&number2.transform()),
                    ComplexNumber::PF(number2) => number1.add(number2),
                }
                
            },
        };
    }

    pub fn reduce(&mut self, reduce_by: &ComplexNumber) {
        match self {
            ComplexNumber::CF(number1) => {
                match reduce_by {
                    ComplexNumber::CF(number2) => number1.reduce(number2),
                    ComplexNumber::PF(number2) => number1.reduce(&number2.transform()),
                }
            },
            ComplexNumber::PF(number1) => {
                match reduce_by {
                    ComplexNumber::CF(number2) => number1.reduce(&number2.transform()),
                    ComplexNumber::PF(number2) => number1.reduce(number2),
                }
                
            },
        };
    }

    pub fn multiply(&mut self, multiply_with: &ComplexNumber) {
        match self {
            ComplexNumber::CF(number1) => {
                match multiply_with {
                    ComplexNumber::CF(number2) => number1.multiply(number2),
                    ComplexNumber::PF(number2) => number1.multiply(&number2.transform()),
                }
            },
            ComplexNumber::PF(number1) => {
                match multiply_with {
                    ComplexNumber::CF(number2) => number1.multiply(&number2.transform()),
                    ComplexNumber::PF(number2) => number1.multiply(number2),
                }
                
            },
        };
    }

    pub fn exponentiation(&mut self, index: &ComplexNumber) {
        match self {
            ComplexNumber::CF(base) => {
                match index {
                    ComplexNumber::CF(index) => base.exponentiation(index),
                    ComplexNumber::PF(index) => base.exponentiation(&index.transform()),
                }
            },
            ComplexNumber::PF(base) => {
                match index {
                    ComplexNumber::CF(index) => base.exponentiation(&index.transform()),
                    ComplexNumber::PF(index) => base.exponentiation(index)
                }
            }
        };
    }

    pub fn addition_of(number1: &ComplexNumber, number2: &ComplexNumber, in_cf: bool) -> ComplexNumber {
        match number1 {
            ComplexNumber::CF(num1) => {
                match number2 {
                    ComplexNumber::CF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::addition_of(num1, num2)),
                            false => ComplexNumber::PF(CartesianFormat::addition_of(num1, num2).transform())
                        }
                    },
                    ComplexNumber::PF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::addition_of(num1, &num2.transform())),
                            false => ComplexNumber::PF(PolarFormat::addition_of(&num1.transform(), num2))
                        }
                    },
                }
            },
            ComplexNumber::PF(num1) => {
                match number2 {
                    ComplexNumber::CF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::addition_of(&num1.transform(), num2)),
                            false => ComplexNumber::PF(PolarFormat::addition_of(num1, &num2.transform()))
                        }
                    },
                    ComplexNumber::PF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(PolarFormat::addition_of(num1, num2).transform()),
                            false => ComplexNumber::PF(PolarFormat::addition_of(num1, num2))
                        }
                    }
                }
            }
        }
    }


    pub fn subtraction_of(number1: &ComplexNumber, number2: &ComplexNumber, in_cf: bool) -> ComplexNumber {
        match number1 {
            ComplexNumber::CF(num1) => {
                match number2 {
                    ComplexNumber::CF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::subtraction_of(num1, num2)),
                            false => ComplexNumber::PF(CartesianFormat::subtraction_of(num1, num2).transform())
                        }
                    },
                    ComplexNumber::PF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::subtraction_of(num1, &num2.transform())),
                            false => ComplexNumber::PF(PolarFormat::subtraction_of(&num1.transform(), num2))
                        }
                    },
                }
            },
            ComplexNumber::PF(num1) => {
                match number2 {
                    ComplexNumber::CF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::subtraction_of(&num1.transform(), num2)),
                            false => ComplexNumber::PF(PolarFormat::subtraction_of(num1, &num2.transform()))
                        }
                    },
                    ComplexNumber::PF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(PolarFormat::subtraction_of(num1, num2).transform()),
                            false => ComplexNumber::PF(PolarFormat::subtraction_of(num1, num2))
                        }
                    }
                }
            }
        }
    }


    pub fn multiplication_of(number1: &ComplexNumber, number2: &ComplexNumber, in_cf: bool) -> ComplexNumber {
        match number1 {
            ComplexNumber::CF(num1) => {
                match number2 {
                    ComplexNumber::CF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::multiplication_of(num1, num2)),
                            false => ComplexNumber::PF(CartesianFormat::multiplication_of(num1, num2).transform())
                        }
                    },
                    ComplexNumber::PF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::multiplication_of(num1, &num2.transform())),
                            false => ComplexNumber::PF(PolarFormat::multiplication_of(&num1.transform(), num2))
                        }
                    },
                }
            },
            ComplexNumber::PF(num1) => {
                match number2 {
                    ComplexNumber::CF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::multiplication_of(&num1.transform(), num2)),
                            false => ComplexNumber::PF(PolarFormat::multiplication_of(num1, &num2.transform()))
                        }
                    },
                    ComplexNumber::PF(num2) => {
                        match in_cf {
                            true => ComplexNumber::CF(PolarFormat::multiplication_of(num1, num2).transform()),
                            false => ComplexNumber::PF(PolarFormat::multiplication_of(num1, num2))
                        }
                    }
                }
            }
        }
    }    

    pub fn exponentiation_of(base: &ComplexNumber, power: &ComplexNumber, in_cf: bool) -> ComplexNumber {
        match base {
            ComplexNumber::CF(base) => {
                match power {
                    ComplexNumber::CF(power) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::exponentiation_of(base, power)),
                            false => ComplexNumber::PF(CartesianFormat::exponentiation_of(base, power).transform())
                        }
                    },
                    ComplexNumber::PF(power) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::exponentiation_of(base, &power.transform())),
                            false => ComplexNumber::PF(PolarFormat::exponentiation_of(&base.transform(), power))
                        }
                    }
                }
            },
            ComplexNumber::PF(base) => {
                match power {
                    ComplexNumber::CF(power) => {
                        match in_cf {
                            true => ComplexNumber::CF(CartesianFormat::exponentiation_of(&base.transform(), power)),
                            false => ComplexNumber::PF(PolarFormat::exponentiation_of(base, &power.transform()))
                        }
                    },
                    ComplexNumber::PF(power) => {
                        match in_cf {
                            true => ComplexNumber::CF(PolarFormat::exponentiation_of(base, power).transform()),
                            false => ComplexNumber::PF(PolarFormat::exponentiation_of(base, power))
                        }
                    }
                }
            }
        }
    }

}
