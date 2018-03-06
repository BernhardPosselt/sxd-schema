extern crate sxd_document;

use sxd_document::dom::{Document, Root};
use sxd_document::dom::Element;

static XSD_NS_URI: &'static str = "http://www.w3.org/2001/XMLSchema";
static XSD_10_SCHEMA_STR: &'static str = include!("schemas/1.0.xsd");
static XSD_11_SCHEMA_STR: &'static str = include!("schemas/1.1.xsd");
static XSD_10_SCHEMA_DOCUMENT: &'static Document = parser::parse(&XSD_10_SCHEMA_STR).expect("Failed to parse 1.0 Schema XSD");
static XSD_11_SCHEMA_DOCUMENT: &'static Document = parser::parse(&XSD_11_SCHEMA_STR).expect("Failed to parse 1.1 Schema XSD");
static XSD_10_SCHEMA: &'static Schema = Schema {
    document: XSD_10_SCHEMA_DOCUMENT,
    version: "1.1",
    meta: parse_meta(&XSD_10_SCHEMA_DOCUMENT),
};


static XSD_11_SCHEMA: &'static Schema = Schema {
    document: XSD_11_SCHEMA_DOCUMENT,
    version: "1.1",
    meta: parse_meta(&XSD_11_SCHEMA_DOCUMENT),
};

pub struct SchemaMeta<'a> {
}

pub struct Schema<'a> {
    document: Document<'a>,
    pub version: &'a str,
    pub meta: SchemaMeta<'a>
}

#[derive(Debug)]
pub enum SchemaError {
    UnsupportedSchemaVersion(&'static str),
}

enum XSDType {
    XSDComplexType,
    XSDSimpleType(usize)
}

impl <'a> Schema<'a> {
    pub fn from_document(document: Document) -> Result<Schema, SchemaError> {
        let schema = Schema {
            document: document,
            version: find_version(&document),
            meta: parse_meta(&document),
        };

        // validate schema using a schema
        if (schema.version == "1.0") {
            XSD_10_SCHEMA.validate(schema)?;
        } else if(schema.version == "1.1") {
            XSD_11_SCHEMA.validate(schema)?;
        } else {
            return Err(UnsupportedSchemaVersion(schema.version))
        }

        Ok(schema)
    }

    pub fn validate(&self, document: &Document) -> Result<(), SchemaError> {
        Ok(())
    }

}

fn find_schema_root(root: &Root) -> Option<&Root> {

}

fn parse_types(root: &Root) -> Vec<XSDType> {
    root.children().iter()
        .filter_map(|&child| child.element())
        .map(|element| parse_type(&element))
        .collect()
}

fn parse_type(child: &Element) -> XSDType {
    println!("{:?}", child.name());
    XSDType::XSDSimpleType(1)
}

#[cfg(test)]
mod tests {
    extern crate sxd_document;

    use sxd_document::parser;
    use super::*;

    #[test]
    fn empty() {
        let schema_xml =  r#"<?xml version="1.0"?>
        <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"/>
        "#;

        let schema_doc = parser::parse(&schema_xml).expect("Failed to parse");
        let schema = Schema::from_document(schema_doc.as_document());

        let xml = r#"<?xml version="1.0"?><root></root>"#;
        let doc = parser::parse(&xml).expect("Failed to parse");

        assert!(schema.is_ok());
        assert!(schema.unwrap().validate(&doc.as_document()).is_ok());
    }

    #[test]
    fn types() {
        let schema_xml =  r#"<?xml version="1.0"?>
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

        let schema_doc = parser::parse(&schema_xml).expect("Failed to parse");
        let schema = Schema::from_document(schema_doc.as_document());

        assert!(schema.is_ok());
        assert_eq!(4, schema.unwrap().types.len());
    }
}