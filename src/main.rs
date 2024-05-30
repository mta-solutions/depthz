use clap::Parser;
use git2::{Cred, RemoteCallbacks};
use serde_json::Result;
use std::env;
use std::path::{Path, PathBuf};

use self::parser::*;

mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Optional override for ~/.ssh/id_rsa
    #[arg(short, long)]
    ssh_id: Option<String>,
    depthz: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("depthz: {}", cli.depthz);

    let data0 = r#"
        {
          "name": "DomainA",
          "type": "domain",
          "repos": [
            "git@host:repoA.git",
            "git@host:repoB.git",
            "git@host:repoC.git"
          ],
          "elements": [
            { "name": "Grafana", "type": "service" },
            { "name": "Loki", "type": "service" }
          ]
        }
    "#;

    let e: Element = serde_json::from_str(data0)?;
    println!("{:?}", e);

    let data1 = r#"
        {
          "name": "ServerA",
          "type": "server",  
          "elements": [
            { "name": "AppA",
              "type": "service",
              "elements": [
                { "name": "PosgresA",
                  "type": "database",
                  "elements": [{ "name": "DatabaseA", "type": "other" }]
                },
                { "name": "ExternalA", "type": "service" },
                { "name": "InternalA", "type": "service" },
                { "name": "LibraryA", "type": "library", "version": "1.0" }
              ]
            }
          ]
        }
    "#;

    let e: Element = serde_json::from_str(data1)?;
    println!("{:?}", e);

    // Git

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    builder
        .clone(
            "git@github.com:rust-lang/git2-rs.git",
            Path::new("/tmp/git2-rs"),
        )
        .unwrap();

    Ok(())
}
