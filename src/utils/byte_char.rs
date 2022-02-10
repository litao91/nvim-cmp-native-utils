pub fn is_white(byte: u8) -> bool {
    match byte {
        b' ' | b'\t' | b'\n' => true,
        _ => false,
    }
}

pub fn is_upper(byte: u8) -> bool {
    match byte {
        b'A' | b'B' | b'C' | b'D' | b'E' | b'F' | b'G' | b'H' | b'I' | b'J' | b'K' | b'L'
        | b'M' | b'N' | b'O' | b'P' | b'Q' | b'R' | b'S' | b'T' | b'U' | b'V' | b'W' | b'X'
        | b'Y' | b'Z' => true,
        _ => false,
    }
}

pub fn is_lower(byte: u8) -> bool {
    match byte {
        b'a' | b'b' | b'c' | b'd' | b'e' | b'f' | b'g' | b'h' | b'i' | b'j' | b'k' | b'l'
        | b'm' | b'n' | b'o' | b'p' | b'q' | b'r' | b's' | b't' | b'u' | b'v' | b'w' | b'x'
        | b'y' | b'z' => true,
        _ => false,
    }
}

pub fn is_alpha(byte: u8) -> bool {
    match byte {
        b'A' | b'B' | b'C' | b'D' | b'E' | b'F' | b'G' | b'H' | b'I' | b'J' | b'K' | b'L'
        | b'M' | b'N' | b'O' | b'P' | b'Q' | b'R' | b'S' | b'T' | b'U' | b'V' | b'W' | b'X'
        | b'Y' | b'Z' => true,
        b'a' | b'b' | b'c' | b'd' | b'e' | b'f' | b'g' | b'h' | b'i' | b'j' | b'k' | b'l'
        | b'm' | b'n' | b'o' | b'p' | b'q' | b'r' | b's' | b't' | b'u' | b'v' | b'w' | b'x'
        | b'y' | b'z' => true,
        _ => false,
    }
}

pub fn is_digit(byte: u8) -> bool {
    match byte {
        b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'0' => true,
        _ => false,
    }
}

pub fn is_alnum(byte: u8) -> bool {
    is_alpha(byte) || is_digit(byte)
}

pub fn is_symbol(byte: u8) -> bool {
    !(is_alnum(byte) || is_white(byte))
}

pub fn is_semantic_index(text: &[u8], index: usize) -> bool {
    if index < 1 {
        return true;
    }
    let bytes = text;
    let prev = bytes[index - 1];
    let curr = bytes[index];
    if !is_upper(prev) && is_upper(curr) {
        return true;
    }
    if !is_upper(prev) && is_upper(curr) {
        return true;
    }
    if is_symbol(curr) || is_white(curr) {
        return true;
    }
    if !is_alpha(prev) && is_alpha(curr) {
        return true;
    }
    if !is_digit(prev) && is_digit(curr) {
        return true;
    }
    return false;
}

pub fn get_next_semantic_index(text: &[u8], current_index: usize) -> usize {
    for i in current_index + 1..text.len() {
        if is_semantic_index(text, i) {
            return i;
        }
    }
    return text.len();
}

pub fn get_real_idx(len: usize, idx: i32) -> usize {
    let ll = len as i32;
    ((ll + idx) % ll) as usize
}

pub fn match_char(byte1: u8, byte2: u8) -> bool {
    if !is_alpha(byte1) || !is_alpha(byte2) {
        return byte1 == byte2;
    }
    let diff = if byte1 > byte2 {
        byte1 - byte2
    } else {
        byte2 - byte1
    };
    diff == 0 || diff == 32
}

pub fn has_prefix(text: &[u8], prefix: &[u8]) -> bool {
    if text.len() < prefix.len() {
        return false;
    }
    for i in 0..prefix.len() {
        if !match_char(text[i], prefix[i]) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn basic_cases() {
        assert_eq!(get_next_semantic_index("a".as_bytes(), 0), 1);
    }
}
