pub mod cartesian;
pub mod polar;


#[derive(Debug, Clone, Copy)]
pub enum ComplexNumber {
    CF(cartesian::CartesianFormat),
    PF(polar::PolarFormat),
}
