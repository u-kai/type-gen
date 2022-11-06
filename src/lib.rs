pub mod json;
pub mod utils {
    pub mod store_fn;
}
pub mod traits {
    pub mod json_lang_mapper;
    pub mod optional_checker;
    pub mod type_statements {
        pub mod lang_type;
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
        pub mod reserved_words;
    }
    pub mod off_side_rule;
}
pub mod lang_common {
    pub mod filed_comment;
    pub mod naming_principal;
    pub mod optional_checker;
    pub mod type_comment;
    pub mod type_define_generator;
}
pub mod rust {
    pub mod filed_statements {
        pub mod filed_attr;
        pub mod filed_statement;
        pub mod filed_visibilty;
        pub mod reserved_words;
    }
    pub mod off_side_rule;
    pub mod rust_visibility;
    pub mod rust_visibility_provider;
    pub mod type_gen_builder;
    pub mod type_statements {
        pub mod type_attr;
        pub mod type_statement;
        pub mod type_visiblity;
    }
    pub mod json_lang_mapper;
}
