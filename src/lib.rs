pub mod convertor;
pub mod json;
pub mod json_to_struct;
pub mod rust {
    pub(super) mod builder;
    pub(super) mod reserved_words;
    pub mod rust_struct_convertor;
}
pub mod traits {
    pub mod type_mapper {
        pub mod array;
        pub mod optional;
        pub mod optional_array;
        pub mod primitive;
    }
    pub mod type_statements {
        pub mod type_attr;
        pub mod type_comment;
        pub mod type_statement;
        pub mod type_visibility;
    }
    pub mod filed_statements {
        pub mod filed_attr;
        pub mod filed_comment;
        pub mod filed_statement;
        pub mod filed_visibility;
    }
    pub mod off_side_rule;
    pub mod optional_checker;
    pub mod reserved_words;
}
pub mod lang_common {
    pub mod naming_principal;
}
pub mod type_gen;
