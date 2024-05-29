use self::parser::*;
use serde_json::Result;

mod parser;

fn main() -> Result<()> {
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

    Ok(())
}
