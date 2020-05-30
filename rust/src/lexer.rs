use crate::CategoryAttribute;
use crate::DataType;
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
    #[token = ","]
    Comma,
    #[token = "#"]
    Sharp,

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
    #[token = "-"]
    Minus,
    #[token = "->"]
    Arrow,
    #[token = "*"]
    Star,
    #[token = "輸能"]
    Transfuse,

    // 識別子，只能是中文、英文、數字、底線
    // TODO: 增強識別子的限制
    #[regex = "[^\\s/\\[\\]]+"]
    Identifier,
}

// 自定義的 Token 型別
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBracket,
    RightSquareBracket,

    Comma,

    AttachTo,
    Minus,
    Arrow,
    Transfuse,
    Star,

    Identifier(String),
    Type(DataType),
    CategoryAttribute(CategoryAttribute),

    Regex(String),

    // 文本結束
    End,
}

pub fn lexer(source: &str) -> Vec<Token> {
    println!("詞法分析開始");
    let mut ret = Vec::new();
    let mut lexer = LogoToken::lexer(source);
    loop {
        match lexer.token {
            LogoToken::Error => {
                panic!("未定義的 token: {}", lexer.slice());
            }
            LogoToken::End => {
                ret.push(Token::End);
                break;
            }
            LogoToken::LeftCurlyBrace => {
                ret.push(Token::LeftCurlyBrace);
            }
            LogoToken::LeftSquareBracket => {
                ret.push(Token::LeftSquareBracket);
            }
            LogoToken::RightCurlyBrace => {
                ret.push(Token::RightCurlyBrace);
            }
            LogoToken::RightSquareBracket => {
                ret.push(Token::RightSquareBracket);
            }
            LogoToken::Comma => {
                ret.push(Token::Comma);
            }
            // NOTE: 暫時爲了測試方便，將井字號視作結束符
            LogoToken::Sharp => {
                ret.push(Token::End);
            }
            LogoToken::AttachTo => {
                ret.push(Token::AttachTo);
            }
            LogoToken::Minus => {
                ret.push(Token::Minus);
            }
            LogoToken::Arrow => {
                ret.push(Token::Arrow);
            }
            LogoToken::Transfuse => {
                ret.push(Token::Transfuse);
            }
            LogoToken::Star => {
                ret.push(Token::Star);
            }

            LogoToken::OneLine => ret.push(Token::Type(DataType::OneLine)),
            LogoToken::Text => ret.push(Token::Type(DataType::Text)),
            LogoToken::Number => ret.push(Token::Type(DataType::Number)),

            LogoToken::NotRoot => {
                ret.push(Token::CategoryAttribute(CategoryAttribute::NotRoot));
            }

            LogoToken::Regex => {
                ret.push(Token::Regex(lexer.slice().to_string()));
            }
            LogoToken::Identifier => {
                ret.push(Token::Identifier(lexer.slice().to_string()));
            }
        }
        lexer.advance();
    }
    ret
}
