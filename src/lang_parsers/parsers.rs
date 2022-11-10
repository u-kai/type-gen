pub fn trim_head<'a>(data: &'a str) -> Option<((), &'a str)> {
    let new_len = data
        .chars()
        .skip_while(|c| c.is_whitespace() || c.is_ascii_whitespace())
        .count();
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
