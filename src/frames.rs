use std::time::Duration;

use crate::{
    interpolator::{Interpolator, Linear},
    Repeat, Transition,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Frames<T> {
    frames: Vec<T>,
    transition: Transition<usize, Linear>,
}

impl<T> Frames<T> {
    pub fn new(frames: Vec<T>, duration: Duration) -> Self {
        let transition =
            Transition::new(0, frames.len() - 1, duration, Linear).repeat(Repeat::Forever);

        Frames { frames, transition }
    }

    pub fn current(&self) -> &T {
        self.frames.get(self.transition.value()).unwrap()
    }
}
