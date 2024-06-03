use serde::{Deserialize, Serialize};
use serde_json::Result;

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

impl Element {
    // Parse a full JSON structure into an Element
    pub fn parse(&self) -> Result<()> {
        Ok(())
    }
    pub fn process(&self) {}
}
