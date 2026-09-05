pub mod list;
pub mod pair;
pub mod tree;

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    use crate::{
        algorithm::{
            list::{LinearListDencoder, TreelikeListDencoder},
            pair::{DiagonalPairDencoder, SquarePairDencoder},
            tree::RecursiveTreeDencoder,
        },
        ctx::{Ctx, ListDencoder, PairDencoder, TreeDencoder},
        paren::Parentheses,
    };

    macro_rules! ctx {
        ($field:ident: $val:expr) => {
            Ctx {
                $field: $val,
                ..Ctx {
                    pair: &SquarePairDencoder,
                    list: &TreelikeListDencoder,
                    tree: &RecursiveTreeDencoder,
                }
            }
        };
    }

    #[template]
    #[rstest]
    fn pair_dencoders(
        #[values(
            &DiagonalPairDencoder,
            &SquarePairDencoder,
        )]
        pair_dencoder: &dyn PairDencoder,
    ) {
    }

    #[apply(pair_dencoders)]
    fn pair_decode_encode_up_to_100(pair_dencoder: &dyn PairDencoder) {
        let ctx = ctx!(pair: pair_dencoder);
        for i in (0..=100).map(BigUint::new_const) {
            let (x, y) = ctx.pair.decode(&ctx, i.clone());
            let encoded = ctx.pair.encode(&ctx, x, y);
            assert_eq!(i, encoded)
        }
    }

    #[template]
    #[rstest]
    fn list_dencoders(
        #[values(
            &TreelikeListDencoder,
            &LinearListDencoder
        )]
        list_dencoder: &dyn ListDencoder,
    ) {
    }

    #[apply(list_dencoders)]
    fn decode_empty(list_dencoder: &dyn ListDencoder) {
        let ctx = ctx!(list: list_dencoder);
        assert_eq!(ctx.list.decode(&ctx, BigUint::ZERO).next(), None)
    }

    #[apply(list_dencoders)]
    fn encode_empty(list_dencoder: &dyn ListDencoder) {
        let ctx = ctx!(list: list_dencoder);
        assert_eq!(
            ctx.list.encode(&ctx, &mut std::iter::empty()),
            BigUint::ZERO
        )
    }

    #[apply(list_dencoders)]
    fn decode_encode(list_dencoder: &dyn ListDencoder) {
        let ctx = ctx!(list: list_dencoder);
        for number in [1, 0, 2341234, 3257893, 1234, 8] {
            let number = BigUint::new_const(number);
            let new_number = ctx
                .list
                .encode(&ctx, &mut ctx.list.decode(&ctx, number.clone()));
            assert_eq!(number, new_number);
        }
    }

    #[apply(list_dencoders)]
    fn decode_encode_sanity(list_dencoder: &dyn ListDencoder) {
        let ctx = ctx!(list: list_dencoder);
        let x = ctx
            .list
            .encode(&ctx, &mut ctx.list.decode(&ctx, BigUint::ZERO));
        let y = ctx
            .list
            .encode(&ctx, &mut ctx.list.decode(&ctx, BigUint::ONE));
        assert_ne!(x, y);
    }

    #[apply(list_dencoders)]
    fn encode_decode_long(list_dencoder: &dyn ListDencoder) {
        let ctx = ctx!(list: list_dencoder);
        let seq: Vec<BigUint> = [
            12, 234, 523, 1, 3, 0, 0, 1598, 8831, 213, 2134, 9324, 123, 656, 0, 0,
        ]
        .into_iter()
        .map(BigUint::new_const)
        .collect();
        let code = ctx.list.encode(&ctx, &mut seq.iter().cloned());
        let decoded: Vec<BigUint> = ctx.list.decode(&ctx, code).collect();
        assert_eq!(seq, decoded);
    }

    #[template]
    #[rstest]
    fn tree_dencoders(#[values(&RecursiveTreeDencoder)] tree_dencoder: &dyn TreeDencoder) {}

    #[apply(tree_dencoders)]
    fn decode_encode_parens() {
        let ctx = ctx!(tree: &RecursiveTreeDencoder);
        for i in (0..=100).map(BigUint::new_const) {
            let decoded = ctx.tree.decode(&ctx, i.clone());
            let encoded = ctx.tree.encode(&ctx, decoded.as_slice());
            assert_eq!(i, encoded);
        }
    }

    #[apply(tree_dencoders)]
    #[case("(()(()))()")]
    #[case("(())()(()()((())())())()")]
    #[case("(()(()))()((())())()")]
    fn encode_decode_parens(tree_dencoder: &dyn TreeDencoder, #[case] parens: &str) {
        let ctx = ctx!(tree: tree_dencoder);
        let parsed: Parentheses = parens.parse().unwrap();
        let code = ctx.tree.encode(&ctx, parsed.as_slice());
        let new_parens = ctx.tree.decode(&ctx, code);
        assert_eq!(parsed, new_parens);
    }
}
