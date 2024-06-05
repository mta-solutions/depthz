use crate::parser::Element;

pub fn build_mermaid(out: &mut String, e: Element) {
    match e.elements {
        Some(elements) => {
            for element in elements.iter() {
                let val = format!("    {} --> {}\n", e.name.clone(), element.name.clone());
                out.push_str(val.as_str());
                build_mermaid(out, element.clone())
            }
        }
        // Leaf node
        None => {}
    }
}
