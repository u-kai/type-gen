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
pub type TypeDefine = String;
