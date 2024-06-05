use crate::parser::Element;

pub fn build_mermaid(out: &mut String, e: Element) {
    match e.elements {
        Some(elements) => {
            for element in elements.iter() {
                // Display version on arrow text
                let val = match element.version.clone() {
                    Some(ver) => {
                        format!(
                            "    {} -->|{}|{}\n",
                            e.name.clone(),
                            ver,
                            element.name.clone()
                        )
                    }
                    None => format!("    {} --> {}\n", e.name.clone(), element.name.clone()),
                };
                out.push_str(val.as_str());
                build_mermaid(out, element.clone())
            }
        }
        // Leaf node
        None => {}
    }
}
