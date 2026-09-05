use num_bigint::BigUint;

use crate::paren::{Paren, Parentheses};

pub struct Ctx {
    pub pair: PairDencoder,
    pub list: ListDencoder,
    pub tree: TreeDencoder,
}

pub struct Dencoder<E, D> {
    pub encode: E,
    pub decode: D,
}

pub type PairDencoder =
    Dencoder<fn(&Ctx, BigUint, BigUint) -> BigUint, fn(&Ctx, BigUint) -> (BigUint, BigUint)>;
pub type ListDencoder = Dencoder<
    fn(&Ctx, &mut dyn Iterator<Item = BigUint>) -> BigUint,
    fn(&Ctx, BigUint) -> Box<dyn Iterator<Item = BigUint>>,
>;
pub type TreeDencoder = Dencoder<fn(&Ctx, &[Paren]) -> BigUint, fn(&Ctx, BigUint) -> Parentheses>;

// Shims for testing purposes
#[cfg(test)]
mod shims {
    use super::*;
    impl Ctx {
        pub const SHIM: Self = Self {
            pair: PairDencoder::SHIM,
            list: ListDencoder::SHIM,
            tree: TreeDencoder::SHIM,
        };
    }
    impl PairDencoder {
        pub const SHIM: Self = Self {
            encode: |_, _, _| unimplemented!(),
            decode: |_, _| unimplemented!(),
        };
    }
    impl ListDencoder {
        pub const SHIM: Self = Self {
            encode: |_, _| unimplemented!(),
            decode: |_, _| unimplemented!(),
        };
    }
    impl TreeDencoder {
        pub const SHIM: Self = Self {
            encode: |_, _| unimplemented!(),
            decode: |_, _| unimplemented!(),
        };
    }
}
