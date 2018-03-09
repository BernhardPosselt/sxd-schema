use sxd_document::dom::{Element as DomElement};

use parser::is_of_element;

pub enum XSDType<'a> {
    ComplexType(ComplexType<'a>),
    SimpleType(SimpleType<'a>),
}

pub struct ComplexType<'a> {
    pub name: &'a str
}

pub struct SimpleType<'a> {
    pub name: &'a str
}

#[inline]
fn is_type(element: &DomElement) -> bool {
    is_of_element(&element, "simpleType") || is_of_element(&element, "complexType")
}

pub fn parse_types<'a>(elements: &Vec<DomElement<'a>>) -> Vec<XSDType<'a>> {
    elements.iter()
        .filter(|&element| is_type(&element))
        .map(|&element| parse_type(element))
        .collect()
}

pub fn parse_type<'a>(element: DomElement<'a>) -> XSDType<'a> {
    let type_name = element.attribute("name").expect("Element defined without name");
    if element.name().local_part() == "simpleType" {
        return XSDType::SimpleType(SimpleType {
            name: &type_name.value()
        });
    } else {
        return XSDType::ComplexType(ComplexType {
            name: &type_name.value()
        });
    }
}
