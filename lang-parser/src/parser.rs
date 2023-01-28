pub struct SyntaxParser {
    syntax: String,
    fn_id: &'static str,
}
pub struct FnSyntax {
    name: String,
}
pub enum OffSideRule {
    Indent,
    Bracket,
}
impl FnSyntax {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
impl SyntaxParser {
    pub fn new(syntax: impl Into<String>, fn_id: &'static str) -> Self {
        Self {
            syntax: syntax.into(),
            fn_id,
        }
    }
    pub fn fns(&self) -> impl Iterator<Item = FnSyntax> {
        [FnSyntax::new("test")].into_iter().map(|f| f)
    }
}
pub fn read_until_end(source: &str, surronud: Surround) -> &str {
    let mut acc_start_num = 0;
    let mut start_index = 0;
    let mut end_index = 0;

    for (i, c) in source.chars().enumerate() {
        if c == surronud.start() {
            if acc_start_num == 0 {
                start_index += i + 1;
            };
            acc_start_num += 1;
        }
        if c == surronud.end() {
            acc_start_num -= 1;
            if acc_start_num == 0 {
                end_index = i;
                break;
            }
        }
    }
    &source[start_index..end_index]
}
pub enum Surround {
    Bracket,
    DoubleQute,
    SingleQute,
}
impl Surround {
    pub fn start(&self) -> char {
        match self {
            Surround::Bracket => '{',
            Surround::DoubleQute => '"',
            Surround::SingleQute => '\'',
        }
    }
    pub fn end(&self) -> char {
        match self {
            Surround::Bracket => '}',
            Surround::DoubleQute => '"',
            Surround::SingleQute => '\'',
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

pub fn read_after_empty(source: &str) -> &str {
    if let Some(index) = source.find(|c: char| !c.is_whitespace()) {
        &source[index..]
    } else {
        source
    }
}

#[cfg(test)]
#[test]
fn 空白以外の文字まで読み飛ばす() {
    let tobe = r#"let data = "data";"#;
    let source = format!(r#"      {}"#, tobe);

    let result = read_after_empty(&source);

    assert_eq!(tobe, result);
}
#[cfg(test)]
#[test]
fn マッチする値まで文字列を読み飛ばす() {
    let source = r#"let data = "data"; fn {hello world};"#;
    let result = read_after_match(source, "fn ");
    assert_eq!("fn {hello world};", result);
}
#[cfg(test)]
#[test]
fn 対応するbracketまで文字列を読み取る() {
    let source = "{hello world}";
    let result = read_until_end(source, Surround::Bracket);
    assert_eq!("hello world", result);
}
#[test]
fn ネストしている対応するbracketまで文字列を読み取る() {
    let source = "{ fn test(){hello world} }";
    let result = read_until_end(source, Surround::Bracket);
    assert_eq!(" fn test(){hello world} ", result);
}
// #[cfg(test)]
// mod 関数定義の識別子がfnかつoffsideruleがbracesの場合 {
//     use super::*;
//     #[test]
//     fn 空実装の関数を識別して関数名を取得する() {
//         let fn_str = "fn test(){}";
//         let sut = SyntaxParser::new(fn_str, "fn");

//         let fnsyntax = sut.fns().next().unwrap();
//         let result = fnsyntax.name();
//         assert_eq!(result, "test");

//         let fn_str = "fn test_fn(){}";
//         let sut = SyntaxParser::new(fn_str, "fn");

//         let fnsyntax = sut.fns().next().unwrap();
//         let result = fnsyntax.name();
//         assert_eq!(result, "test_fn");
//
//     }
// }
