use clap::Parser;

use self::git::download_git;
use self::parser::*;

pub mod git;
pub mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to DEPTHZ file
    #[arg(short, long)]
    pub path: String,

    /// Optional git repo to download
    #[arg(short, long)]
    pub git_url: Option<String>,

    /// Name to give repo if git_url is set
    #[arg(short, long)]
    pub name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut depthz: Vec<Element> = vec![];

    // If a git_url and name are provided, download the intitial repo
    // Otherwise path will point to a local DEPTHZ to start from
    if let (Some(url), Some(name)) = (cli.git_url, cli.name) {
        download_git(&Git {
            url: url.clone(),
            name: name.clone(),
            path: Some(cli.path.clone()),
        });
        let path = format!("/tmp/{}{}/DEPTHZ", name.clone(), cli.path);
        parser::parse_json(path, &mut depthz).unwrap();
    } else {
        // Read and process starting from initial DEPTHZ
        parser::parse_json(cli.path, &mut depthz).unwrap();
    }

    println!("{:?}", depthz);
}
