use sxd_document::dom::{
    Element as DomElement,
    Attribute as DomAttribute,
};

use parser::{
    parse_children,
    parse_child,
    is_of_element,
    XSD_NS_URI,
};

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#built-in-datatypes
#[derive(Eq, PartialEq, Debug)]
pub enum BuiltIn<'a> {
    String(&'a str),
    NormalizedString(&'a str),
    Token,
    Language(&'a str),
    Name,
    NcName,
    Id(Id<'a>),
    IdRef,
    IdRefs,
    Entity,
    Entities,
    NmToken,
    NmTokens,
    Boolean,
    Decimal,
    Integer,
    NonPositiveInteger,
    NegativeInteger,
    NonNegativeInteger,
    PositiveInteger,
    Long,
    UnsignedLong,
    Int,
    UnsignedInt,
    Short,
    UnsignedShort,
    Byte,
    UnsignedByte,
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
    QName,
    Notation,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Id<'a> {
    pub id: &'a str,
}


/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI
#[derive(Eq, PartialEq, Debug)]
pub struct AnyUri<'a> {
    pub uri: &'a str,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Language<'a> {
    pub iso_code: &'a str,
}


/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-appinfo
#[derive(Eq, PartialEq, Debug)]
pub struct Appinfo<'a> {
    pub source: Option<AnyUri<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub content: &'a str,  // FIXME: this is actually an inlined, mixed complex type
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-documentation
#[derive(Eq, PartialEq, Debug)]
pub struct Documentation<'a> {
    pub source: Option<AnyUri<'a>>,
    // xml:lang
    pub language: Option<Language<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub content: &'a str,  // FIXME: this is actually an inlined, mixed complex type
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation
#[derive(Eq, PartialEq, Debug)]
pub struct Annotation<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub appinfo: Vec<Appinfo<'a>>,
    pub documentation: Vec<Documentation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-restriction
#[derive(Eq, PartialEq, Debug)]
pub struct Restriction<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    // take from either base or nested simpleType
    pub annotation: Option<Annotation<'a>>,
    pub restriction_type: AnySimpleType<'a>,
    pub rules: Vec<RestrictionRule<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minInclusive
#[derive(Eq, PartialEq, Debug)]
pub struct MinExclusive<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minExclusive
#[derive(Eq, PartialEq, Debug)]
pub struct MinInclusive<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxInclusive
#[derive(Eq, PartialEq, Debug)]
pub struct MaxExclusive<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxExclusive
#[derive(Eq, PartialEq, Debug)]
pub struct MaxInclusive<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: &'a str,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-whiteSpace
#[derive(Eq, PartialEq, Debug)]
pub struct WhiteSpace<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: WhiteSpaceValue,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum WhiteSpaceValue {
    Collapse,
    Preserve,
    Replace,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-totalDigits
#[derive(Eq, PartialEq, Debug)]
pub struct TotalDigits<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-fractionDigits
#[derive(Eq, PartialEq, Debug)]
pub struct FractionDigits<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-pattern
#[derive(Eq, PartialEq, Debug)]
pub struct Pattern<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: &'a str,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-enumeration
#[derive(Eq, PartialEq, Debug)]
pub struct Enumeration<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: &'a str,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-length
#[derive(Eq, PartialEq, Debug)]
pub struct Length<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-minLength
#[derive(Eq, PartialEq, Debug)]
pub struct MinLength<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-maxLength
#[derive(Eq, PartialEq, Debug)]
pub struct MaxLength<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub value: usize,
    // default false
    pub fixed: bool,
    pub annotation: Option<Annotation<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
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
#[derive(Eq, PartialEq, Debug)]
pub struct Union<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    pub annotation: Option<Annotation<'a>>,
    pub member_types: Vec<SimpleType<'a>>,  // choose from memberTypes(QName) or nested simpleType
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-list
#[derive(Eq, PartialEq, Debug)]
pub struct List<'a> {
    pub id: Option<Id<'a>>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    // choose from itemType(QName) or nested simpleType
    pub annotation: Option<Annotation<'a>>,
    pub item_type: SimpleType<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum AnyType<'a> {
    ComplexType(ComplexType<'a>),
    AnySimpleType(AnySimpleType<'a>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum AnySimpleType<'a> {
    BuiltIn(BuiltIn<'a>),
    SimpleType(SimpleType<'a>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum TopLevelType<'a> {
    SimpleType(SimpleType<'a>),
    ComplexType(ComplexType<'a>),
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-complexType
#[derive(Eq, PartialEq, Debug)]
pub struct ComplexType<'a> {
    pub name: &'a str,
    pub id: Option<Id<'a>>,
    // defaults to false
    pub is_mixed: bool,
    pub annotation: Option<Annotation<'a>>,
    // defaults to false
    pub is_abstract: bool,
    // block = (#all | List of (extension | restriction))
    // final = (#all | List of (extension | restriction))
    pub additional_attributes: Vec<DomAttribute<'a>>,
    // Content: (simpleContent | complexContent | ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?)))
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-simpleType
#[derive(Eq, PartialEq, Debug)]
pub struct SimpleType<'a> {
    pub id: Option<Id<'a>>,
    pub name: &'a str,
    pub content: Box<SimpleTypeContent<'a>>,
    pub annotation: Option<Annotation<'a>>,
    // final = (#all | List of (list | union | restriction))
    pub additional_attributes: Vec<DomAttribute<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
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

pub fn parse_id<'a>(element: &DomElement<'a>) -> Option<Id<'a>> {
    element.attribute("id")
        .map(|attr| Id { id: &attr.value() })
}

pub fn parse_additional_attributes<'a>(element: &DomElement<'a>) -> Vec<DomAttribute<'a>> {
    element.attributes().into_iter()
        .filter(|&attr| {
            match attr.name().namespace_uri() {
                Some(namespace) => namespace != XSD_NS_URI,
                _ => false
            }
        })
        .collect()
}

pub fn parse_annotation<'a>(element: &DomElement<'a>) -> Option<Annotation<'a>> {
    parse_child(&element,
                |&el| is_of_element(&el, "annotation"),
                |el| {
                    Annotation {
                        id: parse_id(&element),
                        additional_attributes: Vec::new(),
                        appinfo: Vec::new(),
                        documentation: Vec::new(),
                    }
                })
}

pub fn parse_boolean_attribute<'a>(element: &DomElement<'a>, name: &str, default: bool) -> bool {
    element.attribute(name)
        .map(|attr| attr.value() == "true")
        .unwrap_or(default)
}

pub fn parse_type<'a>(element: DomElement<'a>) -> TopLevelType<'a> {
    let type_name = element.attribute("name").expect("Element defined without name");
    if element.name().local_part() == "simpleType" {
        return TopLevelType::SimpleType(SimpleType {
            name: &type_name.value(),
            annotation: parse_annotation(&element),
            additional_attributes: parse_additional_attributes(&element),
            id: parse_id(&element),
            content: Box::new(SimpleTypeContent::Restriction(Restriction {
                additional_attributes: vec![],
                annotation: None,
                id: None,
                restriction_type: AnySimpleType::BuiltIn(BuiltIn::String),
                rules: vec![
                    RestrictionRule::Pattern(Pattern {
                        id: None,
                        additional_attributes: vec![],
                        value: "\\d{3}-[A-Z]{2}",
                        annotation: None,
                    })
                ],
            })),

        });
    } else {
        return TopLevelType::ComplexType(ComplexType {
            name: &type_name.value(),
            id: parse_id(&element),
            annotation: parse_annotation(&element),
            additional_attributes: parse_additional_attributes(&element),
            is_mixed: parse_boolean_attribute(&element, "mixed", false),
            is_abstract: parse_boolean_attribute(&element, "mixed", false),
        });
    }
}
