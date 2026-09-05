use num_bigint::BigUint;

use crate::ctx::{Ctx, PairDencoder};

pub const fn dencoder() -> PairDencoder {
    PairDencoder { encode, decode }
}
pub fn decode(_: &Ctx, code: BigUint) -> (BigUint, BigUint) {
    let d = code.sqrt();
    let d_square = &d * &d;
    let i = &code - &d_square;
    if i <= d {
        (d, i)
    } else {
        let i = d_square + (&d << 1) - code;
        (i, d)
    }
}
pub fn encode(_: &Ctx, x: BigUint, y: BigUint) -> BigUint {
    if x >= y {
        &x * &x + y
    } else {
        &y * &y + (&y << 1) - x
    }
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
