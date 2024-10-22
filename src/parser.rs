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
    Mobile,
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
    // Optional DEPTHZ file name
    pub depthz: Option<String>,
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
    // Optional note to attach, for example: "logging"
    pub note: Option<String>,
    // Additional related elements to associate
    pub elements: Option<Vec<Element>>,
    // Optional arbitrary tags
    pub tags: Option<Vec<String>>,
}

pub fn parse(path: String, depthz: String) -> Result<Element, Error> {
    let f = format!("{}/{}", path, depthz);
    let data0 = fs::read_to_string(f).expect("DEPTHZ file was unreadable");
    let mut e: Element = {
        let j = serde_json::from_str::<Element>(data0.as_str());
        let y = serde_yaml::from_str::<Element>(data0.as_str());
        let t = toml::from_str::<Element>(data0.as_str());
        if j.is_ok() {
            j.unwrap()
        } else if y.is_ok() {
            y.unwrap()
        } else if t.is_ok() {
            t.unwrap()
        } else {
            panic!("invalid DEPTHZ format. must be JSON, YAML, or TOML")
        }
    };

    let tmp = std::env::temp_dir();

    // Loop over any git repos it may contain and clone/update them
    if let Some(repos) = e.repos.clone() {
        for repo in repos.iter() {
            download_git(repo);
            // Read the DEPTHZ files defined for this repo
            let out: String = if let Some(rpath) = repo.path.clone() {
                // Read from defined path
                format!("{}/{}/{}", tmp.display(), repo.name, rpath)
            } else {
                // Assume top of repo
                format!("{}/{}", tmp.display(), repo.name)
            };
            let dz = repo.depthz.clone().unwrap_or(depthz.clone());
            // Recurse through the rest of the structure
            let ie = parse(out, dz).unwrap();
            match e.elements.as_mut() {
                Some(v) => v.push(ie),
                None => e.elements = Some(vec![ie]),
            }
        }
    }

    Ok(e)
}
