use logos::Logos;

// 先藉助 logos 函式庫自動生成 LogoToken ，再將之轉成自定義的 Token

// logos 詞法分析庫的 token
#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum LogoToken {
    #[end]
    End,
    #[error]
    Error,

    // 特殊符號
    #[token = "{"]
    LeftCurlyBrace,
    #[token = "}"]
    RightCurlyBrace,
    #[token = "["]
    LeftSquareBracket,
    #[token = "]"]
    RightSquareBracket,

    // 宣告
    #[token = "分類"]
    Class,
    #[token = "鍵結"]
    Link,

    // 域型別
    #[token = "單行"]
    OneLine,
    #[token = "文本"]
    Text,
    #[token = "數字"]
    Number,

    // 分類的屬性
    #[token = "非根"]
    NotRoot,

    // 正則表達式
    #[regex = "/[^/]+/"]
    Regex,

    // 鍵結的符號
    #[token = "-->"]
    AttachTo,
    #[token = "*"]
    Star,

    // 識別子，只能是中文、英文、數字、底線
    // TODO: 增強識別子的限制
    #[regex = "[^\\s/\\[\\]]+"]
    Identifier,
}

#[derive(Debug)]
pub enum DataType {
    OneLine,
    Text,
    Number,
}

// 自定義的 Token 型別
#[derive(Debug)]
pub enum Token {
    Link,
    Class,

    Symbol(String),
    Type(DataType),

    Regex(String),
}

pub fn lexer(source: &str) -> Vec<LogoToken> {
    println!("詞法分析開始");
    let mut ret = Vec::new();
    let mut lexer = LogoToken::lexer(source);
    loop {
        match lexer.token {
            LogoToken::Error => {
                panic!("未定義的 token: {}", lexer.slice());
            }
            LogoToken::End => {
                break;
            }
            token => {
                ret.push(token);
            }
        }
        lexer.advance();
    }
    ret
}
