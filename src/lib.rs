#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Regex for Humans
//! ## About
//! The goal of this crate is simple: give everybody the power of regular expressions without having
//! to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).

//!## Example usage
//!### Matching a date
//!If you want to match a date of the format `2021-10-30`, you would use the following code to generate a regex:
//!```rust
//!let hr = human_regex::HumanRegex::new()
//!    .begin()
//!    .exactly(4, human_regex::DIGIT)
//!    .text("-")
//!    .exactly(2, human_regex::DIGIT)
//!    .text("-")
//!    .exactly(2, human_regex::DIGIT)
//!    .end();
//!assert!(hr.is_match("2014-01-01"));
//!```
//!Specifically, this chunk of code would yield the regex `^\d{4}-\d{2}-\d{2}$`, which is exactly what we want!

use regex::Regex;

/// A constant for the digit character class (i.e., the digits 0 through 9)
pub const DIGIT: &str = r"\d";
/// A constant for the non-digit character class (i.e., everything BUT the digits 0-9)
pub const NON_DIGIT: &str = r"\D";
/// A constant for the word character class (i.e., all alphanumeric characters plus underscore)
pub const WORD: &str = r"\w";
/// A constant for the non-word character class (i.e., everything BUT the alphanumeric characters plus underscore)
pub const NON_WORD: &str = r"\W";
/// A constant for the whitespace character class (i.e., space and tab)
pub const WHITESPACE: &str = r"\t";
/// A constant for the whitespace character class (i.e., everything BUT space and tab)
pub const NON_WHITESPACE: &str = r"\T";

/// The HumanRegex struct which maintains and updates the regex string
#[derive(Default)]
pub struct HumanRegex {
    /// The internally-maintained true regex string
    pub regex_string: String,
}

impl HumanRegex {
    /// Generate a new HumanRegex with a blank regex_string
    pub fn new() -> Self {
        HumanRegex {
            regex_string: String::from(""),
        }
    }

    /// Match exactly a certain number of a certain target
    pub fn exactly(&self, n: u8, target: &str) -> Self {
        let new_regex = format!("{}{}{{{}}}", self.regex_string, target, n);
        HumanRegex {
            regex_string: new_regex,
        }
    }

    /// Add text directly to the match string
    pub fn text(&self, text: &str) -> Self {
        let new_regex = format!("{}{}", self.regex_string, text);
        HumanRegex {
            regex_string: new_regex,
        }
    }

    /// Represents the beginning of the text
    pub fn begin(&self) -> Self {
        let new_regex = format!("{}{}", self.regex_string, r"^");
        HumanRegex {
            regex_string: new_regex,
        }
    }

    /// Represents the end of the text
    pub fn end(&self) -> Self {
        let new_regex = format!("{}{}", self.regex_string, r"$");
        HumanRegex {
            regex_string: new_regex,
        }
    }

    /// Generates a new human regex directly from a regex string
    pub fn from_regex_string(regex_string: &str) -> Self {
        HumanRegex {
            regex_string: String::from(regex_string),
        }
    }

    /// Returns the current state of the constructed regex string
    pub fn get_regex_string(&self) -> &String {
        return &self.regex_string;
    }

    /// Checks whether or not a string matches with the constructed regex
    pub fn is_match(&self, string_to_match: &str) -> bool {
        let re = Regex::new(&*self.regex_string).unwrap();
        re.is_match(string_to_match)
    }
}
