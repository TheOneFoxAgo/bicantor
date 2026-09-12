use std::{collections::VecDeque, iter};

use crate::{
    ctx::{Ctx, TreeDencoder},
    paren::{Paren, Parentheses},
};
use num_bigint::BigUint;

pub struct WidthDencoder;
impl TreeDencoder for WidthDencoder {
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

pub struct DepthDencoder;
impl TreeDencoder for DepthDencoder {
    fn encode(&self, ctx: &Ctx<'_>, parens: &[Paren]) -> BigUint {
        // Me and claude independently arrived into this solution.
        // Tbh, it's even more fascinating than algorithm above.
        // If I wasn't qualified in parser theory, I would be completely lost.
        fn recursion(ctx: &Ctx, parens: &[Paren]) -> VecDeque<BigUint> {
            if parens.is_empty() {
                return VecDeque::new();
            }
            let mut depth = 0;
            let mut last_group_start = 0;
            for (i, p) in parens.iter().enumerate() {
                match p {
                    Paren::Open => {
                        if depth == 0 {
                            last_group_start = i;
                        }
                        depth += 1;
                    }
                    Paren::Close => depth -= 1,
                }
            }
            let first_parens = &parens[..last_group_start];
            let second_parens = &parens[last_group_start..];
            let inner = &second_parens[1..second_parens.len() - 1];
            let first = ctx
                .list
                .encode(ctx, &mut recursion(ctx, first_parens).into_iter());
            let mut rest = recursion(ctx, inner);
            rest.push_front(first);
            rest
        }
        ctx.list
            .encode(ctx, &mut recursion(ctx, parens).into_iter())
    }

    fn decode(&self, ctx: &Ctx<'_>, code: BigUint) -> Parentheses {
        let mut parens = vec![];
        fn recursion(ctx: &Ctx, mut iter: Box<dyn Iterator<Item = BigUint>>, buf: &mut Vec<Paren>) {
            if let Some(num) = iter.next() {
                recursion(ctx, ctx.list.decode(ctx, num), buf);
                buf.push(Paren::Open);
                recursion(ctx, iter, buf);
                buf.push(Paren::Close);
            }
        }
        recursion(ctx, ctx.list.decode(ctx, code), &mut parens);
        parens.try_into().unwrap()
    }
}
