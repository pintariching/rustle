#[derive(PartialEq)]
#[repr(u8)]
pub enum Bracket {
    SquareBracketOpen = b'[',
    SquareBracketClose = b']',
    CurlyBracketOpen = b'{',
    CurlyBracketClose = b'}',
}

impl Bracket {
    pub fn is_open(&self) -> bool {
        self == &Bracket::SquareBracketOpen || self == &Bracket::CurlyBracketOpen
    }

    pub fn is_closed(&self) -> bool {
        self == &Bracket::SquareBracketClose || self == &Bracket::CurlyBracketClose
    }

    pub fn is_pair(&self, close: &Bracket) -> bool {
        (self == &Bracket::SquareBracketOpen && close == &Bracket::SquareBracketClose)
            || (self == &Bracket::CurlyBracketOpen && close == &Bracket::CurlyBracketClose)
    }

    pub fn get_bracket_close(&self) -> Bracket {
        match self {
			Bracket::SquareBracketOpen => Bracket::SquareBracketClose,
			Bracket::CurlyBracketOpen => Bracket::CurlyBracketClose,
			_ => panic!("Bracket needs to be either Bracket::SquareBracketOpen or Bracket::CurlyBracketOpen")
		}
    }
}
