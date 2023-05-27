use crate::interpolator::Interpolator;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Linear;

impl Interpolator for Linear {
    fn y(&self, x: f64) -> f64 {
        x
    }
}

// TODO: QuadBezier
// TODO: CubicBezier
