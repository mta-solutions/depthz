use crate::parser::Element;

fn match_filter(t: &Option<Vec<String>>, f: &Option<Vec<String>>) -> bool {
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

// Build a mermaid.js output string. Filter based on an optional list of tags
pub fn build_mermaid(out: &mut String, e: Element, f: &Option<Vec<String>>) {
    // Skip this element if there isn't a filter match
    if !match_filter(&e.tags, &f) {
        return;
    };
    match e.elements {
        Some(elements) => {
            for element in elements.iter() {
                let mid = match (element.version.clone(), element.note.clone()) {
                    (Some(v), Some(n)) => format!("|'{}' {}|", n, v),
                    (Some(v), None) => format!("|{}|", v),
                    (None, Some(n)) => format!("|'{}'|", n),
                    (None, None) => String::from(""),
                };
                let left: String = e.name.split_whitespace().collect();
                let right: String = element.name.split_whitespace().collect();
                let done = format!("    {}---{}{}\n", left, mid, right);
                out.push_str(done.as_str());
                build_mermaid(out, element.clone(), f)
            }
        }
        // Leaf node
        None => {}
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

        assert_eq!(match_filter(&tags_none, &filters_none), true);
        assert_eq!(match_filter(&tags_none, &filters), false);
        assert_eq!(match_filter(&tags, &filters_none), true);
        assert_eq!(match_filter(&tags, &filters), true);
        assert_eq!(match_filter(&tags, &filters_), false);
        assert_eq!(match_filter(&tags_, &filters), false);
    }
}
