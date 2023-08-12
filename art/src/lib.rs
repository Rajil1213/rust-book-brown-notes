//! Art
//!
//! A library for modeling artistic concepts

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    ///
    /// Example:
    /// ```
    /// use art::{mix, PrimaryColor, SecondaryColor};
    ///
    /// let primary_color_1 = PrimaryColor::Yellow;
    /// let primary_color_2 = PrimaryColor::Red;
    /// let mixture = mix(primary_color_1, primary_color_2);
    /// assert_eq!(SecondaryColor::Orange, mixture)
    /// ```
    ///
    /// Panics:
    /// if both the primary colors passed to it are the same.
    pub fn mix(pc1: PrimaryColor, pc2: PrimaryColor) -> SecondaryColor {
        if pc1 == pc2 {
            panic!("you must mix two different primary colors to get a secondary color!")
        }

        if pc1 == PrimaryColor::Blue && pc2 == PrimaryColor::Red {
            SecondaryColor::Purple
        } else if pc1 == PrimaryColor::Blue && pc2 == PrimaryColor::Yellow {
            SecondaryColor::Green
        } else {
            SecondaryColor::Orange
        }
    }
}
