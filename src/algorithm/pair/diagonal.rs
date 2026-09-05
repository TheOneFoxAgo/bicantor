use num_bigint::BigUint;

use crate::ctx::{Ctx, PairDencoder};

pub const fn dencoder() -> PairDencoder {
    PairDencoder { encode, decode }
}
pub fn decode(_: &Ctx, code: BigUint) -> (BigUint, BigUint) {
    let d = ((BigUint::ONE + BigUint::new_const(8) * &code).sqrt() - BigUint::ONE) >> 1;
    let y = code - ((&d * (&d + BigUint::ONE)) >> 1);
    let x = d - &y;
    (x, y)
}
pub fn encode(_: &Ctx, x: BigUint, y: BigUint) -> BigUint {
    let d = x + &y;
    let d_p1 = &d + BigUint::ONE;
    ((d * d_p1) >> 1) + y
}

#[cfg(test)]
mod tests {
    use crate::ctx::Ctx;

    use super::super::pair_tests;
    use super::*;
    static CTX: Ctx = Ctx {
        pair: dencoder(),
        ..Ctx::SHIM
    };
    #[test]
    fn decode_encode() {
        pair_tests::decode_encode(&CTX);
    }
    #[test]
    fn encode_decode() {
        pair_tests::encode_decode(&CTX);
    }
}
