use std::iter;

use crate::{
    ctx::{Ctx, TreeDencoder},
    paren::{Paren, Parentheses},
};
use num_bigint::BigUint;

pub const fn dencoder() -> TreeDencoder {
    TreeDencoder { encode, decode }
}

pub fn decode(ctx: &Ctx, code: BigUint) -> Parentheses {
    let mut parens = vec![];
    fn recursion(ctx: &Ctx, code: BigUint, buf: &mut Vec<Paren>) {
        for subcode in (ctx.list.decode)(ctx, code) {
            buf.push(Paren::Open);
            recursion(ctx, subcode, buf);
            buf.push(Paren::Close);
        }
    }
    recursion(ctx, code, &mut parens);
    parens.try_into().unwrap()
}

pub fn encode(ctx: &Ctx, parens: &[Paren]) -> BigUint {
    fn recursion(
        ctx: &Ctx,
        parens: &mut impl Iterator<Item = Paren>,
    ) -> impl Iterator<Item = BigUint> {
        iter::from_fn(move || match parens.next() {
            None | Some(Paren::Close) => None,
            Some(Paren::Open) => Some((ctx.list.encode)(ctx, &mut recursion(ctx, parens))),
        })
    }
    let mut parens = parens.iter().copied();
    (ctx.list.encode)(ctx, &mut recursion(ctx, &mut parens))
}

#[cfg(test)]
mod tests {
    use crate::algorithm::{list::linear, pair::diagonal};

    use super::*;

    static CTX: Ctx = Ctx {
        pair: diagonal::dencoder(),
        list: linear::dencoder(),
        tree: dencoder(),
    };
    #[test]
    fn decode_encode_parens() {
        let code = BigUint::new_const(19);
        let parens = (CTX.tree.decode)(&CTX, code.clone());
        let new_code = (CTX.tree.encode)(&CTX, parens.as_slice());
        assert_eq!(code, new_code);
    }

    #[test]
    fn encode_decode_parens() {
        let parens: Parentheses = "(()(()))()".parse().unwrap();
        let code = (CTX.tree.encode)(&CTX, parens.as_slice());
        let new_parens = (CTX.tree.decode)(&CTX, code);
        assert_eq!(parens, new_parens);
    }
}
