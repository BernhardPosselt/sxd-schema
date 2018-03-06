use sxd_document::dom::{Document};
//use sxd_document::dom::Element;

pub struct SchemaMeta<'a> {
    elements: &'a Vec<&'a str>
}

pub fn parse_version(document: &Document) -> String {
    String::from("1.0")
}

pub fn parse_meta<'a>(document: &'a Document) -> SchemaMeta<'a> {
    SchemaMeta<'a> {
        elements: &vec!("hi")
    }
}