use sxd_document::dom::{
    Element as DomElement,
    Attribute as DomAttribute,
};
use sxd_document::QName;

use parser::is_of_element;

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#built-in-datatypes
pub enum Primitive<'a> {
    String,
    Boolean,
    Decimal,
    Float,
    Double,
    Duration,
    DateTime,
    Time,
    Date,
    GYearMonth,
    GYear,
    GDay,
    GMonth,
    HexBinary,
    Base64Binary,
    AnyUri,
    QName(QName<'a>),
    Notation,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI
pub struct AnyUri<'a> {
    pub uri: &'a str,
}

pub struct Language<'a> {
    pub iso_code: &'a str,
}


/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-appinfo
pub struct Appinfo<'a> {
    pub source: Option<AnyUri<'a>>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub content: &'a str,  // FIXME: this is actually an inlined, mixed complex type
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-documentation
pub struct Documentation<'a> {
    pub source: Option<AnyUri<'a>>,
    // xml:lang
    pub language: Option<Language<'a>>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub content: &'a str,  // FIXME: this is actually an inlined, mixed complex type
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation
pub struct Annotation<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub appinfo: Vec<Appinfo<'a>>,
    pub documentation: Vec<Documentation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-restriction
pub struct Restriction<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    // take from either base or nested simpleType
    pub annotation: Option<Annotation<'a>>,
    pub restriction_type: AnySimpleType<'a>,
    pub rules: Vec<RestrictionRule<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minInclusive
struct MinExclusive<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minExclusive
struct MinInclusive<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxInclusive
struct MaxExclusive<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxExclusive
struct MaxInclusive<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-whiteSpace
struct WhiteSpace<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: WhiteSpaceValue,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

pub enum WhiteSpaceValue {
    Collapse,
    Preserve,
    Replace,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-totalDigits
struct TotalDigits<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-fractionDigits
struct FractionDigits<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-pattern
struct Pattern<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: &'a str,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-enumeration
struct Enumeration<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: &'a str,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-length
struct Length<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-minLength
struct MinLength<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-maxLength
struct MaxLength<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

pub enum RestrictionRule<'a> {
    MinExclusive(MinExclusive<'a>),
    MinInclusive(MinInclusive<'a>),
    MaxExclusive(MaxExclusive<'a>),
    MaxInclusive(MaxInclusive<'a>),
    TotalDigits(TotalDigits<'a>),
    FractionDigits(FractionDigits<'a>),
    Length(Length<'a>),
    MinLength(MinLength<'a>),
    MaxLength(MaxLength<'a>),
    Enumeration(Enumeration<'a>),
    WhiteSpace(WhiteSpace<'a>),
    Pattern(Pattern<'a>),
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-union
pub struct Union<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    pub annotation: Option<Annotation<'a>>,
    pub member_types: Vec<SimpleType<'a>>,  // choose from memberTypes(QName) or nested simpleType
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-list
pub struct List<'a> {
    pub id: Option<&'a str>,
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    // choose from itemType(QName) or nested simpleType
    pub annotation: Option<Annotation<'a>>,
    pub item_type: SimpleType<'a>,
}

pub enum AnyType<'a> {
    ComplexType(ComplexType<'a>),
    AnySimpleType(AnySimpleType<'a>),
}

pub enum AnySimpleType<'a> {
    Primitive(Primitive<'a>),
    SimpleType(SimpleType<'a>),
}

pub enum TopLevelType<'a> {
    SimpleType(SimpleType<'a>),
    ComplexType(ComplexType<'a>),
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-complexType
pub struct ComplexType<'a> {
    pub name: &'a str,
    // defaults to false
    pub is_mixed: bool,
    pub annotation: Option<Annotation<'a>>,
    // defaults to false
    pub is_abstract: bool,
    // block = (#all | List of (extension | restriction))
    // final = (#all | List of (extension | restriction))
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
    // Content: (simpleContent | complexContent | ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?)))
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-simpleType
pub struct SimpleType<'a> {
    pub id: Option<&'a str>,
    pub name: &'a str,
    pub content: Box<SimpleTypeContent<'a>>,
    pub annotation: Option<Annotation<'a>>,
    // final = (#all | List of (list | union | restriction))
    pub additional_attributes: Vec<&'a DomAttribute<'a>>,
}

pub enum SimpleTypeContent<'a> {
    Restriction(Restriction<'a>),
    List(List<'a>),
    Union(Union<'a>),
}

#[inline]
fn is_type(element: &DomElement) -> bool {
    is_of_element(&element, "simpleType") || is_of_element(&element, "complexType")
}

pub fn parse_types<'a>(elements: &Vec<DomElement<'a>>) -> Vec<TopLevelType<'a>> {
    elements.iter()
        .filter(|&element| is_type(&element))
        .map(|&element| parse_type(element))
        .collect()
}

pub fn parse_type<'a>(element: DomElement<'a>) -> TopLevelType<'a> {
    let type_name = element.attribute("name").expect("Element defined without name");
    if element.name().local_part() == "simpleType" {
        return TopLevelType::SimpleType(SimpleType {
            name: &type_name.value(),
            annotation: None,
            additional_attributes: vec![],
            content: Box::new(SimpleTypeContent::Restriction(Restriction {
                additional_attributes: vec![],
                annotation: None,
                id: None,
                restriction_type: AnySimpleType::Primitive(Primitive::String),
                rules: vec![],
            })),
            id: None,
        });
    } else {
        return TopLevelType::ComplexType(ComplexType {
            name: &type_name.value(),
            annotation: None,
            additional_attributes: vec![],
            is_mixed: false,
            is_abstract: false,
        });
    }
}
