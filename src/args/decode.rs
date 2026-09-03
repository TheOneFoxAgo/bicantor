use argh::FromArgs;
use num_bigint::BigUint;

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
pub enum DataType {
    Pair(Pair),
    List(List),
    Tree(Tree),
}

/// Decode a pair of numbers
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "pair")]
pub struct Pair {
    /// encoded pair
    #[argh(positional)]
    pub number: BigUint,
}

/// Decode a list of numbers
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "list")]
pub struct List {
    /// encoded list
    #[argh(positional)]
    pub number: BigUint,
}

/// Decode a tree
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "tree")]
pub struct Tree {
    /// encoded tree
    #[argh(positional)]
    pub number: BigUint,
}
