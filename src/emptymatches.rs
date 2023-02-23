//! Functions for the empty matches

use super::humanregex::StandardSymbolClass;

/// A function to match a word boundary
pub fn word_boundary() -> StandardSymbolClass {
    StandardSymbolClass(r"\b".to_string())
}

/// A function to match anything BUT a word boundary
pub fn non_word_boundary() -> StandardSymbolClass {
    StandardSymbolClass(r"\B".to_string())
}

/// A function to match the beginning of text (or start-of-line with multi-line mode)
/// ```
/// use human_regex::{beginning, text};
/// let regex_string = beginning() + text("hex");
/// assert!(regex_string.to_regex().is_match("hexagon"));
/// assert!(!regex_string.to_regex().is_match("chlorhexadine"));
/// ```
pub fn beginning() -> StandardSymbolClass {
    StandardSymbolClass(r"^".to_string())
}

/// A function to match the end of text (or end-of-line with multi-line mode)
/// ```
/// use human_regex::{end, text};
/// let regex_string = text("end") + end();
/// assert!(regex_string.to_regex().is_match("mend"));
/// assert!(!regex_string.to_regex().is_match("endocrinologist"));
/// ```
pub fn end() -> StandardSymbolClass {
    StandardSymbolClass(r"$".to_string())
}

/// A function to match the beginning of text (even with multi-line mode enabled)
/// ```
/// use human_regex::{beginning_of_text, text};
/// let regex_string = beginning_of_text() + text("hex");
/// assert!(regex_string.to_regex().is_match("hexagon"));
/// assert!(!regex_string.to_regex().is_match("chlorhexadine"));
/// ```
pub fn beginning_of_text() -> StandardSymbolClass {
    StandardSymbolClass(r"\A".to_string())
}

/// A function to match the end of text (even with multi-line mode enabled)
/// ```
/// use human_regex::{end_of_text, text};
/// let regex_string = text("end") + end_of_text();
/// assert!(regex_string.to_regex().is_match("mend"));
/// assert!(!regex_string.to_regex().is_match("endocrinologist"));
/// ```
pub fn end_of_text() -> StandardSymbolClass {
    StandardSymbolClass(r"\z".to_string())
}
