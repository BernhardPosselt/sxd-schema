use sxd_document::dom::Element as DomElement;

use parser::is_of_element;

/// https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI
pub struct AnyUri {

}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-appinfo
pub struct Appinfo {
    source: Option<AnyUri>,
    // {any attributes with non-schema namespace . . .}>
    //  Content: ({any})*
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-documentation
pub struct Documentation {
    source: Option<AnyUri>,
    // xml:lang = language
    // {any attributes with non-schema namespace . . .}>
    // Content: ({any})*
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation
pub struct Annotation {
    // id = ID
    // {any attributes with non-schema namespace . . .}>
    // Content: (appinfo | documentation)*
    appinfo: Vec<Appinfo>,
    documentation: Vec<Documentation>,
}

pub struct Restriction {}

pub struct Union {}

pub struct List {}

pub enum XSDType<'a> {
    ComplexType(ComplexType<'a>),
    SimpleType(SimpleType<'a>),
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-complexType
pub struct ComplexType<'a> {
    pub name: &'a str,
    pub is_abstract: boolean,
    // defaults to false
    pub is_mixed: boolean,
    // defaults to false
    pub annotation: Option<Annotation>,
    // block = (#all | List of (extension | restriction))
    // final = (#all | List of (extension | restriction))
    //  {any attributes with non-schema namespace . . .}>
    // Content: (annotation?, (simpleContent | complexContent | ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?))))
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-simpleType
pub struct SimpleType<'a> {
    pub name: &'a str,
    pub content: SimpleTypeContent,
    pub annotation: Option<Annotation>,
    // final = (#all | List of (list | union | restriction))
    // id
    // {any attributes with non-schema namespace . . .}
}

pub enum SimpleTypeContent {
    Restriction(Restriction),
    List(List),
    Union(Union),
}

#[inline]
fn is_type(element: &DomElement) -> bool {
    is_of_element(&element, "simpleType") || is_of_element(&element, "complexType")
}

pub fn parse_types<'a>(elements: &Vec<DomElement<'a>>) -> Vec<XSDType<'a>> {
    elements.iter()
        .filter(|&element| is_type(&element))
        .map(|&element| parse_type(element))
        .collect()
}

pub fn parse_type<'a>(element: DomElement<'a>) -> XSDType<'a> {
    let type_name = element.attribute("name").expect("Element defined without name");
    if element.name().local_part() == "simpleType" {
        return XSDType::SimpleType(SimpleType {
            name: &type_name.value()
        });
    } else {
        return XSDType::ComplexType(ComplexType {
            name: &type_name.value()
        });
    }
}
