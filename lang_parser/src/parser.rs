pub enum Surround {
    Parentheses,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
    DoubleQuote,
    SingleQuote,
}
impl From<char> for Surround {
    fn from(c: char) -> Self {
        match c {
            '{' | '}' => Self::CurlyBracket,
            '"' => Self::DoubleQuote,
            '\'' => Self::SingleQuote,
            '<' | '>' => Self::AngleBracket,
            '(' | ')' => Self::Parentheses,
            '[' | ']' => Self::SquareBracket,
            _ => panic!("not consider {}", c),
        }
    }
}
impl Surround {
    pub fn read_surrounded<'a>(&self, source: &'a str) -> &'a str {
        let mut acc_start_num = 0;
        let mut start_index = 0;
        let mut end_index = 0;

        for (i, c) in source.chars().enumerate() {
            if c == self.start() {
                if acc_start_num == 0 {
                    start_index += i + 1;
                };
                acc_start_num += 1;
            }
            if c == self.end() {
                acc_start_num -= 1;
                if acc_start_num == 0 {
                    end_index = i;
                    break;
                }
            }
        }
        &source[start_index..end_index]
    }
    pub fn start(&self) -> char {
        match self {
            Surround::Parentheses => '(',
            Surround::CurlyBracket => '{',
            Surround::DoubleQuote => '"',
            Surround::SingleQuote => '\'',
            Surround::SquareBracket => '[',
            Surround::AngleBracket => '<',
        }
    }
    pub fn end(&self) -> char {
        match self {
            Surround::Parentheses => ')',
            Surround::CurlyBracket => '}',
            Surround::DoubleQuote => '"',
            Surround::SingleQuote => '\'',
            Surround::SquareBracket => ']',
            Surround::AngleBracket => '>',
        }
    }
}

pub fn read_after_match<'a>(source: &'a str, match_str: &str) -> &'a str {
    if let Some(match_index) = source.find(match_str) {
        &source[match_index..]
    } else {
        source
    }
}

pub fn skip_whitespace(source: &str) -> &str {
    if let Some(index) = source.find(|c: char| !c.is_whitespace()) {
        &source[index..]
    } else {
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    #[test]
    fn 囲まれている範囲の文字列を読み取る() {
        let source = "{hello world}";

        let sut = Surround::CurlyBracket;

        let result = sut.read_surrounded(source);
        assert_eq!("hello world", result);
    }
    #[test]
    fn 空白以外の文字まで読み飛ばす() {
        let tobe = r#"let data = "data";"#;
        let source = format!(r#"      {}"#, tobe);

        let result = skip_whitespace(&source);

        assert_eq!(tobe, result);
    }
}
