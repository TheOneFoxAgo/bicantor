mod algorithm;
mod args;
mod paren;

use args::Args;

use crate::args::{Action, decode, encode};

fn main() {
    let args: Args = Args::from_env();
    match args.action {
        Action::Encode(encode) => match encode.data_type {
            encode::DataType::Pair(pair) => {
                println!("{}", algorithm::pair::encode(pair.first, pair.second))
            }
            encode::DataType::List(list) => {
                println!("{}", algorithm::list::encode(list.numbers.into_iter()))
            }
            encode::DataType::Tree(tree) => {
                println!("{}", algorithm::tree::encode(tree.tree.as_slice()))
            }
        },
        Action::Decode(decode) => match decode.data_type {
            decode::DataType::Pair(pair) => {
                let (x, y) = algorithm::pair::decode(pair.number);
                println!("{} {}", x, y)
            }
            decode::DataType::List(list) => {
                let mut iter = algorithm::list::decode(list.number);
                if let Some(number) = iter.next() {
                    print!("{number}");
                    for number in iter {
                        print!(" {number}");
                    }
                }
                println!()
            }
            decode::DataType::Tree(tree) => {
                println!("{}", algorithm::tree::decode(tree.number))
            }
        },
    }
}
