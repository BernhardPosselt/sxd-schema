# SXD Schema

[![Build Status](https://travis-ci.org/BernhardPosselt/sxd-schema.svg?branch=master)](https://travis-ci.org/BernhardPosselt/sxd-schema)

Work in progress

## Usage

```rust
use sxd_schema::Schema;
use sxd_document::dom::Document;
use sxd_document::parser;

let schema_xml = r#"<?xml version="1.0"?>
<?xml version="1.0"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema">
    <xsd:element name="data" type="Data"/>
    <xsd:complexType name="Data">
        <xsd:sequence>
            <xsd:element name="datum" type="Datum" minOccurs="0" maxOccurs="unbounded"/>
        </xsd:sequence>
        <xsd:attribute name="awesome" type="xsd:boolean"/>
    </xsd:complexType>
    <xsd:simpleType name="Datum">
        <xsd:restriction base="xsd:string"/>
    </xsd:simpleType>
</xsd:schema>"#;
let schema_document = parser::parse(schema_xml).expect("Failed to parse");
let schema = Schema::from_document(&schema_document).expect("Schema is invalid");

let xml = r#"<?xml version="1.0"?>
<!-- Awesome data incoming -->
<data awesome="true">
  <datum>Science</datum>
  <datum><![CDATA[Literature]]></datum>
  <datum>Math &gt; others</datum>
</data>"#;
let document = parser::parse(xml).expect("Failed to parse");

schema.validate(&document).expect("Document did not validate");
```