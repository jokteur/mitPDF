use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}
fn main() {
    let args = Args::parse_args();
    let mut doc = Document::load(args.name).unwrap();
}
