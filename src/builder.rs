use crate::parser::{Element, Type};

pub trait Builder {
    fn build(
        &self,
        name_assoc: &mut Vec<String>,
        class_list: &mut Vec<String>,
        node_rels: &mut Vec<String>,
        e: Element,
        f: &Option<Vec<String>>,
    );

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
    fn build(
        &self,
        name_assoc: &mut Vec<String>,
        class_list: &mut Vec<String>,
        node_rels: &mut Vec<String>,
        e: Element,
        f: &Option<Vec<String>>,
    ) {
        // Skip this element if there isn't a filter match
        if !self.match_filter(&e.tags, &f) {
            return;
        };

        // Group systems by their domain
        let mut domain = false;
        if e.d_type == Type::Domain {
            domain = true;
            let str = format!("    subgraph {}_Domain\n", e.name.clone());
            node_rels.push(str);
        }

        // Parent element string type
        let p_e_type = serde_json::to_string(&e.d_type)
            .unwrap()
            .trim_end_matches('"')
            .trim_start_matches('"')
            .to_string();

        match e.elements {
            Some(elements) => {
                for element in elements.iter() {
                    // Versions and Notes
                    let mid = match (element.version.clone(), element.note.clone()) {
                        (Some(v), Some(n)) => format!("|'{}' {}|", n, v),
                        (Some(v), None) => format!("|{}|", v),
                        (None, Some(n)) => format!("|'{}'|", n),
                        (None, None) => String::from(""),
                    };

                    // Element string type
                    let e_type = serde_json::to_string(&element.d_type)
                        .unwrap()
                        .trim_end_matches('"')
                        .trim_start_matches('"')
                        .to_string();

                    // Output
                    let l_name: String = e.name.split_whitespace().collect();
                    let left: String = format!("{}_{}", l_name, p_e_type);
                    let r_name: String = element.name.split_whitespace().collect();
                    let right: String = format!("{}_{}", r_name, e_type);
                    let res = format!("    {}---{}{}\n", left, mid, right);
                    node_rels.push(res);

                    let name_data = format!("    {}[{}]\n", right, element.name.clone());
                    name_assoc.push(name_data);

                    let class_data = format!("    class {} {}\n", right, e_type);
                    class_list.push(class_data);

                    // Continue to recurse
                    self.build(name_assoc, class_list, node_rels, element.clone(), f);
                }
            }
            // Leaf node
            None => {}
        }

        if domain {
            node_rels.push("    end\n".to_string());
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
