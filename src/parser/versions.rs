use sxd_document::dom::{
    Document,
};


#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SchemaVersion {
    Xsd10,
    Xsd11,
}

#[allow(unused_variables)]
pub fn parse_version(document: &Document) -> SchemaVersion {
    SchemaVersion::Xsd10
}
