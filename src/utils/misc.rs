pub fn to_vimindex(text: &str, utfindex: usize) -> usize {
    let mut r = 0;
    let mut chars = text.chars();
    for i in 0..utfindex {
        if let Some(c) = chars.next() {
            r += c.len_utf8();
        }
    }
    return r + 1;
}

pub fn to_utfindex(text: &str, mut vim_index: usize) -> usize {
    let mut utfindex = 0;
    for c in text.chars() {
        vim_index -= c.len_utf8();
        utfindex += 1;
        if vim_index <= 0 {
            break;
        }
    }

    utfindex
}

