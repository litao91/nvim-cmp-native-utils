use crate::utils::byte_char;

const WORD_BOUNDALY_ORDER_FACTOR: i32 = 10;
const PREFIX_FACTOR: i32 = 8;
const NOT_FUZZY_FACTOR: i32 = 6;

#[derive(Debug, Clone)]
pub struct MatchRegion {
    pub input_match_start: usize,
    pub input_match_end: usize,
    pub word_match_start: usize,
    pub word_match_end: usize,
    pub strict_ratio: f64,
    pub fuzzy: bool,
    pub index: usize,
}
/// score
///
/// ### The score
///
///   The `score` is `matched char count` generally.
///
///   But cmp will fix the score with some of the below points so the actual score is not `matched char count`.
///
///   1. Word boundary order
///
///     cmp prefers the match that near by word-beginning.
///
///   2. Strict case
///
///     cmp prefers strict match than ignorecase match.
///
///
/// ### Matching specs.
///
///   1. Prefix matching per word boundary
///
///     `bora`         -> `border-radius` # imaginary score: 4
///      ^^~~              ^^     ~~
///
///   2. Try sequential match first
///
///     `woroff`       -> `word_offset`   # imaginary score: 6
///      ^^^~~~            ^^^  ~~~
///
///     * The `woroff`'s second `o` should not match `word_offset`'s first `o`
///
///   3. Prefer early word boundary
///
///     `call`         -> `call`          # imaginary score: 4.1
///      ^^^^              ^^^^
///     `call`         -> `condition_all` # imaginary score: 4
///      ^~~~              ^         ~~~
///
///   4. Prefer strict match
///
///     `Buffer`       -> `Buffer`        # imaginary score: 6.1
///      ^^^^^^            ^^^^^^
///     `buffer`       -> `Buffer`        # imaginary score: 6
///      ^^^^^^            ^^^^^^
///
///   5. Use remaining characters for substring match
///
///     `fmodify`        -> `fnamemodify`   # imaginary score: 1
///      ^~~~~~~             ^    ~~~~~~
///
///   6. Avoid unexpected match detection
///
///     `candlesingle` -> candle#accept#single
///      ^^^^^^~~~~~~     ^^^^^^        ~~~~~~
///
///      * The `accept`'s `a` should not match to `candle`'s `a`
///
pub fn do_match(input: &[u8], word: &[u8], words: &[&[u8]]) -> (f64, Vec<MatchRegion>) {
    if input.is_empty() {
        return (PREFIX_FACTOR as f64 + NOT_FUZZY_FACTOR as f64, Vec::new());
    }
    let mut matches = Vec::<MatchRegion>::new();
    let mut input_start_index = 0;
    let mut input_end_index = 0;
    let mut word_index = 0;
    let mut word_bound_index = 0;
    while input_end_index < input.len() && word_index < word.len() {
        let m = find_match_region(input, input_start_index, input_end_index, word, word_index);
        if m.is_some() && input_end_index < m.as_ref().unwrap().input_match_end {
            let mut m = m.unwrap();
            m.index = word_bound_index;
            input_start_index = m.input_match_start + 1;
            input_end_index = m.input_match_end;
            word_index = byte_char::get_next_semantic_index(word, m.word_match_end - 1);
            matches.push(m);
        } else {
            word_index = byte_char::get_next_semantic_index(word, word_index);
        }
        word_bound_index += 1;
    }

    if matches.is_empty() {
        return (0.0, Vec::new());
    }

    // Add prefix bonus
    let mut prefix = false;
    if matches[0].input_match_start == 0 && matches[0].word_match_start == 0 {
        prefix = true
    } else {
        for w in words {
            prefix = true;
            let mut o = 0;
            for i in matches[0].input_match_start..matches[0].input_match_end {
                if !byte_char::match_char(w[o], input[i]) {
                    prefix = false;
                    break;
                }
                o = o + 1;
            }
            if prefix {
                break;
            }
        }
    }

    // Compute prefix match score
    let mut score = if prefix { PREFIX_FACTOR } else { 0 } as f64;
    let offset = if prefix { matches[0].index as i32 } else { 0 };
    let mut idx = 0;
    for m in &matches {
        let mut s: f64 = 0.0;
        for i in std::cmp::max(idx, m.input_match_start)..m.input_match_end {
            s = s + 1.0;
            idx = i;
        }
        idx = idx + 1;
        if s > 0.0 {
            s = s as f64 * (1.0 + m.strict_ratio);
            s = s as f64
                * (1.0
                    + std::cmp::max(
                        0,
                        WORD_BOUNDALY_ORDER_FACTOR - (m.index as i32 - offset as i32 + 1),
                    ) as f64
                        / WORD_BOUNDALY_ORDER_FACTOR as f64);
            score = score + s
        }
    }

    // Check remaining input as fuzzy
    if matches.last().unwrap().input_match_end < input.len() {
        if prefix && fuzzy(input, word, &mut matches) {
            return (score, matches);
        }
        return (0.0, Vec::new());
    }

    return (score + NOT_FUZZY_FACTOR as f64, matches);
}

pub fn find_match_region<'a>(
    input: &'a [u8],
    input_start_index: usize,
    mut input_end_index: usize,
    word: &[u8],
    word_index: usize,
) -> Option<MatchRegion> {
    // determine input position (woroff -> word_offset)
    while input_start_index < input_end_index {
        if byte_char::match_char(input[input_end_index], word[word_index]) {
            break;
        }
        input_end_index = input_end_index - 1;
    }
    if input_end_index < input_start_index {
        return None;
    }
    let mut input_match_start: i32 = -1;
    let mut input_index = input_end_index;
    let mut word_offset = 0;
    let mut strict_count = 0;
    let mut match_count = 0;
    while input_index < input.len() && word_index + word_offset < word.len() {
        let c1 = input[input_index];
        let c2 = word[word_index + word_offset];
        if byte_char::match_char(c1, c2) {
            if input_match_start == -1 {
                input_match_start = input_index as i32;
            }
            strict_count += if c1 == c2 { 1 } else { 0 };
            match_count += 1;
            word_offset += 1;
        } else {
            // match end (partial region)
            if input_match_start != -1 {
                return Some(MatchRegion {
                    input_match_start: input_match_start as usize,
                    input_match_end: input_index,
                    word_match_start: word_index,
                    word_match_end: word_index + word_offset,
                    strict_ratio: strict_count as f64 / match_count as f64,
                    fuzzy: false,
                    index: 0,
                });
            } else {
                return None;
            }
        }
        input_index = input_index + 1;
    }
    if input_match_start != -1 {
        return Some(MatchRegion {
            input_match_start: input_match_start as usize,
            input_match_end: input_index,
            word_match_start: word_index,
            word_match_end: word_index + word_offset,
            strict_ratio: strict_count as f64 / match_count as f64,
            fuzzy: false,
            index: 0,
        });
    }
    return None;
}

pub fn fuzzy(input: &[u8], word: &[u8], matches: &mut Vec<MatchRegion>) -> bool {
    if let Some(last_match) = matches.last() {
        let mut input_index = last_match.input_match_end;
        for i in 0..matches.len() - 1 {
            let curr_match = &matches[i];
            let next_match = &matches[i + 1];
            let mut word_offset = 0;
            let mut word_index =
                byte_char::get_next_semantic_index(word, curr_match.word_match_end - 1);
            while word_offset + word_index < next_match.word_match_start
                && input_index < input.len()
            {
                if byte_char::match_char(word[word_index + word_offset], input[input_index]) {
                    input_index = input_index + 1;
                    word_offset = word_offset + 1;
                } else {
                    word_index = byte_char::get_next_semantic_index(word, word_index + word_offset);
                    word_offset = 0;
                }
            }
        }
        let last_input_index = input_index;
        let mut matched = false;
        let mut word_offset = 0;
        let word_index = last_match.word_match_end;
        let mut input_match_start: i32 = -1;
        let mut input_match_end: i32 = -1;
        let mut word_match_start: i32 = -1;
        let mut strict_count = 0;
        let mut match_count = 0;
        while word_offset + word_index < word.len() && input_index < input.len() {
            let c1 = word[word_index + word_offset];
            let c2 = input[input_index];
            if byte_char::match_char(c1, c2) {
                if !matched {
                    input_match_start = input_index as i32;
                    word_match_start = word_index as i32 + word_offset as i32;
                }
                matched = true;
                input_index += 1;
                strict_count += if c1 == c2 { 1 } else { 0 };
                match_count += 1;
            } else if matched {
                input_index = last_input_index;
                input_match_end = input_index as i32;
            }
            word_offset = word_offset + 1;
        }
        if input_index >= input.len() {
            matches.push(MatchRegion {
                input_match_start: input_match_start as usize,
                input_match_end: input_match_end as usize,
                word_match_start: word_match_start as usize,
                word_match_end: word_index + word_offset,
                strict_ratio: strict_count as f64 / match_count as f64,
                fuzzy: true,
                index: 0,
            });
            return true;
        }
        return false;
    }
    return false;
    // remaining text fuzzy match
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic() {
        {
            let r = do_match("".as_bytes(), "a".as_bytes(), &[]);
            println!("'' match 'a' {:?}", r);
            assert!(r.0 >= 1.0);
        }
        {
            let r = do_match("a".as_bytes(), "a".as_bytes(), &[]);
            println!("'a' match 'a': {:?}", r);
            assert!((r.0 - 17.8).abs() < 0.001);
        }

        {
            let r = do_match("ab".as_bytes(), "a".as_bytes(), &[]);
            println!("'ab' match 'a': {:?}", r);
            assert!(r.0 == 0.0);
        }

        {
            let r = do_match("ab".as_bytes(), "ab".as_bytes(), &[]);
            println!("'ab' match 'ab': {:?}", r);
            assert!((r.0 - 21.6).abs() < 0.0001);
        }

        {
            let r = do_match("ab".as_bytes(), "a_b".as_bytes(), &[]);
            println!("'ab' match 'a_b': {:?}", r);
            assert!((r.0 - 21.2).abs() < 0.0001);
        }

        {
            let r = do_match("ab".as_bytes(), "a_b_c".as_bytes(), &[]);
            println!("'ab' match 'a_b_c': {:?}", r);
            assert!((r.0 - 21.2).abs() < 0.0001);
        }

        {
            let lhs = "bora";
            let rhs = "border-raidus";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            assert_eq!(r.1.len(), 2);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "bor".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "bor".as_bytes()
            );

            assert_eq!(
                &lhs.as_bytes()[r.1[1].input_match_start..r.1[1].input_match_end],
                "ra".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[1].word_match_start..r.1[1].word_match_end],
                "ra".as_bytes()
            );
            assert!((r.0 - 28.8).abs() < 0.0001);
        }

        {
            let lhs = "woroff";
            let rhs = "word_offset";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            assert_eq!(r.1.len(), 2);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "wor".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "wor".as_bytes()
            );

            assert_eq!(
                &lhs.as_bytes()[r.1[1].input_match_start..r.1[1].input_match_end],
                "off".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[1].word_match_start..r.1[1].word_match_end],
                "off".as_bytes()
            );
            assert!((r.0 - 35.6).abs() < 0.0001);
        }

        {
            let lhs = "call";
            let rhs = "call";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "call".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "call".as_bytes()
            );
            assert!((r.0 - 29.2).abs() < 0.0001);
        }
        {
            let lhs = "call";
            let rhs = "condition_all";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "c".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "c".as_bytes()
            );

            assert_eq!(
                &lhs.as_bytes()[r.1[1].input_match_start..r.1[1].input_match_end],
                "all".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[1].word_match_start..r.1[1].word_match_end],
                "all".as_bytes()
            );
            assert!((r.0 - 28.0).abs() < 0.0001);
        }
        {
            let lhs = "Buffer";
            let rhs = "Buffer";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "Buffer".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "Buffer".as_bytes()
            );
            assert!((r.0 - 36.8).abs() < 0.0001);
        }
        {
            let lhs = "Buffer";
            let rhs = "buffer";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "Buffer".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "buffer".as_bytes()
            );
            assert!((r.0 - 34.9).abs() < 0.0001);
        }

        {
            let lhs = "fmodify";
            let rhs = "fnamemodify";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            assert_eq!(r.1.len(), 2);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "f".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "f".as_bytes()
            );
            assert_eq!(
                &lhs.as_bytes()[r.1[1].input_match_start..r.1[1].input_match_end],
                "".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[1].word_match_start..r.1[1].word_match_end],
                "memodify".as_bytes()
            );
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 11.8).abs() < 0.0001);
        }

        {
            let lhs = "candlesingle";
            let rhs = "candle#accept#single";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "candle".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "candle".as_bytes()
            );
            assert_eq!(
                &lhs.as_bytes()[r.1[1].input_match_start..r.1[1].input_match_end],
                "single".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[1].word_match_start..r.1[1].word_match_end],
                "single".as_bytes()
            );
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 54.8).abs() < 0.0001);
        }

        {
            let lhs = "conso";
            let rhs = "console";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 33.0).abs() < 0.0001);
        }

        {
            let lhs = "conso";
            let rhs = "ConstantSourceNode";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 30.0).abs() < 0.0001);
        }
        {
            let lhs = "var_";
            let rhs = "var_dump";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 29.2).abs() < 0.0001);
        }
        {
            let lhs = "my_";
            let rhs = "my_awesome_varible";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 25.4).abs() < 0.0001);
        }
        {
            let lhs = "my_";
            let rhs = "completion_matching_strategy";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 0.0).abs() < 0.0001);
        }
        {
            let lhs = "luacon";
            let rhs = "lua_context";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 35.6).abs() < 0.0001);
        }
        {
            let lhs = "call";
            let rhs = "calc";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 0.0).abs() < 0.0001);
        }
        {
            let lhs = "vi";
            let rhs = "void#";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 11.8).abs() < 0.0001);
        }

        {
            let lhs = "vo";
            let rhs = "void#";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 21.6).abs() < 0.0001);
        }

        {
            let lhs = "usela";
            let rhs = "useLayoutEffect";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 31.1).abs() < 0.0001);
        }

        {
            let lhs = "usela";
            let rhs = "useDataLayer";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("'{}' match '{}': {:?}", lhs, rhs, r);
            assert!((r.0 - 30.5).abs() < 0.0001);
        }
        {
            let lhs = "true";
            let rhs = "v:true";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &["true".as_bytes()]);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "true".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "true".as_bytes()
            );
            assert!((r.0 - 29.2).abs() < 0.0001);
        }
        {
            let lhs = "g";
            let rhs = "get";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &["get".as_bytes()]);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "g".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "g".as_bytes()
            );
            assert!((r.0 - 17.8).abs() < 0.0001);
        }

        {
            let lhs = "g";
            let rhs = "dein#get";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &["dein#get".as_bytes()]);
            println!("{:?}", r);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "g".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "g".as_bytes()
            );
            assert!((r.0 - 9.4).abs() < 0.0001);
        }

        {
            let lhs = "2";
            let rhs = "[[2021";
            let r = do_match(lhs.as_bytes(), rhs.as_bytes(), &[]);
            println!("{:?}", r);
            assert_eq!(
                &lhs.as_bytes()[r.1[0].input_match_start..r.1[0].input_match_end],
                "2".as_bytes()
            );
            assert_eq!(
                &rhs.as_bytes()[r.1[0].word_match_start..r.1[0].word_match_end],
                "2".as_bytes()
            );
            assert!((r.0 - 9.4).abs() < 0.0001);
        }
        // assert!(false);
    }
}
