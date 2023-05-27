use std::f64::consts::PI;

use crate::interpolator::Interpolator;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseInOut;

impl Interpolator for EaseInOut {
    fn y(&self, x: f64) -> f64 {
        -((PI * x).cos() - 1.0) / 2.0
    }
}

pub type EaseInOutQuad = EaseInOutExpo<2>;
pub type EaseInOutCubic = EaseInOutExpo<3>;
pub type EaseInOutQuart = EaseInOutExpo<4>;
pub type EaseInOutQuint = EaseInOutExpo<5>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseInOutExpo<const E: i32>;

impl<const E: i32> Interpolator for EaseInOutExpo<E> {
    fn y(&self, x: f64) -> f64 {
        if x < 0.5 {
            2.0f64.powi(E - 1) * x.powi(E)
        } else {
            1.0 - (1.0 - x).powi(E) * 2.0f64.powi(E - 1)
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EaseInOutCirc;

impl Interpolator for EaseInOutCirc {
    fn y(&self, x: f64) -> f64 {
        if x < 0.5 {
            (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
        } else {
            (1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() / 2.0 + 0.5
        }
    }
}
