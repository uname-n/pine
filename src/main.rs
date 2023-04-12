mod error;
mod pine;
mod vector;

pub use crate::error::PineError;
pub use crate::pine::Pine;
pub use crate::vector::Vector;

use std::path::PathBuf;

fn main() {
    let pine = Pine::new(PathBuf::from("pine_test"), 0.9).expect("Cannot create Pine instance");

    println!("{:?}", pine);
}
