use std::mem::MaybeUninit;

pub use boink_derive::Lerp;
use paste::paste;

use crate::interpolator::Interpolator;

pub trait Lerp {
    fn lerp(&self, to: &Self, at: f64) -> Self;

    fn interpolate(&self, to: &Self, at: f64, interpolator: &impl Interpolator) -> Self
    where
        Self: Sized,
    {
        self.lerp(to, interpolator.y(at))
    }
}

macro_rules! impl_lerp_num {
    ($($t:ty),*) => {$(
        impl Lerp for $t {
            fn lerp(&self, to: &Self, at: f64) -> Self {
                *self + (((*to - *self) as f64) * at) as $t
            }
        }
    )*}
}

impl_lerp_num!(i8, i16, i32, i64, i128, isize);
impl_lerp_num!(u8, u16, u32, u64, u128, usize);
impl_lerp_num!(f32, f64);

macro_rules! impl_lerp_array {
    ($($n:expr),*) => {$(
        impl<T> Lerp for [T; $n]
        where
            T: Lerp,
        {
            fn lerp(&self, to: &Self, at: f64) -> Self {
                let mut array: [MaybeUninit<T>; $n] =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for (index, value) in array.iter_mut().enumerate() {
                    value.write(self[index].lerp(&to[index], at));
                }
                unsafe { array.as_ptr().cast::<Self>().read() }
            }
        }
    )*}
}

impl_lerp_array!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

macro_rules! impl_lerp_tuple {
    ($($n:ident),*) => {
    	paste! {
			impl<$($n),*> Lerp for ($($n,)*)
			where
				$($n: Lerp,)*
			{
				fn lerp(&self, to: &Self, at: f64) -> Self {
					let (
						$([<$n:snake _self>],)*
					) = self;
					let (
						$([<$n:snake _to>],)*
					) = to;
					(
						$([<$n:snake _self>].lerp([<$n:snake _to>], at),)*
					)
				}
			}
        }
    }
}

macro_rules! impl_lerp_tuples {
	($first:ident $(, $n:ident)*) => {
		impl_lerp_tuple!($first $(, $n)*);
		impl_lerp_tuples!($($n),*);
	};

	() => {};
}

impl_lerp_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
