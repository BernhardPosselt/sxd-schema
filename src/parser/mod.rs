pub mod elements;
pub mod types;
pub mod versions;
pub mod annotations;
pub mod schema;

extern crate sxd_document;

use sxd_document::dom::{
    Root,
    Element as DomElement,
    Attribute as DomAttribute,
    ChildOfElement,
};

use std::iter::Filter;
use std::slice::Iter;
use std::iter::FilterMap;
use parser::types::Id;
use std::collections::HashMap;
use std::hash::Hash;
use parser::annotations::Annotation;

static XSD_NS_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

#[derive(Eq, PartialEq, Debug)]
pub struct Language<'a> {
    pub iso_code: &'a str,
}

fn is_of_element<'a>(element: &'a DomElement, element_name: &str) -> bool {
    let name = element.name();
    return name.namespace_uri() == Some(XSD_NS_URI) && name.local_part() == element_name;
}

pub fn is_schema(element: &DomElement) -> bool {
    is_of_element(&element, "schema")
}

fn extract_element<'a>(element: &ChildOfElement<'a>) -> Option<DomElement<'a>> {
    match element {
        &ChildOfElement::Element(e) => Some(e),
        _ => None
    }
}

pub fn parse_id<'a>(element: &DomElement<'a>) -> Option<Id<'a>> {
    element.attribute("id")
        .map(|attr| Id { id: &attr.value() })
}

pub fn parse_additional_attributes<'a>(element: &DomElement<'a>) -> Vec<DomAttribute<'a>> {
    element.attributes().into_iter()
        .filter(|&attr| {
            match attr.name().namespace_uri() {
                Some(namespace) => namespace != XSD_NS_URI,
                _ => false
            }
        })
        .collect()
}

pub fn parse_boolean_attribute<'a>(element: &DomElement<'a>, name: &str, default: bool) -> bool {
    element.attribute(name)
        .map(|attr| attr.value() == "true")
        .unwrap_or(default)
}

pub fn find_schema_children<'a>(root: Root<'a>) -> Vec<DomElement<'a>> {
    root.children().iter()
        .filter_map(|&child| child.element())
        .filter(|&element| is_schema(&element))
        .flat_map(|schema_element| schema_element.children().into_iter())
        .filter_map(|child| child.element())
        .collect()
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
pub fn parse_children<'a, T, F, S>(element: &DomElement<'a>, select_func: S, map_func: F) -> Vec<T>
    where F: Fn(DomElement<'a>) -> T,
          S: Fn(&DomElement<'a>) -> bool {
    element.children().iter()
        .filter_map(|&el| extract_element(&el))
        .filter(|&el| select_func(&el))
        .map(map_func)
        .collect()
}

/////////////////////////// refactored ///////////////////////////////////
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SchemaElement {
    Schema
}

pub struct SchemaRoot<'a> {
    annotations: Vec<Annotation<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SchemaError {
    UnsupportedSchemaVersion,
    NoSchemaRootFound,
}


fn find_schema_group<'a>(element: &DomElement<'a>) -> Option<SchemaElement> {
    if is_schema(&element) {
        Some(SchemaElement::Schema)
    } else {
        None
    }
}

pub fn parse_schema<'a>(root: Root<'a>) -> Result<SchemaRoot<'a>, SchemaError> {
    let schema_elem = find_root_schema(root)
        .ok_or(SchemaError::NoSchemaRootFound)?;

    println!("{:?}", schema_elem);

    Ok(SchemaRoot {
        annotations: vec![]
    })
}

pub fn find_root_schema<'a>(root: Root<'a>) -> Option<DomElement<'a>> {
    root.children().into_iter()
        .filter_map(|child| child.element())
        .filter(|child| is_schema(&child))
        .next()
}

pub fn group_root_children<'a, K, G>(element: Root<'a>, groups: G) -> HashMap<K, Vec<DomElement<'a>>>
    where G: Fn(&DomElement<'a>) -> Option<K>,
          K: Eq + Hash {
    let mut grouped_elements = HashMap::new();
    for elem in element.children().into_iter().filter_map(|child| child.element()) {
        if let Some(group) = groups(&elem) {
            let results = grouped_elements.entry(group).or_insert(Vec::new());
            results.push(elem);
        };
    }
    return grouped_elements;
}

pub fn group_children<'a, K, G>(element: DomElement<'a>, groups: G) -> HashMap<K, Vec<DomElement<'a>>>
    where G: Fn(&DomElement<'a>) -> Option<K>,
          K: Eq + Hash {
    let mut grouped_elements = HashMap::new();
    for elem in element.children().into_iter().filter_map(|child| child.element()) {
        if let Some(group) = groups(&elem) {
            let results = grouped_elements.entry(group).or_insert(Vec::new());
            results.push(elem);
        };
    }
    return grouped_elements;
}

#[cfg(test)]
mod tests {
    extern crate sxd_document;

    use sxd_document::parser as DomParser;

    use super::*;
    use parser::schema::*;
    use parser::versions::*;
    use parser::types::TopLevelType;
    use parser::types::SimpleType;
    use parser::types::SimpleTypeContent;
    use parser::types::Restriction;
    use parser::types::AnySimpleType;
    use parser::types::BuiltIn;
    use parser::types::RestrictionRule;
    use parser::types::Pattern;
    use std::collections::HashSet;

    #[test]
    fn empty() {
        let xml = include_str!("../../tests/parser/mod/empty.xsd");
        let package = DomParser::parse(&xml);
        assert_eq!(true, package.is_err());
    }

    #[test]
    fn wrong_root() {
        let xml = include_str!("../../tests/parser/mod/wrong-root.xsd");
        let package = DomParser::parse(&xml).expect("Failed to parse");
        let document = package.as_document();
        let schema = Schema::from_document(&document);

        assert_eq!(SchemaError::NoSchemaRootFound, schema.err().unwrap());
    }

    #[test]
    fn parse() {
        let xml = include_str!("../../tests/parser/mod/purchase.xsd");
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
            final_modes: HashSet::new(),
            additional_attributes: vec![],
            content: Box::new(SimpleTypeContent::Restriction(Restriction {
                additional_attributes: vec![],
                annotation: None,
                id: None,
                restriction_type: AnySimpleType::BuiltIn(BuiltIn::String),
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

