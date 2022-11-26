pub trait Parser<T>: Fn(&str) -> Option<(T, &str)> {}
impl<T, F> Parser<T> for F where F: Fn(&str) -> Option<(T, &str)> {}
fn add_lifetime<T, F>(f: F) -> F
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    f
}
pub fn skip_character(c: char) -> impl Parser<()> {
    add_lifetime(move |s| {
        let mut chars = s.chars();
        if chars.next() == Some(c) {
            return None;
        };
        Some(((), chars.as_str()))
    })
}

pub fn trim_head<'a>(data: &'a str) -> Option<((), &'a str)> {
    let new_len = data.chars().skip_while(|c| c.is_whitespace()).count();
    let start = data.len() - new_len;
    Some(((), &data[start..]))
}
#[cfg(test)]
mod test_parsers {
    use super::*;
    #[test]
    fn test_trim_empty() {
        let data = "   hello world";
        let tobe = "hello world";
        assert_eq!(trim_head(data).unwrap().1, tobe);
    }
}
