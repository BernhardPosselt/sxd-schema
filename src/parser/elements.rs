use sxd_document::dom::{Element as DomElement};

use parser::is_of_element;


pub struct Element<'a> {
    pub name: &'a str,
    pub element_type: &'a str,
}

pub fn parse_elements<'a>(elements: &Vec<DomElement<'a>>) -> Vec<Element<'a>> {
    elements.iter()
        .filter(|&element| is_element(&element))
        .map(|&element| parse_element(element))
        .collect()
}

#[inline]
fn is_element(element: &DomElement) -> bool {
    is_of_element(&element, "element")
}

pub fn parse_element<'a>(element: DomElement<'a>) -> Element<'a> {
    let name = element.attribute("name").expect("Element defined without name");
    let element_type = element.attribute("type").expect("Element defined without type");
    return Element {
        name: name.value(),
        element_type: element_type.value(),
    };
}
