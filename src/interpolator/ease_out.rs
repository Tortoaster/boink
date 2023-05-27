use std::f64::consts::PI;

use crate::interpolator::Interpolator;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseOut;

impl Interpolator for EaseOut {
    fn y(&self, x: f64) -> f64 {
        (x * PI / 2.0).sin()
    }
}

pub type EaseOutQuad = EaseOutExpo<2>;
pub type EaseOutCubic = EaseOutExpo<3>;
pub type EaseOutQuart = EaseOutExpo<4>;
pub type EaseOutQuint = EaseOutExpo<5>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseOutExpo<const E: i32>;

impl<const E: i32> Interpolator for EaseOutExpo<E> {
    fn y(&self, x: f64) -> f64 {
        1.0 - (1.0 - x).powi(E)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseOutCirc;

impl Interpolator for EaseOutCirc {
    fn y(&self, x: f64) -> f64 {
        (1.0 - (x - 1.0).powi(2)).sqrt()
    }
}
