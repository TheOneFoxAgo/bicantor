use std::{
    fmt::{Debug, Display, Write},
    str::FromStr,
};

use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Paren {
    Open,
    Close,
}

#[derive(Clone, PartialEq)]
pub struct Parentheses(Vec<Paren>);
impl Parentheses {
    pub fn as_slice(&self) -> &[Paren] {
        &self.0
    }
}
#[derive(Error, Debug)]
pub enum ParenParsingError {
    #[error("Wrong symbol \"{character}\" at index {idx}")]
    WrongCharacter { character: char, idx: usize },
    #[error("Unmatched opening parenthesis at index {idx}.")]
    UnmatchedOpeningParen { idx: usize },
    #[error("Unmatched closing parenthesis at index {idx}.")]
    UnmatchedClosingParen { idx: usize },
}
impl Display for Parentheses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for paren in self.as_slice() {
            f.write_char(match paren {
                Paren::Open => '(',
                Paren::Close => ')',
            })?
        }
        Ok(())
    }
}
impl Debug for Parentheses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
impl TryFrom<Vec<Paren>> for Parentheses {
    type Error = ParenParsingError;
    fn try_from(value: Vec<Paren>) -> Result<Self, Self::Error> {
        fn check(iter: impl Iterator<Item = Paren>) -> Result<(), usize> {
            let mut opened = 0;
            for (i, brace) in iter.enumerate() {
                match brace {
                    Paren::Open => opened += 1,
                    Paren::Close => {
                        if opened == 0 {
                            return Err(i);
                        } else {
                            opened -= 1
                        }
                    }
                }
            }
            Ok(())
        }
        if let Err(idx) = check(value.iter().copied()) {
            return Err(Self::Error::UnmatchedClosingParen { idx });
        }
        if let Err(i) = check(value.iter().rev().map(|p| match p {
            Paren::Open => Paren::Close,
            Paren::Close => Paren::Open,
        })) {
            return Err(Self::Error::UnmatchedOpeningParen {
                idx: value.len() - 1 - i,
            });
        }
        Ok(Self(value))
    }
}
impl FromStr for Parentheses {
    type Err = ParenParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parentheses: Result<Vec<Paren>, Self::Err> = s
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '(' => Ok(Paren::Open),
                ')' => Ok(Paren::Close),
                _ => Err(Self::Err::WrongCharacter {
                    character: c,
                    idx: i,
                }),
            })
            .collect();
        parentheses?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn wrong_character() {
        let p: Result<Parentheses, ParenParsingError> = "(a()".parse();
        assert_matches!(
            p,
            Err(ParenParsingError::WrongCharacter {
                idx: 1,
                character: 'a'
            })
        )
    }

    #[test]
    fn incorrect_opening() {
        let p: Result<Parentheses, ParenParsingError> = "()(()".parse();
        assert_matches!(p, Err(ParenParsingError::UnmatchedOpeningParen { idx: 2 }))
    }

    #[test]
    fn incorrect_closing() {
        let p: Result<Parentheses, ParenParsingError> = "())()".parse();
        assert_matches!(p, Err(ParenParsingError::UnmatchedClosingParen { idx: 2 }))
    }
}
