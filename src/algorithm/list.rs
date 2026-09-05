pub mod linear;
pub mod treelike;

#[cfg(test)]
mod list_tests {
    use crate::ctx::Ctx;
    use num_bigint::BigUint;

    pub fn decode_empty(ctx: &Ctx) {
        assert_eq!((ctx.list.decode)(ctx, BigUint::ZERO).next(), None)
    }

    pub fn encode_empty(ctx: &Ctx) {
        assert_eq!(
            (ctx.list.encode)(ctx, &mut std::iter::empty()),
            BigUint::ZERO
        )
    }

    pub fn decode_encode(ctx: &Ctx) {
        for number in [1, 0, 2341234, 3257893, 1234, 8] {
            let number = BigUint::new_const(number);
            let new_number = (ctx.list.encode)(ctx, &mut (ctx.list.decode)(ctx, number.clone()));
            assert_eq!(number, new_number);
        }
    }

    pub fn decode_encode_sanity(ctx: &Ctx) {
        let x = (ctx.list.encode)(ctx, &mut (ctx.list.decode)(ctx, BigUint::ZERO));
        let y = (ctx.list.encode)(ctx, &mut (ctx.list.decode)(ctx, BigUint::ONE));
        assert_ne!(x, y);
    }

    pub fn encode_decode_long(ctx: &Ctx) {
        let seq: Vec<BigUint> = [
            12, 234, 523, 1, 3, 0, 0, 1598, 889231, 213, 2134, 9324, 123, 656, 0, 0,
        ]
        .into_iter()
        .map(BigUint::new_const)
        .collect();
        let code = (ctx.list.encode)(ctx, &mut seq.iter().cloned());
        let decoded: Vec<BigUint> = (ctx.list.decode)(ctx, code).collect();
        assert_eq!(seq, decoded);
    }
}
