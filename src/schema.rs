extern crate sxd_document;

use sxd_document::parser as DomParser;
use sxd_document::dom::Document;

use parser::{
    parse_elements,
    parse_version,
    SchemaVersion,
    Element
};

static XSD_10_SCHEMA_STR: &'static str = include_str!("schemas/1.0.xsd");
static XSD_11_SCHEMA_STR: &'static str = include_str!("schemas/1.1.xsd");

pub struct Schema<'a> {
    pub version: SchemaVersion,
    pub elements: Vec<Element<'a>>,
}

#[derive(Debug)]
pub enum SchemaError {
    UnsupportedSchemaVersion,
}

enum XSDType {
    XSDComplexType,
    XSDSimpleType(usize),
}

impl<'a> Schema<'a> {

    pub fn from_document<'b>(document: &'b Document) -> Result<Schema<'b>, SchemaError> {
        let version = parse_version(&document);

        let schema = Schema {
            version: version,
            elements: parse_elements(document.root()),
        };

        // uncomment once https://github.com/shepmaster/sxd-document/issues/50 is fixed
        // return Schema::validate_schema(schema, &document);
        return Ok(schema);
    }

    fn validate_schema<'b>(schema: Schema<'b>, document: &'b Document) -> Result<Schema<'b>, SchemaError> {
        let root = document.root();

        if schema.version == SchemaVersion::Xsd10 {
            let schema_package = DomParser::parse(&XSD_10_SCHEMA_STR)
                .expect("Failed to parse 1.0 Schema XSD");
            let schema_document = schema_package.as_document();
            let schema_schema = Schema {
                version: SchemaVersion::Xsd10,
                elements: parse_elements(root),
            };
            schema_schema.validate(&document)?;
        } else if schema.version == SchemaVersion::Xsd11 {
            let schema_package = DomParser::parse(&XSD_11_SCHEMA_STR)
                .expect("Failed to parse 1.1 Schema XSD");
            let schema_document = schema_package.as_document();
            let schema_schema = Schema {
                version: SchemaVersion::Xsd11,
                elements: parse_elements(root),
            };
            schema_schema.validate(&document)?;
        } else {
            return Err(SchemaError::UnsupportedSchemaVersion);
        }

        return Ok(schema);
    }

    pub fn validate(&self, document: &Document) -> Result<(), SchemaError> {
        Ok(())
    }
}