use std::collections::HashSet;

use sxd_document::dom::{
    Element as DomElement,
    Attribute as DomAttribute,
};

use parser::{
    XSD_NS_URI,
    parse_children,
    parse_child,
    is_of_element,
    parse_additional_attributes,
    parse_id,
    parse_boolean_attribute,
};

use parser::annotations::{
    Annotation,
    parse_annotation,
};


/// This is a list of already built in simple types that can be referenced by using
/// NS:Type, e.g. xsd:string where xsd = http://www.w3.org/2001/XMLSchema
/// They hold no values since their implementations are built in
/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#built-in-datatypes
#[derive(Eq, PartialEq, Debug)]
pub enum BuiltIn {
    String,
    NormalizedString,
    Token,
    Language,
    Name,
    NcName,
    Id,
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

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID
#[derive(Eq, PartialEq, Debug)]
pub struct Id<'a> {
    pub id: &'a str,
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
    BuiltIn(BuiltIn),
    SimpleType(SimpleType<'a>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum TopLevelType<'a> {
    SimpleType(SimpleType<'a>),
    ComplexType(ComplexType<'a>),
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum SimpleFinal {
    Extension,
    Restriction,
    Union,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum ComplexFinal {
    Extension,
    Restriction,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum ComplexBlock {
    Extension,
    Restriction,
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
    pub block_modes: HashSet<ComplexBlock>,
    pub final_modes: HashSet<ComplexFinal>,
    pub additional_attributes: Vec<DomAttribute<'a>>,
    // TODO: Content: (simpleContent | complexContent | ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?)))
}

/// see https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-simpleType
#[derive(Eq, PartialEq, Debug)]
pub struct SimpleType<'a> {
    pub id: Option<Id<'a>>,
    pub name: &'a str,
    pub content: Box<SimpleTypeContent<'a>>,
    pub annotation: Option<Annotation<'a>>,
    pub final_modes: HashSet<SimpleFinal>,
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

pub fn parse_type<'a>(element: DomElement<'a>) -> TopLevelType<'a> {
    let type_name = element.attribute("name").expect("Element defined without name");
    if element.name().local_part() == "simpleType" {
        return TopLevelType::SimpleType(SimpleType {
            name: &type_name.value(),
            annotation: parse_annotation(&element),
            final_modes: HashSet::new(),
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
            block_modes: HashSet::new(),
            final_modes: HashSet::new(),
        });
    }
}
