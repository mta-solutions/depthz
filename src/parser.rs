use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs;

use crate::git::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Domain,
    Server,
    Service,
    Database,
    Library,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Git {
    // Git URL
    pub url: String,
    // Name to clone repo to
    pub name: String,
    // Optional path, useful for monorepos
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
    // Name of element
    pub name: String,
    // Type of element [domain|server|service|database|library|other]
    #[serde(rename = "type")]
    pub d_type: Type,
    // Repos to allocate
    pub repos: Option<Vec<Git>>,
    // Version to attach to resource, usually a library
    pub version: Option<String>,
    // Additional related elements to associate
    pub elements: Option<Vec<Element>>,
}

pub fn parse_json(dfile: String) -> Result<Element, Error> {
    let data0 = fs::read_to_string(dfile).expect("DEPTHZ file was unreadable");
    let mut e: Element = serde_json::from_str(data0.as_str())?;

    // Loop over any git repos it may contain and clone/update them
    if let Some(repos) = e.repos.clone() {
        for repo in repos.iter() {
            download_git(repo);
            // Read the DEPTHZ files defined for this repo
            let out: String = if let Some(path) = repo.path.clone() {
                // Read from defined path
                format!("/tmp/{}{}/DEPTHZ", repo.name, path)
            } else {
                // Assume top of repo
                format!("/tmp/{}/DEPTHZ", repo.name)
            };
            // Recurse through the rest of the structure
            let ie = parse_json(out).unwrap();
            match e.elements.as_mut() {
                Some(v) => v.push(ie),
                None => e.elements = Some(vec![ie]),
            }
        }
    }

    Ok(e)
}
