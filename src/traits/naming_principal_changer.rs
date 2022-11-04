pub trait NamingPrincipalChanger {
    fn change_statement(&self, filed_statement: &mut String) -> ();
}
