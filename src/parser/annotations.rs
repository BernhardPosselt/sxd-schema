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
pub struct Appinfo<'a> {
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
    pub appinfo: Vec<Appinfo<'a>>,
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
                        id: parse_id(&element),
                        additional_attributes: parse_additional_attributes(&element),
                        appinfo: Vec::new(),
                        documentation: Vec::new(),
                    }
                })
}