pub trait TypeStatement {
    const TYPE_STATEMENT: &'static str;
    fn create_statement(&self, type_key: &str) -> String;
}
