use std::iter;

use super::list;
use crate::paren::{Paren, Parentheses};
use num_bigint::BigUint;

pub fn decode(code: BigUint) -> Parentheses {
    let mut parens = vec![];
    fn recursion(code: BigUint, buf: &mut Vec<Paren>) {
        for subcode in list::decode(code) {
            buf.push(Paren::Open);
            recursion(subcode, buf);
            buf.push(Paren::Close);
        }
    }
    recursion(code, &mut parens);
    parens.try_into().unwrap()
}

pub fn encode(parens: &[Paren]) -> BigUint {
    fn recursion(parens: &mut impl Iterator<Item = Paren>) -> impl Iterator<Item = BigUint> {
        iter::from_fn(move || match parens.next() {
            None | Some(Paren::Close) => None,
            Some(Paren::Open) => Some(list::encode(recursion(parens))),
        })
    }
    let mut parens = parens.iter().copied();
    list::encode(recursion(&mut parens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode_parens() {
        let code = BigUint::new_const(19);
        let parens = decode(code.clone());
        let new_code = encode(parens.as_slice());
        assert_eq!(code, new_code);
    }

    #[test]
    fn encode_decode_parens() {
        let parens: Parentheses = "(()(()))()".parse().unwrap();
        let code = encode(parens.as_slice());
        let new_parens = decode(code);
        assert_eq!(parens, new_parens);
    }
}
