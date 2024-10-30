use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

impl Git {
    pub fn download_git(&self) -> String {
        // All relevant repos get stored in the OS's temp directory
        let mut out = std::env::temp_dir();
        out.push(self.name.clone());
        let output = if Path::new(out.as_path()).exists() {
            Command::new("git")
                .current_dir(out.clone())
                .arg("pull")
                .output()
                .expect("failed to update repo")
        } else {
            Command::new("git")
                .arg("clone")
                .arg(self.url.clone())
                .arg(out.clone())
                .output()
                .expect("failed to clone repo")
        };

        println!("status - {}: {}", self.url, output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let date = Command::new("git")
            .current_dir(out)
            .args(["log", "-n 1", "--pretty=format:%cI"])
            .output()
            .expect("failed to get last date from repo");

        return String::from_utf8(date.stdout).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
    // Name of element
    pub name: String,
    // Type of element [domain|server|service|database|library|mobile|other]
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

    // Internal field for tracking the last git commit date
    // for the top level DEPTHZ project
    git_date: Option<String>,
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
            let date = repo.download_git();
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
            let mut ie = parse(out, dz).unwrap();
            ie.git_date = Some(date);
            match e.elements.as_mut() {
                Some(v) => v.push(ie),
                None => e.elements = Some(vec![ie]),
            }
        }
    }

    Ok(e)
}
