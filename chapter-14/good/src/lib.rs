///# function_add
///
///## useage
///
///
///## example
///
///```bash
///cargo run
///cargo build
///cargo test
///cargo new 
///
///```
///
///```
///let x = 5;
///
///let y = 10;
///
///let result = good::add(x, y);
///
///assert_eq!(15, result);
///
///
///
///
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


//! # Art
//!
//! A libary for modeling artistic concepts.
//!



pub mod kinds {
    /// The primary colors according to the RYB color model.

    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.

    pub enum SencondaryColor {
        Orange,
        Green,
        Purple,
    }

}

pub mod utils {
    use crate::kingds*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        unimplemented!();
    }
}

