pub type TypeStatement = String;
pub trait LangTypeMapper {
    fn case_string(&self) -> TypeStatement;
    fn case_boolean(&self) -> TypeStatement;
    fn case_usize(&self) -> TypeStatement;
    fn case_isize(&self) -> TypeStatement;
    fn case_float(&self) -> TypeStatement;
    fn case_null(&self) -> TypeStatement;
    fn case_any(&self) -> TypeStatement;
    fn case_array_type<T: Into<TypeStatement>>(&self, type_statement: T) -> TypeStatement;
    fn case_optional_type<T: Into<TypeStatement>>(&self, type_statement: T) -> TypeStatement;
}
