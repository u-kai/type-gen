use crate::types::statement::TypeStatement;

/// ObjectTypeDefine example is below
/// ```
/// // this is test struct
/// #[derive(Debug,Clone)]
/// struct Test {
///     // id is must set
///     id: usize,
///     name: Option<String>
/// }
/// ```
///
/// PrimitiveTypeAlias example is below
/// ```
/// // this is test data alias
/// #[cfg(test)]
/// type TestData = String;
/// ```
///
/// - "Test" is name
/// - "id: usize,name: Option<String>" is properties
/// - "// this is test struct" is comment
/// - "#\[derive(Debug,Clone)\]" is attributes
///
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefine<V, C, A>
where
    V: LangVisibility,
    C: LangComment,
    A: LangAttribute,
{
    statement: TypeStatement,
    visibility: V,
    comment: Option<C>,
    attribute: Option<A>,
}
impl<V, C, A> TypeDefine<V, C, A>
where
    V: LangVisibility,
    C: LangComment,
    A: LangAttribute,
{
    pub fn new(
        statement: TypeStatement,
        visibility: V,
        comment: Option<C>,
        attribute: Option<A>,
    ) -> Self {
        Self {
            statement,
            visibility,
            comment,
            attribute,
        }
    }
}
pub trait LangVisibility {
    fn to_define(self) -> String;
}
pub trait LangComment {
    fn to_define(self) -> String;
}
pub trait LangAttribute {
    fn to_define(self) -> String;
}

#[cfg(test)]
mod test_composite_type_to_define {
    #[test]
    fn test_simple_case() {}
}