use crate::parser::Element;

pub fn build_mermaid(out: &mut String, e: Element) {
    match e.elements {
        Some(elements) => {
            for element in elements.iter() {
                let mid = match (element.version.clone(), element.note.clone()) {
                    (Some(v), Some(n)) => format!("|'{}' {}|", n, v),
                    (Some(v), None) => format!("|{}|", v),
                    (None, Some(n)) => format!("|'{}'|", n),
                    (None, None) => String::from(""),
                };
                let done = format!(
                    "    {}---{}{}\n",
                    e.name.clone(),
                    mid,
                    element.name.clone()
                );
                out.push_str(done.as_str());
                build_mermaid(out, element.clone())
            }
        }
        // Leaf node
        None => {}
    }
}
