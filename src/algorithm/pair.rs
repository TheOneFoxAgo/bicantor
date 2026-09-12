use crate::ctx::{Ctx, PairDencoder};
use num_bigint::BigUint;

pub struct DiagonalDencoder;
impl PairDencoder for DiagonalDencoder {
    fn encode(&self, _: &Ctx<'_>, x: BigUint, y: BigUint) -> BigUint {
        let d = x + &y;
        let d_p1 = &d + BigUint::ONE;
        ((d * d_p1) >> 1) + y
    }

    fn decode(&self, _: &Ctx<'_>, code: BigUint) -> (BigUint, BigUint) {
        let d = ((BigUint::ONE + BigUint::new_const(8) * &code).sqrt() - BigUint::ONE) >> 1;
        let y = code - ((&d * (&d + BigUint::ONE)) >> 1);
        let x = d - &y;
        (x, y)
    }
}

pub struct SquareDencoder;
impl PairDencoder for SquareDencoder {
    fn encode(&self, _: &Ctx<'_>, x: BigUint, y: BigUint) -> BigUint {
        if x >= y {
            &x * &x + y
        } else {
            &y * &y + (&y << 1) - x
        }
    }

    fn decode(&self, _: &Ctx<'_>, code: BigUint) -> (BigUint, BigUint) {
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
}
