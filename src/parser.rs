use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub name: String,
    #[serde(rename = "type")]
    pub d_type: String,
    pub repos: Option<Vec<String>>,
    pub version: Option<String>,
    pub elements: Option<Vec<Element>>,
}
