use State::*;

pub fn is_number(s: String) -> bool {
    let mut state = Start;

    for c in s.chars() {
        state = state.next(c);
        if state == Invalid {
            return false;
        }
    }

    matches!(
        state,
        Integer | DotAfterInteger | Decimal | IntegerAfterE | End
    )
}

#[derive(PartialEq)]
enum State {
    Start,
    SignBeforeE,
    Integer,
    DotBeforeInteger,
    DotAfterInteger,
    Decimal,
    E,
    SignAfterE,
    IntegerAfterE,
    End,
    Invalid,
}

impl State {
    fn next(&self, c: char) -> Self {
        match self {
            Start => match c {
                ' ' => Start,
                '+' | '-' => SignBeforeE,
                '0'..='9' => Integer,
                '.' => DotBeforeInteger,
                _ => Invalid,
            },
            SignBeforeE => match c {
                '0'..='9' => Integer,
                '.' => DotBeforeInteger,
                _ => Invalid,
            },
            Integer => match c {
                '0'..='9' => Integer,
                '.' => DotAfterInteger,
                'e' | 'E' => E,
                ' ' => End,
                _ => Invalid,
            },
            DotBeforeInteger => match c {
                '0'..='9' => Decimal,
                _ => Invalid,
            },
            DotAfterInteger => match c {
                '0'..='9' => Decimal,
                'e' | 'E' => E,
                ' ' => End,
                _ => Invalid,
            },
            Decimal => match c {
                '0'..='9' => Decimal,
                'e' | 'E' => E,
                ' ' => End,
                _ => Invalid,
            },
            E => match c {
                '+' | '-' => SignAfterE,
                '0'..='9' => IntegerAfterE,
                _ => Invalid,
            },
            SignAfterE => match c {
                '0'..='9' => IntegerAfterE,
                _ => Self::Invalid,
            },
            IntegerAfterE => match c {
                '0'..='9' => IntegerAfterE,
                ' ' => End,
                _ => Invalid,
            },
            End => match c {
                ' ' => End,
                _ => Invalid,
            },
            Invalid => Invalid,
        }
    }
}
