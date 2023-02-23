//! Functions for performing logical operations

use super::humanregex::{CustomSymbolClass, LiteralSymbolChain, SymbolChain, SymbolClass};
use std::fmt;

/// A function for establishing an OR relationship between two or more possible matches
/// ```
/// use human_regex::{text, logical::or};
/// let regex_string = text("what's") + or(&["up", "going on", "good") + text("?");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("what's up?"));
/// assert!(regex_string.to_regex().is_match("what's going on?"));
/// assert!(regex_string.to_regex().is_match("what's good?"));
/// assert!(!regex_string.to_regex().is_match("what's wrong?"));
/// ```
pub fn or<T>(options: &[T]) -> SymbolChain
where
    T: Into<LiteralSymbolChain> + fmt::Display,
{
    let mut regex_string = format!("{}", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("{}|{}", regex_string, options[idx].to_string())
    }
    SymbolChain(format!("(:?{})", regex_string))
}

// pub fn nor<T>(options: &[T]) -> {}

// pub fn xor<T>(lhs: , rhs: ) -> {}

/// A function for taking the intersection of two or more character classes
/// ```
/// use human_regex::{text, and, or, within};
/// let regex_string = and(&vec![within('a'..='y'),or(&['x','y','z'])]);
/// println!("{}", regex_string);
/// assert!(regex_string.to_regex().is_match("x"));
/// assert!(regex_string.to_regex().is_match("y"));
/// assert!(!regex_string.to_regex().is_match("z"));
/// ```
pub fn and<T>(options: &[impl SymbolClass]) -> CustomSymbolClass {
    let mut regex_string = format!("{}", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("{}&&{}", regex_string, options[idx].to_string())
    }
    CustomSymbolClass(format!("[{}]", regex_string))
}

/// Subtracts the second argument from the first
///
/// If you would like to use ranges, collect them into a Vec<T>.
/// ```
/// use human_regex::subtract;
/// let regex_string = subtract(within_range('0'..='9'), within_set(&['4']));
/// println!("{}", regex_string);
/// assert!(regex_string.to_regex().is_match("3"));
/// assert!(regex_string.to_regex().is_match("9"));
/// assert!(!regex_string.to_regex().is_match("4"));
/// ```
pub fn subtract(from: impl SymbolClass, subtract: impl SymbolClass) -> CustomSymbolClass {
    CustomSymbolClass(format!("[{}--{}]", from, subtract))
}
