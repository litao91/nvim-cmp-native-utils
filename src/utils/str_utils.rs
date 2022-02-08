use std::collections::HashSet;

pub fn is_invalid_chars(c: u8) -> bool {
    match c {
        b'\'' => true,
        b'"' => true,
        b'=' => true,
        b'$' => true,
        b'(' => true,
        b'[' => true,
        b' ' => true,
        b'\t' => true,
        b'\n' => true,
        b'\r' => true,
        _ => false,
    }
}

pub fn pair_chars(c: u8) -> Option<u8> {
    match c {
        b'[' => Some(b']'),
        b'(' => Some(b')'),
        b'<' => Some(b'>'),
        _ => None,
    }
}

pub fn get_word(text: &str, stop_char: u8) -> &str {
    let mut valids = HashSet::new();
    let bytes = text.as_bytes();
    let mut has_valid = false;
    for idx in 0..bytes.len() {
        let c = bytes[idx];
        let invalid = is_invalid_chars(c) && !(valids.contains(&c) && stop_char != c);
        if has_valid && invalid {
            return &text[..idx - 1];
        }
        valids.insert(c);
        if let Some(pair) = pair_chars(c) {
            valids.insert(pair);
        }
        has_valid = has_valid || !invalid
    }
    return text;
}

pub fn oneline(text: &str) -> &str {
    let bytes = text.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            return std::str::from_utf8(&bytes[..i]).unwrap();
        }
    }
    return text;
}

pub fn remove_suffix<'a>(text: &'a str, suffix: &str) -> &'a str {
    if text.ends_with(suffix) {
        &text[..(text.len() - suffix.len())]
    } else {
        text
    }
}
