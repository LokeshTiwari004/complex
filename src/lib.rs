pub mod complex {
    #[derive(Debug)]
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    impl ComplexNumber {

        pub fn new(real: f64, imag: f64) -> ComplexNumber {
            ComplexNumber { real, imag }
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }
    }
}