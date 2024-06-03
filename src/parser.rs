use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    // Name of element
    pub name: String,
    // Type of element [domain|server|service|database|library|other]
    #[serde(rename = "type")]
    pub d_type: String,
    // Repos to allocate
    pub repos: Option<Vec<Git>>,
    // Version to attach to resource, usually a library
    pub version: Option<String>,
    // Additional related elements to associate
    pub elements: Option<Vec<Element>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Git {
    // Git URL
    pub url: String,
    // Name to clone repo to
    pub name: String,
    // Optional path, useful for monorepos
    pub path: Option<String>,
}
