pub mod elements;
pub mod types;
pub mod versions;

extern crate sxd_document;

use sxd_document::dom::{
    Root,
    Element as DomElement
};

static XSD_NS_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

#[inline]
fn is_of_element<'a>(element: &'a DomElement, element_name: &'a str) -> bool {
    let name = element.name();
    return name.namespace_uri() == Some(XSD_NS_URI) && name.local_part() == element_name;
}

#[inline]
fn is_schema(element: &DomElement) -> bool {
    is_of_element(&element, "schema")
}

pub fn find_schema_children<'a>(root: Root<'a>) -> Vec<DomElement<'a>> {
    root.children().iter()
        .filter_map(|&child| child.element())
        .filter(|&element| is_schema(&element))
        .flat_map(|schema_element| schema_element.children().into_iter())
        .filter_map(|child| child.element())
        .collect()
}

#[cfg(test)]
mod tests {
    extern crate sxd_document;
    use sxd_document::parser as DomParser;

    use super::*;
    use schema::*;
    use parser::versions::*;

    #[test]
    fn elements() {
        let xml = include_str!("../../test/data/purchase/purchase.xsd");
        let package = DomParser::parse(&xml).expect("Failed to parse");
        let document = package.as_document();
        let schema = Schema::from_document(&document).expect("Failed to parse schema");

        assert_eq!(SchemaVersion::Xsd10, schema.version);
        assert_eq!(2, schema.elements.len());

        let order = schema.elements.get(0).unwrap();
        assert_eq!("PurchaseOrderType", order.element_type);
        assert_eq!("purchaseOrder", order.name);

        let order = schema.elements.get(1).unwrap();
        assert_eq!("xsd:string", order.element_type);
        assert_eq!("comment", order.name);

        let types = schema.types;
        assert_eq!(4, types.len());
    }

}

