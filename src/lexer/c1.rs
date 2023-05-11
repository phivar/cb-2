use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
	#[regex(r"[\r]+", logos::skip)]
    #[regex(r"//[^\n]*\n", logos::skip)]
	#[regex(r"/\*([^\*]|(\*[^/]))*\*/", logos::skip)]
	#[regex(r"[ \t\n]+", logos::skip)]
	#[error]
	Error,
	// keywords
	#[token("bool")]
	KwBoolean,
	#[token("do")]
	KwDo,
	#[token("else")]
	KwElse,
	#[token("float")]
	KwFloat,
	#[token("for")]
	KwFor,
	#[token("if")]
	KwIf,
	#[token("int")]
	KwInt,
	#[token("printf")]
	KwPrintf,
	#[token("return")]
	KwReturn,
	#[token("void")]
	KwVoid,
	#[token("while")]
	KwWhile,

	// operators
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("*")]
	Asterisk,
	#[token("/")]
	Slash,
	#[token("=")]
	Assign,
	#[token("==")]
	Eq,
	#[token("!=")]
	Neq,
	#[token("<")]
	Lss,
	#[token(">")]
	Grt,
	#[token("<=")]
	Leq,
	#[token(">=")]
	Geq,
	#[token("&&")]
	And,
	#[token("||")]
	Or,

	// other tokens
	#[token(",")]
	Comma,
	#[token(";")]
	Semicolon,
	#[token("(")]
	LParen,
	#[token(")")]
	RParen,
	#[token("{")]
	LBrace,
	#[token("}")]
	RBrace,

	// terms
	#[regex("[0-9]+")]
	ConstInt,
	#[regex(r"((\d+\.\d+)|(\.\d+))([eE][\+\-]?\d+)?|\d+[eE][\+\-]?\d+")]
	ConstFloat,
	#[regex(r"true|false")]
	ConstBoolean,
	#[regex("\"[^\\n\"]*\"")]
	ConstString,
	#[regex(r"[a-zA-Z]+(\d|[a-zA-Z])*")]
	Id,
}
