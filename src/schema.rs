extern crate sxd_document;

use sxd_document::{parser as DomParser, Package};
use sxd_document::dom::{Document};

use parser::{find_schema_children};
use parser::elements::{parse_elements, Element};
use parser::versions::{parse_version, SchemaVersion};
use parser::types::{parse_types, TopLevelType};

static XSD_10_SCHEMA_STR: &'static str = include_str!("parser/schemas/1.0.xsd");
static XSD_11_SCHEMA_STR: &'static str = include_str!("parser/schemas/1.1.xsd");

pub struct Schema<'a> {
    pub version: SchemaVersion,
    pub elements: Vec<Element<'a>>,
    pub types: Vec<TopLevelType<'a>>,
}

#[derive(Debug)]
pub enum SchemaError {
    UnsupportedSchemaVersion,
}

/// Makes sure that a schema is correct by validating it using the official schemas
fn create_schema_spec<'a>(package: &'a Package) -> Schema<'a> {
    let document = package.as_document();
    let children = find_schema_children(document.root());
    return Schema {
        version: SchemaVersion::Xsd10,
        elements: parse_elements(&children),
        types: parse_types(&children),
    };
}

#[allow(dead_code)]
fn validate_schema<'b>(schema: Schema<'b>, schema_document: &'b Document) -> Result<Schema<'b>, SchemaError> {
    if schema.version == SchemaVersion::Xsd10 {
        let package = DomParser::parse(&XSD_10_SCHEMA_STR)
            .expect("Failed to parse Schema 1.0 XSD");
        let schema_schema = create_schema_spec(&package);
        schema_schema.validate(&schema_document)?;
    } else if schema.version == SchemaVersion::Xsd11 {
        let package = DomParser::parse(&XSD_11_SCHEMA_STR)
            .expect("Failed to parse Schema 1.1 XSD");
        let schema_schema = create_schema_spec(&package);
        schema_schema.validate(&schema_document)?;
    } else {
        return Err(SchemaError::UnsupportedSchemaVersion);
    }
    return Ok(schema);
}

impl<'a> Schema<'a> {

    pub fn from_document<'b>(document: &'b Document) -> Result<Schema<'b>, SchemaError> {
        let version = parse_version(&document);
        let children = find_schema_children(document.root());

        let schema = Schema {
            version: version,
            elements: parse_elements(&children),
            types: parse_types(&children)
        };

        // uncomment once https://github.com/shepmaster/sxd-document/issues/50 is fixed
        // return validate_schema(schema, &document);
        return Ok(schema);
    }

    #[allow(unused_variables)]
    pub fn validate(&self, document: &Document) -> Result<(), SchemaError> {
        Ok(())
    }
}