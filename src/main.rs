mod algorithm;
mod args;
mod ctx;
mod paren;

use args::Args;

use crate::{
    algorithm::{list, pair, tree},
    args::{Action, decode, encode},
    ctx::Ctx,
};

fn main() {
    let args: Args = Args::from_env();
    let ctx = Ctx {
        pair: if args.square {
            pair::square::dencoder()
        } else {
            pair::diagonal::dencoder()
        },
        list: if args.treelike {
            list::treelike::dencoder()
        } else {
            list::linear::dencoder()
        },
        tree: tree::dencoder(),
    };
    match args.action {
        Action::Encode(encode) => match encode.data_type {
            encode::DataType::Pair(pair) => {
                println!("{}", (ctx.pair.encode)(&ctx, pair.first, pair.second))
            }
            encode::DataType::List(list) => {
                println!("{}", (ctx.list.encode)(&ctx, &mut list.numbers.into_iter()))
            }
            encode::DataType::Tree(tree) => {
                println!("{}", (ctx.tree.encode)(&ctx, tree.tree.as_slice()))
            }
        },
        Action::Decode(decode) => match decode.data_type {
            decode::DataType::Pair(pair) => {
                let (x, y) = (ctx.pair.decode)(&ctx, pair.number);
                println!("{} {}", x, y)
            }
            decode::DataType::List(list) => {
                let mut iter = (ctx.list.decode)(&ctx, list.number);
                if let Some(number) = iter.next() {
                    print!("{number}");
                    for number in iter {
                        print!(" {number}");
                    }
                }
                println!()
            }
            decode::DataType::Tree(tree) => {
                println!("{}", (ctx.tree.decode)(&ctx, tree.number))
            }
        },
    }
}
