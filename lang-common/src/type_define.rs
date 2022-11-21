use crate::types::{CompositeType, PrimitiveType};

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
/// - "Test" is name
/// - "id: usize,name: Option<String>" is properties
/// - "// this is test struct" is comment
/// - "#\[derive(Debug,Clone)\]" is attributes
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeTypeDefine {
    r#type: CompositeType,
    visibility: Visibility,
    comments: Option<Vec<Comment>>,
    attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Visibility(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute(String);

/// PrimitiveTypeAlias example is below
/// ```
/// // this is test data alias
/// #[cfg(test)]
/// type TestData = String;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveTypeAlias {
    alias: AliasName,
    r#type: PrimitiveType,
    comments: Option<Vec<Comment>>,
    attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasName(String);
