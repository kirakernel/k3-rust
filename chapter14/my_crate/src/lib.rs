//! My Crate
//! 
//! `my_crate`is a collection of utilities to make performing certain
//! calculations more covenient.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(5);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_to_positive_nums() {
        let arg = 5;
        let answer = add_one(arg);
        assert_eq!(6, answer);
    }
}
