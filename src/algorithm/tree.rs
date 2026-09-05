use std::iter;

use crate::{
    ctx::{Ctx, TreeDencoder},
    paren::{Paren, Parentheses},
};
use num_bigint::BigUint;

pub struct RecursiveTreeDencoder;
impl TreeDencoder for RecursiveTreeDencoder {
    fn encode(&self, ctx: &Ctx<'_>, parens: &[Paren]) -> BigUint {
        fn recursion(
            ctx: &Ctx,
            parens: &mut impl Iterator<Item = Paren>,
        ) -> impl Iterator<Item = BigUint> {
            iter::from_fn(move || match parens.next() {
                None | Some(Paren::Close) => None,
                Some(Paren::Open) => Some(ctx.list.encode(ctx, &mut recursion(ctx, parens))),
            })
        }
        let mut parens = parens.iter().copied();
        ctx.list.encode(ctx, &mut recursion(ctx, &mut parens))
    }

    fn decode(&self, ctx: &Ctx<'_>, code: BigUint) -> Parentheses {
        let mut parens = vec![];
        fn recursion(ctx: &Ctx, code: BigUint, buf: &mut Vec<Paren>) {
            for subcode in ctx.list.decode(ctx, code) {
                buf.push(Paren::Open);
                recursion(ctx, subcode, buf);
                buf.push(Paren::Close);
            }
        }
        recursion(ctx, code, &mut parens);
        parens.try_into().unwrap()
    }
}
