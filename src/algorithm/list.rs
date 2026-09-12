use crate::ctx::{Ctx, ListDencoder};
use num_bigint::BigUint;
use std::collections::VecDeque;

pub struct LinearDencoder;
impl ListDencoder for LinearDencoder {
    fn encode(&self, ctx: &Ctx<'_>, iter: &mut dyn Iterator<Item = BigUint>) -> BigUint {
        let Some(acc) = iter.next() else {
            return BigUint::ZERO;
        };
        let (len, code) = iter.fold((0, acc), |(i, acc), n| {
            (i + 1, ctx.pair.encode(ctx, n, acc))
        });
        ctx.pair.encode(ctx, BigUint::new_const(len), code) + BigUint::ONE
    }

    fn decode(&self, ctx: &Ctx<'_>, mut code: BigUint) -> Box<dyn Iterator<Item = BigUint>> {
        let mut numbers = vec![];
        if code != BigUint::ZERO {
            code -= BigUint::ONE;
            let len;
            (len, code) = ctx.pair.decode(ctx, code);
            for _ in 0..len.try_into().unwrap_or(usize::MAX) {
                let n;
                (n, code) = ctx.pair.decode(ctx, code);
                numbers.push(n);
            }
            numbers.push(code);
            numbers.reverse();
        }
        Box::new(numbers.into_iter())
    }
}

pub struct TreelikeDencoder;
impl ListDencoder for TreelikeDencoder {
    fn encode(&self, ctx: &Ctx<'_>, iter: &mut dyn Iterator<Item = BigUint>) -> BigUint {
        let mut numbers: VecDeque<_> = iter.collect();
        if numbers.is_empty() {
            return BigUint::ZERO;
        }
        let len = numbers.len() as u32 - 1;
        while numbers.len() > 1 {
            let (Some(y), Some(x)) = (numbers.pop_back(), numbers.pop_back()) else {
                unreachable!()
            };
            let head = ctx.pair.encode(ctx, x, y);
            numbers.push_front(head);
        }
        let Some(code) = numbers.pop_back() else {
            unreachable!()
        };

        ctx.pair.encode(ctx, BigUint::new_const(len), code) + BigUint::ONE
    }

    fn decode(&self, ctx: &Ctx<'_>, mut code: BigUint) -> Box<dyn Iterator<Item = BigUint>> {
        let mut numbers = VecDeque::new();
        if code != BigUint::ZERO {
            code -= BigUint::ONE;
            let len;
            (len, code) = ctx.pair.decode(ctx, code);
            let len = len.try_into().unwrap_or(usize::MAX).saturating_add(1);
            numbers.push_back(code);
            while numbers.len() < len {
                let Some(head) = numbers.pop_front() else {
                    unreachable!()
                };
                let (x, y) = ctx.pair.decode(ctx, head);
                numbers.push_back(x);
                numbers.push_back(y);
            }
        }
        Box::new(numbers.into_iter())
    }
}
