extern crate sxd_document;

use sxd_document::{parser as DomParser, Package};
use sxd_document::dom::{Document, Attribute as DomAttribute};

use parser::{find_schema_children, parse_schema, SchemaError};
use parser::elements::{parse_elements, Element};
use parser::versions::{parse_version, SchemaVersion};
use parser::types::{parse_types, TopLevelType};
use parser::annotations::{Annotation, parse_annotations};
use parser::types::Id;
use parser::Language;

static XSD_10_SCHEMA_STR: &'static str = include_str!("schemas/1.0.xsd");
static XSD_11_SCHEMA_STR: &'static str = include_str!("schemas/1.1.xsd");

pub enum FormChoice {
    Qualified,
    Unqualified,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum BlockDefault {
    Extension,
    Restriction,
    Substitution,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum FinalDefault {
    Extension,
    Restriction,
    List,
    Union,
}

/// see https://www.w3.org/TR/xmlschema-1/#declare-schema
pub struct Schema<'a> {
    pub version: SchemaVersion,
    pub target_namespace: Option<&'a str>,
    pub final_default: Vec<FinalDefault>,
    pub block_default: Vec<BlockDefault>,
    // default unqualified
    pub attribute_form_default: FormChoice,
    // default unqualified
    pub element_form_default: FormChoice,
    pub id: Option<Id<'a>>,
    pub elements: Vec<Element<'a>>,
    pub types: Vec<TopLevelType<'a>>,
    pub annotations: Vec<Annotation<'a>>,
    // TODO
    pub includes: Vec<&'a str>,
    pub imports: Vec<&'a str>,
    pub redefines: Vec<&'a str>,
    pub groups: Vec<&'a str>,
    pub attribute_groups: Vec<&'a str>,
    pub notations: Vec<&'a str>,
    // xml:lang
    pub language: Option<Language<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
}


/// Makes sure that a schema is correct by validating it using the official schemas
fn create_schema_spec<'a>(package: &'a Package) -> Schema<'a> {
    let document = package.as_document();
    let children = find_schema_children(document.root());
    return Schema {
        version: SchemaVersion::Xsd10,
        target_namespace: None,
        final_default: Vec::new(),
        block_default: Vec::new(),
        attribute_form_default: FormChoice::Unqualified,
        element_form_default: FormChoice::Unqualified,
        id: None,
        elements: parse_elements(&children),
        types: parse_types(&children),
        annotations: parse_annotations(&children),
        includes: Vec::new(),
        imports: Vec::new(),
        redefines: Vec::new(),
        groups: Vec::new(),
        attribute_groups: Vec::new(),
        notations: Vec::new(),
        language: None,
        additional_attributes: Vec::new(),
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
        let schema_meta = parse_schema(document.root())?;

        let schema = Schema {
            version: version,
            target_namespace: None,
            final_default: Vec::new(),
            block_default: Vec::new(),
            attribute_form_default: FormChoice::Unqualified,
            element_form_default: FormChoice::Unqualified,
            id: None,
            elements: parse_elements(&children),
            types: parse_types(&children),
            annotations: parse_annotations(&children),
            includes: Vec::new(),
            imports: Vec::new(),
            redefines: Vec::new(),
            groups: Vec::new(),
            attribute_groups: Vec::new(),
            notations: Vec::new(),
            language: None,
            additional_attributes: Vec::new(),
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