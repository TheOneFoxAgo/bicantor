use std::collections::VecDeque;

use num_bigint::BigUint;

use crate::ctx::{Ctx, ListDencoder};

pub const fn dencoder() -> ListDencoder {
    ListDencoder { encode, decode }
}

pub fn decode(ctx: &Ctx, mut code: BigUint) -> Box<dyn Iterator<Item = BigUint>> {
    let mut numbers = VecDeque::new();
    if code != BigUint::ZERO {
        code -= BigUint::ONE;
        let len;
        (len, code) = (ctx.pair.decode)(ctx, code);
        let len = len.try_into().unwrap_or(usize::MAX).saturating_add(1);
        numbers.push_back(code);
        while numbers.len() < len {
            let Some(head) = numbers.pop_front() else {
                unreachable!()
            };
            let (x, y) = (ctx.pair.decode)(ctx, head);
            numbers.push_back(x);
            numbers.push_back(y);
        }
    }
    Box::new(numbers.into_iter())
}

pub fn encode(ctx: &Ctx, iter: &mut dyn Iterator<Item = BigUint>) -> BigUint {
    let mut numbers: VecDeque<_> = iter.collect();
    if numbers.is_empty() {
        return BigUint::ZERO;
    }
    let len = numbers.len() as u32 - 1;
    while numbers.len() > 1 {
        let (Some(y), Some(x)) = (numbers.pop_back(), numbers.pop_back()) else {
            unreachable!()
        };
        let head = (ctx.pair.encode)(ctx, x, y);
        numbers.push_front(head);
    }
    let Some(code) = numbers.pop_back() else {
        unreachable!()
    };

    (ctx.pair.encode)(ctx, BigUint::new_const(len), code) + BigUint::ONE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorithm::{list::list_tests, pair::diagonal},
        ctx::Ctx,
    };

    static CTX: Ctx = Ctx {
        pair: diagonal::dencoder(),
        list: dencoder(),
        ..Ctx::SHIM
    };

    #[test]
    fn decode_empty() {
        list_tests::decode_empty(&CTX);
    }
    #[test]
    fn encode_empty() {
        list_tests::encode_empty(&CTX);
    }
    #[test]
    fn decode_encode() {
        list_tests::decode_encode(&CTX);
    }
    #[test]
    fn decode_encode_sanity() {
        list_tests::decode_encode_sanity(&CTX);
    }
    #[test]
    fn encode_decode_long() {
        list_tests::encode_decode_long(&CTX);
    }
}
