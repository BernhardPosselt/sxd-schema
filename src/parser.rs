use sxd_document::dom::{Document, Root};

static XSD_NS_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

#[derive(PartialEq, Debug)]
pub enum SchemaVersion {
    Xsd10,
    Xsd11,
}

pub struct SchemaMeta<'a> {
    root: Root<'a>,
}

pub fn parse_version(document: &Document) -> SchemaVersion {
    SchemaVersion::Xsd10
}

pub fn parse_meta<'a>(document: &'a Document) -> SchemaMeta<'a> {
    SchemaMeta {
        root: document.root()
    }
}