use crate::types::structures::TypeStructure;

pub trait ParseToTypeStructure {
    fn parse_to(self) -> Vec<TypeStructure>;
}
