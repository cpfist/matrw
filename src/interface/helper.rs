//!
//! Module containing helper functions
//!

///
/// Return if string `name` is a valid MATALB variable name.
///
pub fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    static MATLABKEYWORDS: [&str; 20] = [
        "break",
        "case",
        "catch",
        "classdef",
        "continue",
        "else",
        "elseif",
        "end",
        "for",
        "function",
        "global",
        "if",
        "otherwise",
        "parfor",
        "persistent",
        "return",
        "spmd",
        "switch",
        "try",
        "while",
    ];

    if MATLABKEYWORDS.contains(&name) {
        return false;
    }

    // Variable name must start with a letter, then followed by letters, numbers or underscores.
    let ok_first = name.chars().next().unwrap().is_ascii_alphabetic();
    let ok_symbols = name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    let ok_len = name.len() <= 63;

    ok_first && ok_symbols && ok_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_variable_names() {
        // Valid names
        assert!(is_valid_variable_name("a"));
        assert!(is_valid_variable_name("a1"));
        assert!(is_valid_variable_name("a_1"));
        assert!(is_valid_variable_name(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_very_long_variable_name"
        ));

        // Invalid names
        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name(" "));
        assert!(!is_valid_variable_name("1a"));
        assert!(!is_valid_variable_name("_a"));
        assert!(!is_valid_variable_name("!"));
        assert!(!is_valid_variable_name("😀"));
        assert!(!is_valid_variable_name(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_too_long_variable_name"
        ));
    }
}
