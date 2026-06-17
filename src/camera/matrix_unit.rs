use core::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign
    };



/** Discrete unit representation of directional movement along a single matrix axis. */
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum MatrixUnit {
    /** Neutral state, indicating no movement or value. */
    #[default]
    Zero = 0,
    /** Positive state, indicating movement in the positive direction. */
    Positiv = 1,
    /** Negative state, indicating movement in the negative direction. */
    Negative = -1
    }

impl MatrixUnit {
    /** Converts an 8-bit signed integer into a corresponding discrete matrix unit. */
    const fn from_i8(value: i8) -> Self {
        match value {
            0 => Self::Zero,
            1 .. => Self::Positiv,
            ..= -1 => Self::Negative
            }
        }
    }

impl From<i8> for MatrixUnit {
    fn from(value: i8) -> Self {
        Self::from_i8(value)
        }
    }

impl Add for MatrixUnit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_i8((self as i8) + (rhs as i8))     
        }
    }

impl Sub for MatrixUnit {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_i8((self as i8) - (rhs as i8))
        }
    }

impl AddAssign for MatrixUnit {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
        }
    }

impl SubAssign for MatrixUnit {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
        }
    }