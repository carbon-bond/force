use crate::lexer::Token;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Type {
    OneLine,
    Text(Regex),
}

#[derive(Debug)]
struct Arrtibute {
    datatype: Type,
    name: String,
}

#[derive(Debug)]
enum Linkees {
    All,
    Names(String),
}

#[derive(Debug)]
enum Linkee {
    All,
    Name(String),
}

#[derive(Debug)]
struct Category {
    name: String,
    attributes: Vec<Arrtibute>,
    link_to: Linkees, // 可以鏈接到的分類名稱
}

#[derive(Debug)]
struct Link {
    from: String,
    to: Linkee,
}

type Categories = HashMap<String, Category>;
type Links = HashMap<(String, Linkee), Link>;

#[derive(Debug)]
pub struct Force {
    categories: Categories,
    links: Links,
}

#[derive(Debug)]
pub enum ForceError {
    NonExpect { expect: Token, fact: Token },
}

pub type ForceResult<T> = Result<T, ForceError>;

pub struct Parser {
    tokens: Vec<Token>,
    count: usize,
    cur: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            count: 0,
            cur: tokens[0].clone(),
            tokens: tokens,
        };
    }
    fn advance(&mut self) {
        self.count += 1;
        self.cur = self.tokens[self.count].clone()
    }
    fn eat(&mut self, expect: Token) -> ForceResult<()> {
        if self.cur == expect {
            self.advance();
            Ok(())
        } else {
            Err(ForceError::NonExpect {
                expect,
                fact: self.cur.clone(),
            })
        }
    }
    pub fn parse(&mut self) -> ForceResult<Force> {
        let categories = self.parse_categories()?;
        let links = self.parse_links()?;
        return Ok(Force { categories, links });
    }

    fn parse_categories(&mut self) -> ForceResult<Categories> {
        let mut categories = HashMap::new();
        loop {
            match self.cur {
                Token::Category => {
                    let category = self.parse_category()?;
                    categories.insert(category.name.clone(), category);
                }
                _ => {
                    break;
                }
            }
        }
        return Ok(categories);
    }

    fn parse_category(&mut self) -> ForceResult<Category> {
        self.eat(Token::Category);
        self.eat(Token::LeftCurlyBrace);
        unimplemented!();
    }

    fn parse_links(&mut self) -> ForceResult<Links> {
        Ok(HashMap::new())
    }
}
