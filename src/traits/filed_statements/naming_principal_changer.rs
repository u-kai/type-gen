use super::filed_statement::FiledStatement;

pub trait NamingPrincipalChanger {
    fn change_statement(&self, key: &str, statement: &impl FiledStatement) -> String;
}
