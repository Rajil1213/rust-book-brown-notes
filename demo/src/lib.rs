//! # Demo
//!
//! `Demo` is a collection of utilities that demonstrate `cargo`'s capabiltites.

/// Adds one to the given number.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = demo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
