//! # My Crate Pato
//!
//! `my_crate` is a collection of utilities.


/// Adds one to the number given
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate_pato::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use self::kinds::PrimaryColor;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }
}
