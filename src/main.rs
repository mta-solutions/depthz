use clap::Parser;

use crate::builder::build_mermaid;

use self::parser::*;

pub mod builder;
pub mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to DEPTHZ file
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// DEPTHZ file name
    #[arg(short, long, default_value = "DEPTHZ")]
    pub depthz: String,

    /// Optional git repo to download
    #[arg(short, long)]
    pub git_url: Option<String>,

    /// Name to give repo if git_url is set
    #[arg(short, long)]
    pub name: Option<String>,

    /// Optional file to write output to instead of stdout
    #[arg(short, long)]
    pub file: Option<String>,

    /// Optional comma delimited list of tags to filter by
    #[arg(short, long)]
    pub tags: Option<String>,

    /// Optional green health cutoff in months
    #[arg(long, default_value = "3")]
    pub health_green: Option<i8>,

    /// Optional red health cutoff in months
    #[arg(long, default_value = "9")]
    pub health_red: Option<i8>,
}

fn parse_tags(tags: Option<String>) -> Option<Vec<String>> {
    match tags {
        Some(t) => Some(t.split(",").map(|s| String::from(s.trim())).collect()),
        None => None,
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // If a git_url and name are provided, download the intitial repo
    // Otherwise path will point to a local DEPTHZ to start from
    let root = if let (Some(url), Some(name)) = (cli.git_url, cli.name) {
        let _ = Git {
            url: url.clone(),
            name: name.clone(),
            path: Some(cli.path.clone()),
            depthz: Some(cli.depthz.clone()),
        }
        .download_git();
        let tmp = std::env::temp_dir();
        format!("{}/{}/{}", tmp.display(), name.clone(), cli.path)
    } else {
        // Read and process starting from initial DEPTHZ
        cli.path
    };

    let element = parser::parse(root, cli.depthz).unwrap();

    let mut out = String::from("flowchart TB\n");
    let filter = parse_tags(cli.tags);
    build_mermaid(&mut out, element, &filter);

    match cli.file {
        Some(f) => {
            std::fs::write(f, out)?;
            Ok(())
        }
        None => {
            println!("{}", out);
            Ok(())
        }
    }
}
