#[allow(dead_code)]
pub enum XSDType<'a> {
    ComplexType(ComplexType<'a>),
    SimpleType(SimpleType<'a>),
}

#[allow(dead_code)]
pub struct ComplexType<'a> {
    pub name: &'a str
}

#[allow(dead_code)]
pub struct SimpleType<'a> {
    pub name: &'a str
}