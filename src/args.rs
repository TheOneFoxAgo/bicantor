pub mod decode;
pub mod encode;

use argh::FromArgs;

/// The ultimate tool for encoding/decoding natural numbers
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct Args {
    /// enable diagonal pair encoding (on by default)
    #[argh(switch, short = 'd')]
    #[allow(unused)]
    pub diagonal: bool,
    /// enable square pair encoding (off by default)
    #[argh(switch, short = 's')]
    pub square: bool,
    /// enable linear list encoding (on by default)
    #[argh(switch, short = 'l')]
    #[allow(unused)]
    pub linear: bool,
    /// enable treelike list encoding (off by default)
    #[argh(switch, short = 't')]
    pub treelike: bool,
    #[argh(subcommand)]
    pub action: Action,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
pub enum Action {
    Encode(Encode),
    Decode(Decode),
}

/// Encode into integer
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "encode")]
pub struct Encode {
    #[argh(subcommand)]
    pub data_type: encode::DataType,
}

/// Decode from integer
#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "decode")]
pub struct Decode {
    #[argh(subcommand)]
    pub data_type: decode::DataType,
}

impl Args {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}
