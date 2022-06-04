pub mod complex {
    #[derive(Debug, Clone, Copy)]
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    // getters and setters
    impl ComplexNumber {
        // This function returns the value of real part.
        pub fn real(&self) -> f64 {
            self.real
        }

        // This function returns the value of imaginary part.
        pub fn imag(&self) -> f64 {
            self.imag
        }

        // This sets the value of real part with provided param.
        pub fn set_real(&mut self, set_with: f64) {
            self.real = set_with;
        }
        
        // This sets the value of imaginary part with provided param.
        pub fn set_imag(&mut self, set_with: f64) {
            self.imag = set_with;
        }
    }

    // Methods defining Operations with Complex Numbers 
    impl ComplexNumber {
        // This methods mutates the instance by added the `ComplexNumber` provided as an argument
        pub fn add(&mut self, add_with: &ComplexNumber) {
            self.real += add_with.real;
            self.imag += add_with.imag;
        }
        
        // This methods mutates the instance by subtracting the `ComplexNumber` (provided as an argument) from the current instance
        pub fn reduce(&mut self, reduce_by: &ComplexNumber) {
            self.real -= reduce_by.real;
            self.imag -= reduce_by.imag;
        }

        // This method mutates the instance by multiply it with the `ComplexNumber` provided.
        pub fn multiply(&mut self, multiply_with: &ComplexNumber) {
            let real = self.real * multiply_with.real - self.imag * multiply_with.imag;
            let imag = self.real * multiply_with.imag + self.imag * multiply_with.real;

            self.real = real;
            self.imag = imag;
        }
    }

    // Associated Functions for { creating Instances, defining Complex Number Operations}
    impl ComplexNumber {
        // This creates a new instance of `ComplexNumber` with provided arguments for real( 1st param) and imag( 2nd param ) parameter. 
        pub fn new(real: f64, imag: f64) -> ComplexNumber {
            ComplexNumber { real, imag }
        }

        // This return a new `ComplexNumber` which result from addition of num1 and num2
        pub fn addition_of(num1: &ComplexNumber, num2: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: (num1.real + num2.real),
                imag: (num1.imag + num2.imag)
            }
        }

        // This return a new `ComplexNumber` which result from the operation (num1 - num2)
        pub fn subtraction_of(num1: &ComplexNumber, num2: &ComplexNumber) -> ComplexNumber {
            ComplexNumber { 
                real: (num1.real - num2.real),
                imag: (num1.imag - num2.imag)
            }
        }

        // This return a new `ComplexNumber` which result from the multiplication of num1 and num2 `ComplexNumber`s provided as params.
        pub fn multiplication_of(num1: &ComplexNumber, num2: &ComplexNumber) -> ComplexNumber {
            ComplexNumber { 
                real: (num1.real * num2.real - num1.imag * num2.imag),
                imag: (num1.real * num2.imag + num1.imag * num2.real)
            }
        }
    }
}
