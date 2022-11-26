#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Boolean,
    Number(Number),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    Usize,
    Isize,
    Float,
}
//#[cfg(test)]
pub mod primitive_type_factories {
    use super::{Number, PrimitiveType};
    pub fn make_string() -> PrimitiveType {
        PrimitiveType::String
    }
    #[allow(unused)]
    pub fn make_bool() -> PrimitiveType {
        PrimitiveType::Boolean
    }
    pub fn make_usize() -> PrimitiveType {
        PrimitiveType::Number(Number::Usize)
    }
    #[allow(unused)]
    pub fn make_isize() -> PrimitiveType {
        PrimitiveType::Number(Number::Isize)
    }
    #[allow(unused)]
    pub fn make_float() -> PrimitiveType {
        PrimitiveType::Number(Number::Float)
    }
}
