mod error;
mod pine;
mod vector;

pub use crate::error::PineError;
pub use crate::pine::Pine;
pub use crate::vector::Vector;

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
