//! Functions for ASCII character classes

use super::humanregex::AsciiSymbolClass;

/// A function to match any alphanumeric character (`[0-9A-Za-z]`)
pub fn alphanumeric() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:alnum:]]".to_string())
}

/// A function to match any non-alphanumeric character (`[^0-9A-Za-z]`)
pub fn non_alphanumeric() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^alnum:]]".to_string())
}

/// A function to match any alphabetic character (`[A-Za-z]`)
pub fn alphabetic() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:alpha:]]".to_string())
}

/// A function to match any non-alphabetic character (`[^A-Za-z]`)
pub fn non_alphabetic() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^alpha:]]".to_string())
}

/// A function to match any lowercase character (`[a-z]`)
pub fn lowercase() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:lower:]]".to_string())
}

/// A function to match any non-lowercase character (`[^a-z]`)
pub fn non_lowercase() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^lower:]]".to_string())
}

/// A function to match any uppercase character (`[A-Z]`)
pub fn uppercase() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:upper:]]".to_string())
}

/// A function to match any non-uppercase character (`[^A-Z]`)
pub fn non_uppercase() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^upper:]]".to_string())
}

/// A function to match any digit that would appear in a hexadecimal number (`[A-Fa-f0-9]`)
pub fn hexdigit() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:xdigit:]]".to_string())
}

/// A function to match any digit that wouldn't appear in a hexadecimal number (`[^A-Fa-f0-9]`)
pub fn non_hexdigit() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^xdigit:]]".to_string())
}

/// A function to match any ascii digit (`[\x00-\x7F]`)
pub fn ascii() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:ascii:]]".to_string())
}

/// A function to match any non-ascii digit (`[^\x00-\x7F]`)
pub fn non_ascii() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^ascii:]]".to_string())
}

/// A function to match blank characters (`[\t ]`)
pub fn blank() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:blank:]]".to_string())
}

/// A function to match non-blank characters (`[^\t ]`)
pub fn non_blank() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^blank:]]".to_string())
}

/// A function to match control characters (`[\x00-\x1F\x7F]`)
pub fn control() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:cntrl:]]".to_string())
}

/// A function to match non-control characters (`[^\x00-\x1F\x7F]`)
pub fn non_control() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^cntrl:]]".to_string())
}

/// A function to match graphical characters (`[!-~]`)
pub fn graphical() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:graph:]]".to_string())
}

/// A function to match non-graphical characters (`[^!-~]`)
pub fn non_graphical() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^graph:]]".to_string())
}

/// A function to match printable characters (`[ -~]`)
pub fn printable() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:print:]]".to_string())
}

/// A function to match unprintable characters (`[^ -~]`)
pub fn non_printable() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^print:]]".to_string())
}

/// A function to match punctuation (`[!-/:-@\[-`{-~]`)
pub fn punctuation() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:punct:]]".to_string())
}

/// A function to match non-punctuation (`[^!-/:-@\[-`{-~]`)
pub fn non_punctuation() -> AsciiSymbolClass {
    AsciiSymbolClass(r"[[:^punct:]]".to_string())
}
