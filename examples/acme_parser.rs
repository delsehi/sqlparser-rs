use std::str::Chars;
use std::iter::Peekable;

extern crate datafusion_sql;

use datafusion_sql::ansi::tokenizer::ANSISQLTokenizer;
use datafusion_sql::tokenizer::*;
use datafusion_sql::parser::*;

/// This example demonstrates building a custom ACME parser that extends the generic parser
/// by adding support for a factorial expression `!! expr`.

/// Custom SQLToken
#[derive(Debug,PartialEq)]
enum AcmeToken {
    /// Factorial token `!!`
    Factorial
}

/// Custom SQLExpr
#[derive(Debug)]
enum AcmeExpr {
    /// Factorial expression
    Factorial(Box<SQLExpr<AcmeExpr>>)
}

struct AcmeTokenizer {
    generic: ANSISQLTokenizer
}

/// The ACME tokenizer looks for the factorial operator `!!` but delegates everything else
impl SQLTokenizer<AcmeToken> for AcmeTokenizer {

    fn precedence(&self, token: &SQLToken<AcmeToken>) -> usize {
        unimplemented!()
    }

    fn peek_token(&self, chars: &mut Peekable<Chars>) -> Result<Option<SQLToken<AcmeToken>>, TokenizerError<AcmeToken>> {
        unimplemented!()
    }

    fn next_token(&self, chars: &mut Peekable<Chars>) -> Result<Option<SQLToken<AcmeToken>>, TokenizerError<AcmeToken>> {
        match chars.peek() {
            Some(&ch) => match ch {
                '!' => {
                    chars.next(); // consume the first `!`
                    match chars.peek() {
                        Some(&ch) => match ch {
                            '!' => {
                                chars.next(); // consume the second `!`
                                Ok(Some(SQLToken::Custom(AcmeToken::Factorial)))
                            },
                            _ => Err(TokenizerError::UnexpectedChar(ch,Position::new(0,0)))
                        },
                        None => Ok(Some(SQLToken::Not))
                    }
                }
                _ => self.generic.next_token(chars)
            }
            _ => self.generic.next_token(chars)
        }
    }
}

struct AcmeParser<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> AcmeParser<'a> {

    pub fn new(sql: &'a str) -> Self {
        AcmeParser {
            chars: sql.chars().peekable()
        }
    }
}

impl<'a> SQLParser<AcmeToken, AcmeExpr> for AcmeParser<'a> {

    fn parse_prefix(&mut self) -> Result<Box<SQLExpr<AcmeExpr>>, ParserError<AcmeToken>> {
        unimplemented!()
    }

    fn parse_infix(&mut self, left: &SQLExpr<AcmeExpr>, precedence: usize) -> Result<Option<Box<SQLExpr<AcmeExpr>>>, ParserError<AcmeToken>> {
        unimplemented!()
    }
}


fn main() {

    let sql = "1 + !! 5 * 2";

    let acme_parser = AcmeParser::new(sql);


    //acme_parser

//    let mut acme_tokenizer = AcmeTokenizer {
//        generic: ANSISQLTokenizer { }
//    };
//
//    let tokens = tokenize(&sql, &mut acme_tokenizer).unwrap();
//
//    println!("tokens = {:?}", tokens);



}
