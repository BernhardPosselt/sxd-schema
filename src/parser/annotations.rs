use sxd_document::dom::{
    Element as DomElement,
    Attribute as DomAttribute,
};
use parser::types::Id;

use parser::{
    parse_child,
    is_of_element,
    parse_id,
    parse_additional_attributes,
};

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI
#[derive(Eq, PartialEq, Debug)]
pub struct AnyUri<'a> {
    pub uri: &'a str,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-appinfo
#[derive(Eq, PartialEq, Debug)]
pub struct AppInfo<'a> {
    pub source: Option<AnyUri<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub content: DomElement<'a>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-documentation
#[derive(Eq, PartialEq, Debug)]
pub struct Documentation<'a> {
    pub source: Option<AnyUri<'a>>,
    // xml:lang
    pub language: Option<Language<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub content: DomElement<'a>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation
#[derive(Eq, PartialEq, Debug)]
pub struct Annotation<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub app_info: Vec<AppInfo<'a>>,
    pub documentation: Vec<Documentation<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Language<'a> {
    pub iso_code: &'a str,
}

pub fn parse_annotation<'a>(element: &DomElement<'a>) -> Option<Annotation<'a>> {
    parse_child(&element,
                |&el| is_of_element(&el, "annotation"),
                |el| {
                    Annotation {
                        id: parse_id(&el),
                        additional_attributes: parse_additional_attributes(&el),
                        app_info: Vec::new(),
                        documentation: Vec::new(),
                    }
                })
}

pub fn parse_annotations<'a>(elements: &Vec<DomElement<'a>>) -> Vec<Annotation<'a>> {
    elements.iter()
        .filter(|&el| is_of_element(&el, "annotation"))
        .map(|el| {
            Annotation {
                id: parse_id(&el),
                additional_attributes: parse_additional_attributes(&el),
                app_info: Vec::new(),
                documentation: Vec::new(),
            }
        })
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
    use parser::types::BuiltIn;
    use parser::types::RestrictionRule;
    use parser::types::Pattern;
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn annotation() {
        let xml = include_str!("../../tests/parser/annotations/annotations.xsd");
        let package = DomParser::parse(&xml).expect("Failed to parse");
        let document = package.as_document();
        let schema = Schema::from_document(&document).expect("Failed to parse schema");

        let annotations = schema.annotations;
        assert_eq!(2, annotations.len());

        let annotation1 = annotations.get(0).unwrap();
        assert_eq!("annotation1", annotation1.id.as_ref().unwrap().id);
        assert_eq!(1, annotation1.additional_attributes.len());
        //assert_eq!(1, annotation1.app_info.len());
        //assert_eq!(1, annotation1.documentation.len());

        let annotation2 = annotations.get(1).unwrap();
        assert_eq!("annotation2", annotation2.id.as_ref().unwrap().id);
        assert_eq!(0, annotation2.additional_attributes.len());
        assert_eq!(0, annotation2.app_info.len());
        assert_eq!(0, annotation2.documentation.len());
    }
}

