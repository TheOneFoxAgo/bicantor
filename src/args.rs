pub mod decode;
pub mod encode;

use argh::{FromArgValue, FromArgs};

/// The ultimate tool for encoding/decoding natural numbers
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
#[allow(unused)]
pub struct Args {
    /// pair encoder operation mode.
    /// Possible values are: "cantor", "square".
    /// Default is "square"
    #[argh(option, short = 'p', default = "PairMode::Square")]
    pub pair: PairMode,

    /// list encoder operation mode.
    /// Possible values are: "linear", "treelike".
    /// Default is "treelike"
    #[argh(option, short = 'l', default = "ListMode::Treelike")]
    pub list: ListMode,

    /// tree encoder operation mode.
    /// Possible values are: "width", "depth".
    /// Default is "depth"
    #[argh(option, short = 't', default = "TreeMode::Depth")]
    pub tree: TreeMode,

    #[argh(subcommand)]
    pub action: Action,
}

#[derive(FromArgValue)]
pub enum PairMode {
    Cantor,
    Square,
}

#[derive(FromArgValue)]
pub enum ListMode {
    Linear,
    Treelike,
}

#[derive(FromArgValue)]
pub enum TreeMode {
    Depth,
    Width,
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
