use num_bigint::BigUint;

use crate::ctx::{Ctx, ListDencoder};

pub const fn dencoder() -> ListDencoder {
    ListDencoder { encode, decode }
}

pub fn decode(ctx: &Ctx, mut code: BigUint) -> Box<dyn Iterator<Item = BigUint>> {
    let mut numbers = vec![];
    if code != BigUint::ZERO {
        code -= BigUint::ONE;
        let len;
        (len, code) = (ctx.pair.decode)(ctx, code);
        for _ in 0..len.try_into().unwrap_or(usize::MAX) {
            let n;
            (n, code) = (ctx.pair.decode)(ctx, code);
            numbers.push(n);
        }
        numbers.push(code);
        numbers.reverse();
    }
    Box::new(numbers.into_iter())
}

pub fn encode(ctx: &Ctx, iter: &mut dyn Iterator<Item = BigUint>) -> BigUint {
    let Some(acc) = iter.next() else {
        return BigUint::ZERO;
    };
    let (len, code) = iter.fold((0, acc), |(i, acc), n| {
        (i + 1, (ctx.pair.encode)(ctx, n, acc))
    });
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
