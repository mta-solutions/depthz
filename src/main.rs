use clap::Parser;
use serde_json::Result;

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
    println!("depthz: {}", cli.depthz);

    let data0 = r#"
        {
          "name": "DomainA",
          "type": "domain",
          "repos": [
            { "url": "git@github.com:mta-solutions/depthz.git",
        	  "name": "depthz",
        	  "path": "/test/repo/a"
        	},
            { "url": "git@github.com:mta-solutions/depthz.git",
        	  "name": "depthz",
        	  "path": "/test/repo/b"
        	}
          ],
          "elements": [
            { "name": "Grafana", "type": "service" },
            { "name": "Loki", "type": "service" }
          ]
        }
    "#;

    let e: Element = serde_json::from_str(data0)?;
    println!("{:?}", e);

    if let Some(repos) = e.repos {
        for repo in repos.iter() {
            download_git(repo);
        }
    }

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

    Ok(())
}
