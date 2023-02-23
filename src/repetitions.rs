//! Functions for matching repetitions

use super::humanregex::{HumanRegex, SymbolChain};

/// Match at least _n_ of a certain target
/// ```
/// let regex_string = human_regex::at_least(3, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn at_least(n: u8, target: impl HumanRegex) -> SymbolChain {
    SymbolChain(format!("(?:{}){{{},}}", target, n))
}

/// Match at least _n_ and at most _m_ of a certain target
/// ```
/// let regex_string = human_regex::between(3, 5, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn between(n: u8, m: u8, target: impl HumanRegex) -> SymbolChain {
    SymbolChain(format!("(?:{}){{{},{}}}", target, n, m))
}

/// Match one or more of a certain target
/// ```
/// let regex_string = human_regex::one_or_more("a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("bb"));
/// ```
pub fn one_or_more(target: impl HumanRegex) -> SymbolChain {
    SymbolChain(format!("(?:{})+", target))
}

/// Match zero or more of a certain target
/// ```
/// let regex_string = human_regex::zero_or_more("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_more(target: impl HumanRegex) -> SymbolChain {
    SymbolChain(format!("(?:{})*", target))
}

/// Match zero or one of a certain target
/// ```
/// let regex_string = human_regex::zero_or_one("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_one(target: impl HumanRegex) -> SymbolChain {
    SymbolChain(format!("(?:{})?", target))
}

/// Match exactly _n_ of a certain target
/// ```
/// let regex_string = human_regex::exactly(5, "a");
/// assert!(regex_string.to_regex().is_match("aaaaa"));
/// assert!(!regex_string.to_regex().is_match("aaa"));
/// ```
pub fn exactly(n: u8, target: impl HumanRegex) -> SymbolChain {
    SymbolChain(format!("(?:{}){{{}}}", target, n))
}
