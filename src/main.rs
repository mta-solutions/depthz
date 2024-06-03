use clap::Parser;
use serde_json::Result;
use std::fs;

use self::git::*;
use self::parser::*;

pub mod git;
pub mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    depthz: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut depthz: Vec<Element> = vec![];

    // Read initial DEPTHZ
    let data0 = fs::read_to_string(cli.depthz).expect("DEPTHZ file was unreadable");
    let e: Element = serde_json::from_str(data0.as_str())?;

    // Loop over any git repos it may contain and clone/update them
    if let Some(repos) = e.repos.clone() {
        for repo in repos.iter() {
            download_git(repo);
            // Read the DEPTHZ files defined for this repo
            let out: String = if let Some(path) = repo.path.clone() {
                // Read from defined path
                format!("/tmp/{}{}", repo.name, path)
            } else {
                // Assume top of repo
                format!("/tmp/{}/DEPTHZ", repo.name)
            };
            println!("path: {}", out);
            let data = fs::read_to_string(out).expect("DEPTHZ file was unreadable");
            let e: Element = serde_json::from_str(data.as_str())?;
            depthz.push(e);
        }
    }
    depthz.push(e);

    println!("{:?}", depthz);

    Ok(())
}
