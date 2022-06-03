pub mod complex {
    #[derive(Debug, Clone, Copy)]
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    // getters and setters
    impl ComplexNumber {
        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn set_real(&mut self, set_with: f64) {
            self.real = set_with;
        }

        pub fn set_imag(&mut self, set_with: f64) {
            self.imag = set_with;
        }
    }

    // Methods defining Operations with Complex Numbers 
    impl ComplexNumber {
        pub fn add(&mut self, add_with: &ComplexNumber) {
            self.real += add_with.real;
            self.imag += add_with.imag;
        }
    }

    // Associated Functions for { creating Instances, defining Complex Number Operations}
    impl ComplexNumber {
        pub fn new(real: f64, imag: f64) -> ComplexNumber {
            ComplexNumber { real, imag }
        }

        pub fn addition_of(num1: &ComplexNumber, num2: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: (num1.real + num2.real),
                imag: (num1.imag + num2.imag)
            }
        }
    }
}
