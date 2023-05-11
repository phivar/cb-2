use logos::{Lexer, Logos};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
	// TODO 
	#[regex("<a\\s+(name\\s*=\\s*\"[^\"]*\")?\\s*(href\\s*=\\s*\"[^\"]*\")\\s*(name\\s*=\\s*\"[^\"]*\")?\\s*>[^<]*</a[\\s\n\r]*>", extract_link_info)]
    Link((LinkUrl, LinkText)),

	#[regex("<[^>]*>", logos::skip)]
	#[regex("[^<]+",logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

#[derive(Logos, Debug, PartialEq)]
pub enum ArgToken {
	#[regex(r"[ \t\d]+",logos::skip)]
	#[token("<a",logos::skip)]
	#[regex("/a\\s*>",logos::skip)]
	#[regex("name\\s*=\\s*\"[^\"]*\"",logos::skip)]
	#[error]
    Error,

	#[regex("href\\s*=\\s*\"[^\"]*\"", extract_pair)]
	Href(String),

	#[regex(">[^<]*<",extract_text)]
	Text(String),

}

fn extract_pair(lex: &mut Lexer<ArgToken>) -> String {
	let s = lex.slice();
	let i = s.find("\"").unwrap();
	(&s[i+1 .. s.len()-1]).to_string()
}

fn extract_text(lex: &mut Lexer<ArgToken>) -> String {
	let s = lex.slice();
	(&s[1..s.len()-1]).to_string()
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
	let mut lex = ArgToken::lexer(lex.slice());
    // TODO: Implement extraction from link definition
    if let ArgToken::Href(s1) = lex.next().unwrap(){
		if let ArgToken::Text(s2) = lex.next().unwrap(){
			return (LinkUrl(s1),LinkText(s2));
		}
		panic!();
	}
	panic!();
}
