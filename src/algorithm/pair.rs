pub mod diagonal;
pub mod square;

#[cfg(test)]
mod pair_tests {
    use num_bigint::BigUint;

    #[allow(unused_imports)]
    use super::*;
    use crate::ctx::Ctx;

    pub fn decode_encode(ctx: &Ctx) {
        for i in [0, 1, 4, 32, 12] {
            let number = BigUint::new_const(i);
            let (x, y) = (ctx.pair.decode)(&ctx, number.clone());
            let new_number = (ctx.pair.encode)(&ctx, x, y);
            assert_eq!(number, new_number);
        }
    }

    pub fn encode_decode(ctx: &Ctx) {
        for (x, y) in [(0, 0), (3, 2), (10, 0), (0, 10), (5, 12)] {
            let (x, y) = (BigUint::new_const(x), BigUint::new_const(y));
            let number = (ctx.pair.encode)(ctx, x.clone(), y.clone());
            let (new_x, new_y) = (ctx.pair.decode)(ctx, number.clone());
            assert_eq!(x, new_x);
            assert_eq!(y, new_y);
        }
    }
}
