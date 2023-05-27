use std::time::{Duration, Instant};

use crate::{interpolator::Interpolator, lerp::Lerp};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Transition<T, I> {
    from: T,
    to: T,
    duration: Duration,
    interpolator: I,
    start_time: Instant,
    repeat: Repeat,
    direction: Direction,
}

impl<T, I> Transition<T, I> {
    pub fn new(from: T, to: T, duration: Duration, interpolator: I) -> Self {
        Transition {
            from,
            to,
            duration,
            interpolator,
            start_time: Instant::now(),
            repeat: Repeat::default(),
            direction: Direction::default(),
        }
    }

    pub fn repeat(mut self, repeat: Repeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn from(&self) -> &T {
        &self.from
    }

    pub fn to(&self) -> &T {
        &self.to
    }

    pub fn set_from(&mut self, from: T) {
        self.from = from;
    }

    pub fn set_to(&mut self, to: T) {
        self.to = to;
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn restart(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn restart_after(&mut self, delay: Duration) {
        self.start_time = Instant::now() + delay;
    }
}

impl<T, I> Transition<T, I>
where
    T: Lerp,
    I: Interpolator,
{
    pub fn value(&self) -> T {
        let elapsed = self.start_time.elapsed();
        let scalar = elapsed.as_micros() as f64 / self.duration.as_micros() as f64;
        let scalar = self.direction.process(self.repeat.process(scalar));
        let at = self.interpolator.y(scalar);
        self.from.lerp(&self.to, at)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Direction {
    #[default]
    Forwards,
    Backwards,
    ForwardsThenBackwards,
    BackwardsThenForwards,
}

impl Direction {
    fn process(&self, x: f64) -> f64 {
        match self {
            Direction::Forwards => x,
            Direction::Backwards => 1.0 - x,
            Direction::ForwardsThenBackwards => 1.0 - (x * 2.0 - 1.0).abs(),
            Direction::BackwardsThenForwards => (x * 2.0 - 1.0).abs(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Repeat {
    #[default]
    Once,
    Forever,
}

impl Repeat {
    fn process(&self, x: f64) -> f64 {
        match self {
            Repeat::Once => x.clamp(0.0, 1.0),
            Repeat::Forever => x.rem_euclid(1.0),
        }
    }
}
