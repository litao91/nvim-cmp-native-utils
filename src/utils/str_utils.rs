use std::collections::HashSet;

use super::byte_char;

pub fn is_invalid_chars(c: u8) -> bool {
    match c {
        b'\'' => true,
        b'"' => true,
        b'=' => true,
        b'$' => true,
        b'(' => true,
        b'[' => true,
        b'<' => true,
        b'{' => true,
        b' ' => true,
        b'\t' => true,
        b'\n' => true,
        b'\r' => true,
        _ => false,
    }
}

pub fn pair_chars(c: u8) -> Option<u8> {
    match c {
        b'<' => Some(b'>'),
        b'[' => Some(b']'),
        b'(' => Some(b')'),
        b'{' => Some(b'}'),
        b'"' => Some(b'"'),
        b'\'' => Some(b'\''),
        _ => None,
    }
}

pub fn get_word_with_min_len(text: &str, stop_char: u8, min_length: usize) -> String {
    let mut has_alnum = false;
    let mut word = Vec::new();
    let mut stack = Vec::new();

    let add = |word: &mut Vec<u8>, stack: &mut Vec<u8>, c: u8| {
        word.push(c);
        if match stack.first() {
            Some(top) => *top == c,
            None => false,
        } {
            stack.pop();
        } else {
            if let Some(p) = pair_chars(c) {
                stack.push(c);
            }
        }
    };

    for c in text.bytes() {
        if word.len() < min_length {
            word.push(c);
        } else if !is_invalid_chars(c) {
            add(&mut word, &mut stack, c);
            has_alnum = has_alnum || byte_char::is_alnum(c);
        } else if !has_alnum {
            add(&mut word, &mut stack, c);
        } else if !stack.is_empty() {
            add(&mut word, &mut stack, c);
            if has_alnum && stack.is_empty() {
                break;
            }
        } else {
            break;
        }
    }
    if stop_char != 0
        && match word.last() {
            None => false,
            Some(c) => *c == stop_char,
        }
    {
        word.pop();
    }
    String::from_utf8(word).unwrap()
}

pub fn get_word(text: &str, stop_char: u8) -> String {
    get_word_with_min_len(text, stop_char, 0)
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(get_word("print", 0), "print");

        assert_eq!(get_word("$variable", 0), "$variable");
        assert_eq!(get_word("print()", 0), "print");
        assert_eq!(get_word("[\"cmp#confirm\"]", 0), "[\"cmp#confirm\"]");
        assert_eq!(get_word("\"devDependencies\":", b'"'), "\"devDependencies");
        assert_eq!(
            get_word("\"devDependencies\": ${1},", b'"'),
            "\"devDependencies"
        );
        assert_eq!(get_word("#[cfg(test)]", 0), "#[cfg(test)]");
        assert_eq!(
            get_word_with_min_len("import { GetStaticProps$1 } from \"next\";", 0, 9),
            "import { GetStaticProps"
        );
    }
}
