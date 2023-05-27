pub use ease_in::{
    EaseIn, EaseInCirc, EaseInCubic, EaseInExpo, EaseInQuad, EaseInQuart, EaseInQuint,
};
pub use ease_in_out::{
    EaseInOut, EaseInOutCirc, EaseInOutCubic, EaseInOutExpo, EaseInOutQuad, EaseInOutQuart,
    EaseInOutQuint,
};
pub use ease_out::{
    EaseOut, EaseOutCirc, EaseOutCubic, EaseOutExpo, EaseOutQuad, EaseOutQuart, EaseOutQuint,
};
pub use linear::Linear;

mod ease_in;
mod ease_in_out;
mod ease_out;
mod linear;

pub trait Interpolator {
    fn y(&self, x: f64) -> f64;
}
