#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
}

impl Move {
    pub const NULL: Move = Move {
        from: (std::u8::MAX, std::u8::MAX),
        to: (std::u8::MAX, std::u8::MAX),
    };
}

impl TryFrom<&str> for Move {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s == "0000" {
            return Ok(Move::NULL);
        }
        // g1g2 -> (6, 6), (6, 5)
        let mut chars = s.chars();
        let from = (
            chars.next().unwrap() as u8 - b'a',
            6 - (chars.next().unwrap() as u8 - b'1'),
        );
        if let Some(c) = chars.next() {
            let to = (c as u8 - b'a', 6 - (chars.next().unwrap() as u8 - b'1'));
            Ok(Move { from, to })
        } else {
            Ok(Move { from, to: from })
        }
    }

    type Error = ();
}

impl From<Move> for String {
    fn from(m: Move) -> Self {
        (&m).into()
    }
}

impl From<&Move> for String {
    fn from(value: &Move) -> Self {
        if *value == Move::NULL {
            return "0000".to_owned();
        }
        let mut s = String::with_capacity(4);
        s.push((value.from.0 + b'a') as char);
        s.push((value.from.1 + b'1') as char);
        s.push((value.to.0 + b'a') as char);
        s.push((value.to.1 + b'1') as char);
        s
    }
}
