use regex::Regex;

use super::nonescaped_text;
pub(crate) use std::fmt;
use std::ops::{Add, Not};

/// The general HumanRegex struct that signifies any valid regular expression.
pub trait HumanRegex: Add + fmt::Display + Sized {
    /// Convert to a rust Regex
    fn to_regex(&self) -> Regex {
        Regex::new(&self.to_string()).unwrap()
    }

    /// Add a lazy modifier
    fn lazy(&self) -> SymbolChain {
        SymbolChain(format!("{}?", &self.to_string()))
    }
}

pub trait SymbolClass: Not + HumanRegex {}

/// Signifies a standard symbol class; the kind with a backslash followed by a letter.
#[derive(Debug)]
pub struct StandardSymbolClass(pub String);

impl HumanRegex for StandardSymbolClass {}

impl<T: HumanRegex> Add<T> for StandardSymbolClass {
    type Output = SymbolChain;
    fn add(self, rhs: T) -> <Self as Add>::Output {
        SymbolChain(format!("{}{}", self.to_string(), rhs.to_string()))
    }
}

impl fmt::Display for StandardSymbolClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SymbolClass for StandardSymbolClass {}

impl Not for StandardSymbolClass {
    type Output = Self;

    fn not(self) -> Self::Output {
        StandardSymbolClass(
            self.0
                .replace(r"\d", r"\D")
                .replace(r"\D", r"\d")
                .replace(r"\p", r"\P")
                .replace(r"\P", r"\p")
                .replace(r"\s", r"\S")
                .replace(r"\S", r"\s")
                .replace(r"\w", r"\W")
                .replace(r"\b", r"\B")
                .replace(r"\B", r"\b"),
        )
    }
}

/// Signifies a user-defined symbol class; the kind surrounded by one layer of square brackets.
#[derive(Debug)]
pub struct CustomSymbolClass(pub String);

impl HumanRegex for CustomSymbolClass {}

impl<T: HumanRegex> Add<T> for CustomSymbolClass {
    type Output = SymbolChain;
    fn add(self, rhs: T) -> <Self as Add>::Output {
        SymbolChain(format!("{}{}", self.to_string(), rhs.to_string()))
    }
}

impl fmt::Display for CustomSymbolClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SymbolClass for CustomSymbolClass {}

impl Not for CustomSymbolClass {
    type Output = Self;

    fn not(self) -> Self::Output {
        CustomSymbolClass(self.0.replace("[", "[^"))
    }
}

/// Signifies an ASCII symbol class; the kind surrounded by colons and two layers of square brackets.
#[derive(Debug)]
pub struct AsciiSymbolClass(pub String);

impl HumanRegex for AsciiSymbolClass {}

impl<T: HumanRegex> Add<T> for AsciiSymbolClass {
    type Output = SymbolChain;
    fn add(self, rhs: T) -> <Self as Add>::Output {
        SymbolChain(format!("{}{}", self.to_string(), rhs.to_string()))
    }
}

impl fmt::Display for AsciiSymbolClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SymbolClass for AsciiSymbolClass {}

impl Not for AsciiSymbolClass {
    type Output = Self;

    fn not(self) -> Self::Output {
        AsciiSymbolClass(self.0.replace("[[:", "[[:^"))
    }
}

/// Signifies a chain of literal symbols, meaning, a string of literal text.
#[derive(Debug)]
pub struct LiteralSymbolChain(pub String);

impl HumanRegex for LiteralSymbolChain {}

impl<T: HumanRegex> Add<T> for LiteralSymbolChain {
    type Output = SymbolChain;
    fn add(self, rhs: T) -> <Self as Add>::Output {
        SymbolChain(format!("{}{}", self.to_string(), rhs.to_string()))
    }
}

impl fmt::Display for LiteralSymbolChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Not for LiteralSymbolChain {
    type Output = SymbolChain;

    fn not(self) -> Self::Output {
        nonescaped_text(
            &self
                .0
                .chars()
                .into_iter()
                .map(|c| format!("[^{}]", c))
                .reduce(|acc: String, elem: String| acc + &elem)
                .unwrap(),
        )
    }
}

impl From<String> for LiteralSymbolChain {
    fn from(val: String) -> Self {
        Self(val)
    }
}

/// Signifies an arbitrary chain of symbols.
///
/// This is what [HumanRegex] used to do, before it was converted to be a trait.
#[derive(Debug)]
pub struct SymbolChain(pub String);

impl HumanRegex for SymbolChain {}

impl<T: HumanRegex> Add<T> for SymbolChain {
    type Output = SymbolChain;
    fn add(self, rhs: T) -> <Self as Add>::Output {
        SymbolChain(format!("{}{}", self.to_string(), rhs.to_string()))
    }
}
impl fmt::Display for SymbolChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
