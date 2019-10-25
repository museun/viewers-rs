pub fn timeout(s: &str) -> Result<u64, Error> {
    let (mut order, mut buf) = (Order(None), Buf(vec![]));
    let mut iter = s.chars().peekable();
    let mut acc = 0;

    macro_rules! check_empty {
        ($mag:expr) => {{
            if buf.is_empty() {
                return Err(Error::InvalidData);
            }
            order.verify($mag)?
        }};
    }

    while let Some(left) = iter.next() {
        let magnitude = match (left, iter.peek()) {
            ('s', ..) => check_empty!(Magnitude::Second),
            ('m', ..) => check_empty!(Magnitude::Minute),
            ('h', ..) => check_empty!(Magnitude::Hour),
            (c, Some(..)) if c.is_ascii_digit() => {
                if buf.is_empty() && c == '0' {
                    return Err(Error::InvalidData);
                }
                buf.append(c);
                continue;
            }
            _ => continue,
        };

        acc += match buf.parse(magnitude) {
            Some(d) => d,
            None => break,
        }
    }

    return Ok(acc);

    struct Buf(Vec<char>);
    impl Buf {
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        fn append(&mut self, ch: char) {
            self.0.push(ch)
        }
        fn parse(&mut self, magnitude: Magnitude) -> Option<u64> {
            if self.is_empty() {
                return None;
            }

            Some(
                self.0
                    .drain(..)
                    .filter_map(|c| c.to_digit(10).map(u64::from))
                    .fold(0, |a, c| 10 * a + c)
                    * magnitude.to_s(),
            )
        }
    }

    struct Order(Option<Magnitude>);
    impl Order {
        fn verify(&mut self, magnitude: Magnitude) -> Result<Magnitude, Error> {
            match self.0 {
                Some(a) if a > magnitude => self.0.replace(magnitude),
                Some(a) if a == magnitude => return Err(Error::AlreadySeen),
                None => self.0.replace(magnitude),
                _ => return Err(Error::OutOfOrder),
            };
            Ok(magnitude)
        }
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    enum Magnitude {
        Second,
        Minute,
        Hour,
    }
    impl Magnitude {
        fn to_s(self) -> u64 {
            match self {
                Magnitude::Second => 1,
                Magnitude::Minute => 60,
                Magnitude::Hour => 60 * 60,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    AlreadySeen,
    OutOfOrder,
    InvalidData,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadySeen => <_ as std::fmt::Display>::fmt("Already seen", f),
            Error::OutOfOrder => <_ as std::fmt::Display>::fmt("Out of order", f),
            Error::InvalidData => <_ as std::fmt::Display>::fmt("Invalid data", f),
        }
    }
}
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout() {
        let tests = &[
            ("1s", 1),
            ("1m", 60),
            ("1h", 60 * 60),
            ("1h 1m 1s", (60 * 60) + 60 + 1),
            ("1h 1s", (60 * 60) + 1),
            ("30m 59s", (30 * 60) + 59),
            ("1s foobar", 1),
        ];

        for (input, expected) in tests {
            assert_eq!(
                super::timeout(&input).unwrap(),
                *expected,
                "input: {}",
                input
            );
        }

        let tests = &[
            ("1s 1m", Error::OutOfOrder),
            ("1s 1s", Error::AlreadySeen),
            ("0s", Error::InvalidData),
        ];

        for (input, expected) in tests {
            assert_eq!(
                super::timeout(dbg!(&input)).unwrap_err(),
                *expected,
                "input: {}",
                input
            )
        }
    }
}
