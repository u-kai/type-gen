pub trait FiledStatement {
    const HEAD_SPACE: &'static str = "    ";
    const FILED_DERIMITA: &'static str = ",";
    fn create_statement(&self, filed_key: &str, filed_type: &str) -> String;
    fn add_head_space(&self, statement: String) -> String {
        format!("{}{}", Self::HEAD_SPACE, statement)
    }
}
