use std::f64::consts::PI;

use crate::interpolator::Interpolator;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseIn;

impl Interpolator for EaseIn {
    fn y(&self, x: f64) -> f64 {
        1.0 - (x * PI / 2.0).cos()
    }
}

pub type EaseInQuad = EaseInExpo<2>;
pub type EaseInCubic = EaseInExpo<3>;
pub type EaseInQuart = EaseInExpo<4>;
pub type EaseInQuint = EaseInExpo<5>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseInExpo<const E: i32>;

impl<const E: i32> Interpolator for EaseInExpo<E> {
    fn y(&self, x: f64) -> f64 {
        x.powi(E)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseInCirc;

impl Interpolator for EaseInCirc {
    fn y(&self, x: f64) -> f64 {
        1.0 - (1.0 - x * x).sqrt()
    }
}
