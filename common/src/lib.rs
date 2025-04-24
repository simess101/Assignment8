```rust
//! Shared definitions for tokenizer and recognizer
use std::fmt;

/// All token kinds as per lexical structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    WhileKeyword,
    ReturnKeyword,
    Equal,
    Comma,
    Eol,
    VarTypeInt,
    VarTypeVoid,
    Identifier,
    BinOpPlus,
    BinOpMul,
    BinOpNe,
    BinOpEq,
    BinOpMod,
    Number,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        let s = match self {
            LeftParen    => "LEFT_PARENTHESIS",
            RightParen   => "RIGHT_PARENTHESIS",
            LeftBracket  => "LEFT_BRACKET",
            RightBracket => "RIGHT_BRACKET",
            WhileKeyword => "WHILE_KEYWORD",
            ReturnKeyword=> "RETURN_KEYWORD",
            Equal        => "EQUAL",
            Comma        => "COMMA",
            Eol          => "EOL",
            VarTypeInt   => "VARTYPE",
            VarTypeVoid  => "VARTYPE",
            Identifier   => "IDENTIFIER",
            BinOpPlus    => "BINOP",
            BinOpMul     => "BINOP",
            BinOpNe      => "BINOP",
            BinOpEq      => "BINOP",
            BinOpMod     => "BINOP",
            Number       => "NUMBER",
        };
        write!(f, "{}", s)
    }
}

/// A lexeme with its token kind
#[derive(Debug, Clone)]
pub struct Lex {
    pub kind: TokenKind,
    pub lexeme: String,
}
```