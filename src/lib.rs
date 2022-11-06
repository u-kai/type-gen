pub mod convertor;
pub mod json;
pub mod json_to_struct;
pub mod rust_v1 {
    pub(super) mod builder;
    pub(super) mod reserved_words;
    pub mod rust_struct_convertor;
}
pub mod traits {
    pub mod json_lang_mapper {
        pub mod array;
        pub mod json_lang_mapper;
        pub mod optional;
        pub mod optional_array;
        pub mod primitive;
    }
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
        pub mod optional_checker;
        pub mod reserved_words;
    }
    pub mod off_side_rule;
}
pub mod lang_common {
    pub mod filed_comment;
    pub mod naming_principal;
    pub mod optional_checker;
    pub mod type_comment;
}
pub mod lang_config;
pub mod type_gen;
pub mod type_generator;
pub mod rust {
    pub mod type_gen;
    pub mod filed_statements {
        pub mod filed_attr;
        pub mod filed_statement;
        pub mod filed_visibilty;
        pub mod reserved_words;
    }
    pub mod off_side_rule;
    pub mod rust_type_gen;
    pub mod rust_visibility;
    pub mod rust_visibility_provider;
    pub mod type_statements {
        pub mod type_attr;
        pub mod type_statement;
        pub mod type_visiblity;
    }
    pub mod json_lang_mapper {
        pub mod array;
        pub mod json_lang_mapper;
        pub mod optional;
        pub mod optional_array;
        pub mod primitive;
    }
}
