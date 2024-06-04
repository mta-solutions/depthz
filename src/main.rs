use clap::Parser;

use self::parser::*;

pub mod git;
pub mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub depthz: String,
}

fn main() {
    let cli = Cli::parse();

    let mut depthz: Vec<Element> = vec![];

    // Read and process starting from initial DEPTHZ
    parser::parse_json(cli.depthz, &mut depthz).unwrap();

    println!("{:?}", depthz);
}
