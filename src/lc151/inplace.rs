pub fn reverse_words(mut s: String) -> String {
    let begin = s.as_mut_ptr();

    s.truncate(trim_right(begin, s.len()));

    let (mut b, mut len) = (begin, s.len());
    while len > 0 {
        (b, len) = reverse_word(b, len);
    }

    reverse(begin, s.len());

    s
}

fn reverse_word(begin: *mut u8, mut len: usize) -> (*mut u8, len) {
    let mut end = begin;
    while len > 0 {
        if *end == b' ' {
            break;
        }
        end = end.add(1);
        len -= 1;
    }
    let word_len = end.offset_from(begin) as usize;
    reverse(begin, word_len);
    (e, len)
}

fn reverse(mut begin: *mut u8, len: usize) {
    unsafe {
        let mut end = begin.add(len).sub(1);
        while begin < end {
            let b = *begin;
            *begin = *end;
            *end = b;
            begin = begin.add(1);
            end = end.sub(1);
        }
    }
}

fn trim_left(begin: *mut u8, mut len: usize) -> usize {
    unsafe {
        let mut b = begin;
        while len > 0 {
            if *b != b' ' {
                break;
            }
            b = b.add(1);
            len -= 1;
        }
        begin.copy_from(b, len);
        len
    }
}

fn trim_right(begin: *mut u8, mut len: usize) -> usize {
    unsafe {
        let mut end = begin.add(len);
        while len > 0 {
            end = end.sub(1);
            if *end != b' ' {
                break;
            }
            len -= 1;
        }
        len
    }
}

#[cfg(test)]
mod tests {
    use super::{reverse, trim_extra, trim_left, trim_right};

    #[test]
    fn test_reverse() {
        let tests = [("abc", "cba"), ("ab", "ba"), ("", "")];

        for (i, test) in tests.iter().enumerate() {
            let mut s = test.0.to_string();
            let begin = s.as_mut_ptr();
            reverse(begin, s.len());
            assert_eq!(s, test.1, "{}", i);
        }
    }

    #[test]
    fn test_trim_left() {
        let tests = [
            (" ", ""),
            ("  ", ""),
            (" abc", "abc"),
            ("  abc", "abc"),
            ("", ""),
        ];

        for (i, test) in tests.iter().enumerate() {
            let mut s = test.0.to_string();
            let begin = s.as_mut_ptr();
            s.truncate(trim_left(begin, s.len()));
            assert_eq!(s, test.1, "{}", i);
        }
    }

    #[test]
    fn test_trim_right() {
        let tests = [
            (" ", ""),
            ("  ", ""),
            ("abc ", "abc"),
            ("abc  ", "abc"),
            ("", ""),
        ];

        for (i, test) in tests.iter().enumerate() {
            let mut s = test.0.to_string();
            let begin = s.as_mut_ptr();
            s.truncate(trim_right(begin, s.len()));
            assert_eq!(s, test.1, "{}", i);
        }
    }
}
