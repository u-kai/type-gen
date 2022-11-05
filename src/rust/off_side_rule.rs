use crate::traits::off_side_rule::OffSideRule;

pub struct RustOffSideRule;

impl RustOffSideRule {
    const START_AND_NEXT_LINE: &'static str = "{\n";
    const END: &'static str = "}";
    pub fn new() -> Self {
        Self
    }
}
impl OffSideRule for RustOffSideRule {
    fn start(&self) -> &'static str {
        Self::START_AND_NEXT_LINE
    }
    fn end(&self) -> &'static str {
        Self::END
    }
}

#[cfg(test)]
mod rust_offsiderule {
    use super::*;
    #[test]
    fn test_write_fn() {
        let osr = RustOffSideRule::new();
        let tobe = r#"fn main() {
println!("hello");
}"#;
        let result = format!(
            r#"fn main() {}println!("hello");
{}"#,
            osr.start(),
            osr.end()
        );
        assert_eq!(result, tobe.to_string())
    }
}
