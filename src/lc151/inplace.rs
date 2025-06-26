pub fn reverse_words(mut s: String) -> String {
    let begin = s.as_mut_ptr();
    let mut end = unsafe { begin.add(s.len()) };

    s
}

fn reverse(mut begin: *mut u8, mut end: *mut u8) {
    unsafe {
        end = end.sub(1);
        while begin < end {
            let b = *begin;
            *begin = *end;
            *end = b;
            begin = begin.add(1);
            end = end.sub(1);
        }
    }
}

fn trim_left(begin: *mut u8, end: *mut u8) -> usize {
    unsafe {
        let mut b = begin;
        while b < end {
            if *b != b' ' {
                break;
            }
            b = b.add(1);
        }
        let len = end.offset_from(b) as usize;
        begin.copy_from(b, len);
        len
    }
}

fn trim_right(begin: *mut u8, end: *mut u8) -> usize {
    unsafe {
        let mut e = end;
        while begin < e {
            e = e.sub(1);
            if *e != b' ' {
                e = e.add(1);
                break;
            }
        }
        e.offset_from(begin) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::{reverse, trim_left, trim_right};

    #[test]
    fn test_reverse() {
        let tests = [("abc", "cba"), ("ab", "ba"), ("", "")];

        for (i, test) in tests.iter().enumerate() {
            let mut s = test.0.to_string();
            let begin = s.as_mut_ptr();
            let end = unsafe { begin.add(s.len()) };
            reverse(begin, end);
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
            let end = unsafe { begin.add(s.len()) };
            s.truncate(trim_left(begin, end));
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
            let end = unsafe { begin.add(s.len()) };
            s.truncate(trim_right(begin, end));
            assert_eq!(s, test.1, "{}", i);
        }
    }
}
