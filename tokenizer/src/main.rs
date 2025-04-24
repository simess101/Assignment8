```rust
use std::{env, fs};
use common::{Lex, TokenKind};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: tokenizer <input> <output>");
        std::process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let mut lexemes = Vec::new();
    let mut cur = String::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ch if ch.is_whitespace() => { flush_alphanumeric(&mut cur, &mut lexemes); chars.next(); }
            '(' | ')' | '{' | '}' | '=' | ',' | ';' | '+' | '*' | '!' | '%' => {
                flush_alphanumeric(&mut cur, &mut lexemes);
                let lex = match c {
                    '(' => "(", ')' => ")",
                    '{' => "{", '}' => "}",
                    '=' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            chars.next(); "==" } else { "=" }
                    }
                    '+' => "+", '*' => "*",
                    '!' => { chars.next(); if let Some('=') = chars.peek() { chars.next(); "!=" } else { "!" } }
                    '%' => "%", ',' => ",", ';' => ";",
                    _ => unreachable!(),
                };
                lexemes.push(lex.to_string());
                continue;
            }
            _ => { cur.push(c); chars.next(); }
        }
    }
    flush_alphanumeric(&mut cur, &mut lexemes);
    let tokens: Vec<Lex> = lexemes
        .into_iter()
        .map(|s| Lex { kind: classify(&s), lexeme: s })
        .collect();

    let mut out = String::new();
    for lex in tokens {
        out.push_str(&format!("{} {}\n", lex.kind, lex.lexeme));
    }
    fs::write(&args[2], out).expect("Failed to write output file");
}

fn flush_alphanumeric(cur: &mut String, lexemes: &mut Vec<String>) {
    if !cur.is_empty() {
        lexemes.push(cur.clone());
        cur.clear();
    }
}

fn classify(s: &str) -> TokenKind {
    use TokenKind::*;
    match s {
        "(" => LeftParen,
        ")" => RightParen,
        "{" => LeftBracket,
        "}" => RightBracket,
        "while" => WhileKeyword,
        "return" => ReturnKeyword,
        "=" => Equal,
        "," => Comma,
        ";" => Eol,
        "int"    => VarTypeInt,
        "void"   => VarTypeVoid,
        "+" => BinOpPlus,
        "*" => BinOpMul,
        "==" => BinOpEq,
        "!=" => BinOpNe,
        "%" => BinOpMod,
        _ if s.chars().all(|c| c.is_ascii_digit()) => Number,
        _ => Identifier,
    }
}
```
