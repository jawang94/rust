//! # Art
//!
//! A library for modeling artistic concepts.

/// For example, we can make an easy to access Public API using reexporting
/// The syntax for this is pub use
/// The result is instead of having to access by crate::parent::parent::child,
/// they can simply use crate::child
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
