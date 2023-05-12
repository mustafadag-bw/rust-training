// This comments with ! will be included in main page of the documentation
//! # Crates Detail
//!
//! `crates_detail` is a collection of utilities to make performing certain
//! calculations more convenient.
//!
//! # Art
//!
//! A library for modeling artistic concepts.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crates_detail::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Errors
///
/// # Safety
///
/// # Panics
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

// This part provides users to be able to use PrimaryColor, mix and Secondary color directly from crate
// instead of using it via kinds and utils. It also creates reexports section
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
