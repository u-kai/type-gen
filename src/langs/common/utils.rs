
pub fn replace_cannot_use_char(str: &str) -> String {
    str.replace(
        |c| match c {
            ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
            | '!' | '<' | '>' => true,
            _ => false,
        },
        "",
    )
}
