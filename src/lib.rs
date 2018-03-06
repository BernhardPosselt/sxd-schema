extern crate sxd_document;

use sxd_document::parser as DomParser;
use sxd_document::dom::Document;

mod parser;

use parser::{
    parse_meta,
    parse_version,
    SchemaMeta,
    SchemaVersion,
};

static XSD_10_SCHEMA_STR: &'static str = include_str!("schemas/1.0.xsd");
static XSD_11_SCHEMA_STR: &'static str = include_str!("schemas/1.1.xsd");

pub struct Schema<'a> {
    pub version: SchemaVersion,
    pub meta: SchemaMeta<'a>,
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
        let meta = parse_meta(&document);

        let schema = Schema {
            version: version,
            meta: meta,
        };

        // validate schema using a schema
        if schema.version == SchemaVersion::Xsd10 {
            let schema_package = DomParser::parse(&XSD_10_SCHEMA_STR)
                .expect("Failed to parse 1.0 Schema XSD");
            let schema_document = schema_package.as_document();
            let schema_schema = Schema {
                version: SchemaVersion::Xsd10,
                meta: parse_meta(&schema_document),
            };
            schema_schema.validate(&document)?;
        } else if schema.version == SchemaVersion::Xsd11 {
            let schema_package = DomParser::parse(&XSD_11_SCHEMA_STR)
                .expect("Failed to parse 1.1 Schema XSD");
            let schema_document = schema_package.as_document();
            let schema_schema = Schema {
                version: SchemaVersion::Xsd11,
                meta: parse_meta(&schema_document),
            };
            schema_schema.validate(&document)?;
        } else {
            return Err(SchemaError::UnsupportedSchemaVersion);
        }

        Ok(schema)
    }

    pub fn validate(&self, document: &Document) -> Result<(), SchemaError> {
        Ok(())
    }
}

/*fn find_schema_root(root: &Root) -> Option<&Root> {



fn parse_types(root: &Root) -> Vec<XSDType> {
    root.children().iter()
        .filter_map(|&child| child.element())
        .map(|element| parse_type(&element))
        .collect()
}

fn parse_type(child: &Element) -> XSDType {
    println!("{:?}", child.name());
    XSDType::XSDSimpleType(1)
}}*/
/*
#[cfg(test)]
mod tests {
    extern crate sxd_document;

    use sxd_document::parser as DomParser;
    use super::*;

    #[test]
    fn empty() {
        let schema_xml = r#"<?xml version="1.0"?>
        <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"/>
        "#;

        let schema_doc = DomParser::parse(&schema_xml).expect("Failed to parse").as_document();
        let schema = Schema::from_document(&schema_doc);

        let xml = r#"<?xml version="1.0"?><root></root>"#;
        let doc = DomParser::parse(&xml).expect("Failed to parse");

        assert!(schema.is_ok());
        assert!(schema.unwrap().validate(&doc.as_document()).is_ok());
    }


    #[test]
    fn types() {
        let schema_xml = r#"<?xml version="1.0"?>
        <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema">

          <xsd:annotation>
            <xsd:documentation xml:lang="en">
             Purchase order schema for Example.com.
             Copyright 2000 Example.com. All rights reserved.
            </xsd:documentation>
          </xsd:annotation>

          <xsd:element name="purchaseOrder" type="PurchaseOrderType"/>

          <xsd:element name="comment" type="xsd:string"/>

          <xsd:complexType name="PurchaseOrderType">
            <xsd:sequence>
              <xsd:element name="shipTo" type="USAddress"/>
              <xsd:element name="billTo" type="USAddress"/>
              <xsd:element ref="comment" minOccurs="0"/>
              <xsd:element name="items"  type="Items"/>
            </xsd:sequence>
            <xsd:attribute name="orderDate" type="xsd:date"/>
          </xsd:complexType>

          <xsd:complexType name="USAddress">
            <xsd:sequence>
              <xsd:element name="name"   type="xsd:string"/>
              <xsd:element name="street" type="xsd:string"/>
              <xsd:element name="city"   type="xsd:string"/>
              <xsd:element name="state"  type="xsd:string"/>
              <xsd:element name="zip"    type="xsd:decimal"/>
            </xsd:sequence>
            <xsd:attribute name="country" type="xsd:NMTOKEN"
                           fixed="US"/>
          </xsd:complexType>

          <xsd:complexType name="Items">
            <xsd:sequence>
              <xsd:element name="item" minOccurs="0" maxOccurs="unbounded">
                <xsd:complexType>
                  <xsd:sequence>
                    <xsd:element name="productName" type="xsd:string"/>
                    <xsd:element name="quantity">
                      <xsd:simpleType>
                        <xsd:restriction base="xsd:positiveInteger">
                          <xsd:maxExclusive value="100"/>
                        </xsd:restriction>
                      </xsd:simpleType>
                    </xsd:element>
                    <xsd:element name="USPrice"  type="xsd:decimal"/>
                    <xsd:element ref="comment"   minOccurs="0"/>
                    <xsd:element name="shipDate" type="xsd:date" minOccurs="0"/>
                  </xsd:sequence>
                  <xsd:attribute name="partNum" type="SKU" use="required"/>
                </xsd:complexType>
              </xsd:element>
            </xsd:sequence>
          </xsd:complexType>

          <!-- Stock Keeping Unit, a code for identifying products -->
          <xsd:simpleType name="SKU">
            <xsd:restriction base="xsd:string">
              <xsd:pattern value="\d{3}-[A-Z]{2}"/>
            </xsd:restriction>
          </xsd:simpleType>

        </xsd:schema>
        "#;

        let schema_doc = DomParser::parse(&schema_xml).expect("Failed to parse").as_document();
        let schema = Schema::from_document(&schema_doc);

        assert!(schema.is_ok());
        assert_eq!(SchemaVersion::Xsd10, schema.unwrap().version)
    }

}
  */

