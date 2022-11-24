pub(crate) struct TypeConfiguration<'a> {
    root_name: &'a str,
    all_type_visibility: Option<&'a str>,
    all_property_visibility: Option<&'a str>,
    all_type_comment: Option<&'a str>,
    all_property_comment: Option<&'a str>,
    //all_type_comment: Option<&'a str>,
    //all_property_comment: Option<&'a str>,
}
