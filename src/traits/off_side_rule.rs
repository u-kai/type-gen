pub trait OffSideRule {
    fn start(&self) -> &'static str;
    fn end(&self) -> &'static str;
}
