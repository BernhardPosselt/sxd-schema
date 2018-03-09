pub mod elements;
pub mod types;
pub mod versions;

extern crate sxd_document;

use sxd_document::dom::{
    Root,
    Element as DomElement,
    ChildOfElement,
};
use std::iter::Filter;
use std::slice::Iter;
use std::iter::FilterMap;

static XSD_NS_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

#[inline]
fn is_of_element<'a>(element: &'a DomElement, element_name: &str) -> bool {
    let name = element.name();
    return name.namespace_uri() == Some(XSD_NS_URI) && name.local_part() == element_name;
}

#[inline]
fn is_schema(element: &DomElement) -> bool {
    is_of_element(&element, "schema")
}

pub fn find_schema_children<'a>(root: Root<'a>) -> Vec<DomElement<'a>> {
    root.children().iter()
        .filter_map(|&child| child.element())
        .filter(|&element| is_schema(&element))
        .flat_map(|schema_element| schema_element.children().into_iter())
        .filter_map(|child| child.element())
        .collect()
}

#[inline]
fn extract_element<'a>(element: &ChildOfElement<'a>) -> Option<DomElement<'a>> {
    match element {
        &ChildOfElement::Element(e) => Some(e),
        _ => None
    }
}

/// Selects a matching element from the given element's children and runs the map function on it
pub fn parse_child<'a, T, F, S>(element: &DomElement<'a>, select_func: S, map_func: F) -> Option<T>
    where F: Fn(DomElement<'a>) -> T,
          S: Fn(&DomElement<'a>) -> bool {
    element.children().iter()
        .filter_map(|&el| extract_element(&el))
        .filter(|&el| select_func(&el))
        .map(map_func)
        .next()
}

/// Selects all matching elements from the given element's children and runs the map function on it
#[allow(dead_code)]
pub fn parse_children<'a, T, F, S>(element: &DomElement<'a>, select_func: S, map_func: F) -> Vec<T>
    where F: Fn(DomElement<'a>) -> T,
          S: Fn(&DomElement<'a>) -> bool {
    element.children().iter()
        .filter_map(|&el| extract_element(&el))
        .filter(|&el| select_func(&el))
        .map(map_func)
        .collect()
}

#[cfg(test)]
mod tests {
    extern crate sxd_document;

    use sxd_document::parser as DomParser;

    use schema::*;
    use parser::versions::*;
    use parser::types::TopLevelType;
    use parser::types::SimpleType;
    use parser::types::SimpleTypeContent;
    use parser::types::Restriction;
    use parser::types::AnySimpleType;
    use parser::types::Primitive;
    use parser::types::RestrictionRule;
    use parser::types::Pattern;

    #[test]
    fn parse() {
        let xml = include_str!("../../test/data/purchase/purchase.xsd");
        let package = DomParser::parse(&xml).expect("Failed to parse");
        let document = package.as_document();
        let schema = Schema::from_document(&document).expect("Failed to parse schema");

        assert_eq!(SchemaVersion::Xsd10, schema.version);
        assert_eq!(2, schema.elements.len());

        let order = schema.elements.get(0).unwrap();
        assert_eq!("PurchaseOrderType", order.element_type);
        assert_eq!("purchaseOrder", order.name);

        let order = schema.elements.get(1).unwrap();
        assert_eq!("xsd:string", order.element_type);
        assert_eq!("comment", order.name);

        let types = schema.types;
        assert_eq!(4, types.len());

        let sku = types.get(3).unwrap();

        /*
        <xsd:simpleType name="SKU">
            <xsd:restriction base="xsd:string">
                <xsd:pattern value="\d{3}-[A-Z]{2}"/>
            </xsd:restriction>
        </xsd:simpleType>
        */
        let expected = TopLevelType::SimpleType(SimpleType {
            name: "SKU",
            annotation: None,
            additional_attributes: vec![],
            content: Box::new(SimpleTypeContent::Restriction(Restriction {
                additional_attributes: vec![],
                annotation: None,
                id: None,
                restriction_type: AnySimpleType::Primitive(Primitive::String),
                rules: vec![
                    RestrictionRule::Pattern(Pattern {
                        id: None,
                        additional_attributes: Vec::new(),
                        value: "\\d{3}-[A-Z]{2}",
                        annotation: None,
                    })
                ],
            })),
            id: None,
        });
        assert_eq!(expected, *sku);
    }
}

