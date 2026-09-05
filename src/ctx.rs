use num_bigint::BigUint;

use crate::paren::{Paren, Parentheses};

pub struct Ctx<'a> {
    pub pair: &'a dyn PairDencoder,
    pub list: &'a dyn ListDencoder,
    pub tree: &'a dyn TreeDencoder,
}

pub trait PairDencoder {
    fn encode(&self, ctx: &Ctx<'_>, first: BigUint, second: BigUint) -> BigUint;
    fn decode(&self, ctx: &Ctx<'_>, code: BigUint) -> (BigUint, BigUint);
}
pub trait ListDencoder {
    fn encode(&self, ctx: &Ctx<'_>, iter: &mut dyn Iterator<Item = BigUint>) -> BigUint;
    fn decode(&self, ctx: &Ctx<'_>, code: BigUint) -> Box<dyn Iterator<Item = BigUint>>;
}
pub trait TreeDencoder {
    fn encode(&self, ctx: &Ctx<'_>, parens: &[Paren]) -> BigUint;
    fn decode(&self, ctx: &Ctx<'_>, code: BigUint) -> Parentheses;
}
