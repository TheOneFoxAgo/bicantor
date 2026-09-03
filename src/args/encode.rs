use argh::FromArgs;
use num_bigint::BigUint;

use crate::paren::Parentheses;

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
pub enum DataType {
    Pair(Pair),
    List(List),
    Tree(Tree),
}

/// Encode a pair of numbers
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "pair")]
pub struct Pair {
    /// first number in pair
    #[argh(positional)]
    pub first: BigUint,

    /// second number in pair
    #[argh(positional)]
    pub second: BigUint,
}

/// Encode a list of numbers
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "list")]
pub struct List {
    /// input list
    #[argh(positional)]
    pub numbers: Vec<BigUint>,
}

/// Encode a tree
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "tree")]
pub struct Tree {
    /// input tree in form of "correct" parentheses sequence like: ()((()()))
    #[argh(positional)]
    pub tree: Parentheses,
}
