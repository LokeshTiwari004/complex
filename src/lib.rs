pub mod complex {
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    impl ComplexNumber {
        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }
    }
}