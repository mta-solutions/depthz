use crate::parser::{Element, Type};

pub trait Builder {
    fn build(&self, out: &mut String, e: Element, f: &Option<Vec<String>>);

    fn match_filter(&self, t: &Option<Vec<String>>, f: &Option<Vec<String>>) -> bool {
        match (t, f) {
            // nothing to filter means it's a match
            (None, None) => return true,
            // if tags is empty but filter is not, no match
            (None, Some(_)) => return false,
            // match due to lack of filter
            (Some(_), None) => return true,
            // check for matches across both lists
            (Some(tags), Some(filters)) => {
                for tag in tags.iter() {
                    for filter in filters.iter() {
                        if tag == filter {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}

pub struct Mermaid;

impl Builder for Mermaid {
    // Build a mermaid.js output string. Filter based on an optional list of tags
    fn build(&self, out: &mut String, e: Element, f: &Option<Vec<String>>) {
        // Skip this element if there isn't a filter match
        if !self.match_filter(&e.tags, &f) {
            return;
        };

        // Group systems by their domain
        let mut domain = false;
        if e.d_type == Type::Domain {
            domain = true;
            let str = format!("    subgraph {}_Domain\n", e.name.clone());
            out.push_str(str.as_str());
        }

        match e.elements {
            Some(elements) => {
                for element in elements.iter() {
                    // Group systems by their domain
                    // Versions
                    let mid = match (element.version.clone(), element.note.clone()) {
                        (Some(v), Some(n)) => format!("|'{}' {}|", n, v),
                        (Some(v), None) => format!("|{}|", v),
                        (None, Some(n)) => format!("|'{}'|", n),
                        (None, None) => String::from(""),
                    };

                    // Output
                    let left: String = e.name.split_whitespace().collect();
                    let right: String = element.name.split_whitespace().collect();
                    let res = format!("    {}---{}{}\n", left, mid, right);

                    let class_ref = format!("class {}", right);
                    match element.d_type {
                        Type::Server => {
                            let class = format!("{} {}\n", class_ref, "server");
                            out.push_str(class.as_str());
                        }
                        Type::Service => {
                            let class = format!("{} {}\n", class_ref, "service");
                            out.push_str(class.as_str());
                        }
                        Type::Database => {
                            let class = format!("{} {}\n", class_ref, "database");
                            out.push_str(class.as_str());
                        }
                        Type::Library => {
                            let class = format!("{} {}\n", class_ref, "library");
                            out.push_str(class.as_str());
                        }
                        Type::Mobile => {
                            let class = format!("{} {}\n", class_ref, "mobile");
                            out.push_str(class.as_str());
                        }
                        _ => {}
                    }

                    out.push_str(res.as_str());

                    // Continue to recurse
                    self.build(out, element.clone(), f);
                }
            }
            // Leaf node
            None => {}
        }

        if domain {
            out.push_str("    end\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_filter() {
        let tags = Some(vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
        ]);
        let filters = Some(vec![String::from("abc")]);
        let tags_ = Some(vec![String::from("xyz")]);
        let filters_ = Some(vec![String::from("mno")]);

        let tags_none = None;
        let filters_none = None;

        assert_eq!(Mermaid.match_filter(&tags_none, &filters_none), true);
        assert_eq!(Mermaid.match_filter(&tags_none, &filters), false);
        assert_eq!(Mermaid.match_filter(&tags, &filters_none), true);
        assert_eq!(Mermaid.match_filter(&tags, &filters), true);
        assert_eq!(Mermaid.match_filter(&tags, &filters_), false);
        assert_eq!(Mermaid.match_filter(&tags_, &filters), false);
    }
}
