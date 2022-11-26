pub trait LangType {
    fn get_lang_type(&self) -> &'static str;
}
