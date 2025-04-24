```rust
use std::{env, fs};
use common::{Lex, TokenKind};

struct Parser {
    tokens: Vec<Lex>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Lex>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Lex> {
        self.tokens.get(self.pos)
    }

    fn eat(&mut self, expected: TokenKind) {
        if let Some(tok) = self.peek() {
            if tok.kind == expected {
                self.pos += 1;
                return;
            } else {
                error(&format!(
                    "In grammar rule expecting {:?}, token #{} expected {:?} but was {:?}",
                    expected,
                    self.pos + 1,
                    expected,
                    tok.kind
                ));
            }
        }
        error("Unexpected end of tokens");
    }

    fn parse(&mut self) {
        self.parse_function();
        if self.pos != self.tokens.len() {
            error(&format!(
                "Only consumed {} of the {} given tokens",
                self.pos,
                self.tokens.len()
            ));
        }
    }

    fn parse_function(&mut self) {
        self.parse_header();
        self.parse_body();
    }

    fn parse_header(&mut self) {
        // header --> VARTYPE IDENTIFIER LEFT_PAREN [arg-decl] RIGHT_PAREN
        match self.peek() {
            Some(l) if matches!(l.kind, TokenKind::VarTypeInt | TokenKind::VarTypeVoid) => {
                self.pos += 1
            }
            _ => error("Error: In grammar rule header, expected VARTYPE"),
        }
        self.eat(TokenKind::Identifier);
        self.eat(TokenKind::LeftParen);
        if let Some(l) = self.peek() {
            if matches!(l.kind, TokenKind::VarTypeInt | TokenKind::VarTypeVoid) {
                self.parse_arg_decl();
            }
        }
        self.eat(TokenKind::RightParen);
    }

    fn parse_arg_decl(&mut self) {
        // arg-decl --> VARTYPE IDENTIFIER {COMMA VARTYPE IDENTIFIER}
        loop {
            match self.peek() {
                Some(l) if matches!(l.kind, TokenKind::VarTypeInt | TokenKind::VarTypeVoid) => {
                    self.pos += 1
                }
                _ => error("Error: In grammar rule arg-decl, expected VARTYPE"),
            }
            self.eat(TokenKind::Identifier);
            if let Some(l) = self.peek() {
                if l.kind == TokenKind::Comma {
                    self.eat(TokenKind::Comma);
                    continue;
                }
            }
            break;
        }
    }

    fn parse_body(&mut self) {
        // body --> LEFT_BRACKET [statement-list] RIGHT_BRACKET
        self.eat(TokenKind::LeftBracket);
        if let Some(l) = self.peek() {
            if l.kind != TokenKind::RightBracket {
                self.parse_statement_list();
            }
        }
        self.eat(TokenKind::RightBracket);
    }

    fn parse_statement_list(&mut self) {
        // statement-list --> statement {statement}
        self.parse_statement();
        while let Some(l) = self.peek() {
            if matches!(l.kind, TokenKind::WhileKeyword | TokenKind::ReturnKeyword | TokenKind::Identifier) {
                self.parse_statement();
            } else {
                break;
            }
        }
    }

    fn parse_statement(&mut self) {
        match self.peek() {
            Some(l) if l.kind == TokenKind::WhileKeyword => self.parse_while_loop(),
            Some(l) if l.kind == TokenKind::ReturnKeyword => self.parse_return(),
            Some(l) if l.kind == TokenKind::Identifier => self.parse_assignment(),
            _ => error("Error: In grammar rule statement, expected statement non-terminal"),
        }
    }

    fn parse_while_loop(&mut self) {
        // while-loop --> WHILE_KEYWORD LEFT_PAREN expression RIGHT_PAREN body
        self.eat(TokenKind::WhileKeyword);
        self.eat(TokenKind::LeftParen);
        self.parse_expression();
        self.eat(TokenKind::RightParen);
        self.parse_body();
    }

    fn parse_return(&mut self) {
        // return --> RETURN_KEYWORD expression EOL
        self.eat(TokenKind::ReturnKeyword);
        self.parse_expression();
        self.eat(TokenKind::Eol);
    }

    fn parse_assignment(&mut self) {
        // assignment --> IDENTIFIER EQUAL expression EOL
        self.eat(TokenKind::Identifier);
        self.eat(TokenKind::Equal);
        self.parse_expression();
        self.eat(TokenKind::Eol);
    }

    fn parse_expression(&mut self) {
        // expression --> term {BINOP term} | LEFT_PARENTHESIS expression RIGHT_PARENTHESIS
        if let Some(l) = self.peek() {
            if l.kind == TokenKind::LeftParen {
                self.eat(TokenKind::LeftParen);
                self.parse_expression();
                self.eat(TokenKind::RightParen);
                return;
            }
        }
        self.parse_term();
        while let Some(l) = self.peek() {
            match l.kind {
                TokenKind::BinOpPlus
                | TokenKind::BinOpMul
                | TokenKind::BinOpEq
                | TokenKind::BinOpNe
                | TokenKind::BinOpMod => {
                    self.pos += 1;
                    self.parse_term();
                }
                _ => break,
            }
        }
    }

    fn parse_term(&mut self) {
        // term --> IDENTIFIER | NUMBER
        match self.peek() {
            Some(l) if l.kind == TokenKind::Identifier => self.eat(TokenKind::Identifier),
            Some(l) if l.kind == TokenKind::Number => self.eat(TokenKind::Number),
            _ => error("Error: In grammar rule term, expected IDENTIFIER or NUMBER"),
        }
    }
}

fn error(msg: &str) {
    println!("Error: {}", msg);
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: recognizer <input> <output>");
        std::process::exit(1);
    }
    let data = fs::read_to_string(&args[1]).expect("Failed to read input");
    let tokens: Vec<Lex> = data
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, ' ');
            let tk = parts.next().unwrap();
            let lx = parts.next().unwrap_or("");
            let kind = match tk {
                "LEFT_PARENTHESIS" => TokenKind::LeftParen,
                "RIGHT_PARENTHESIS" => TokenKind::RightParen,
                "LEFT_BRACKET" => TokenKind::LeftBracket,
                "RIGHT_BRACKET" => TokenKind::RightBracket,
                "WHILE_KEYWORD" => TokenKind::WhileKeyword,
                "RETURN_KEYWORD" => TokenKind::ReturnKeyword,
                "EQUAL" => TokenKind::Equal,
                "COMMA" => TokenKind::Comma,
                "EOL" => TokenKind::Eol,
                "VARTYPE" => {
                    if lx == "void" {
                        TokenKind::VarTypeVoid
                    } else {
                        TokenKind::VarTypeInt
                    }
                }
                "IDENTIFIER" => TokenKind::Identifier,
                "BINOP" => match lx {
                    "+" => TokenKind::BinOpPlus,
                    "*" => TokenKind::BinOpMul,
                    "==" => TokenKind::BinOpEq,
                    "!=" => TokenKind::BinOpNe,
                    "%" => TokenKind::BinOpMod,
                    _ => error("Unknown BINOP"),
                },
                "NUMBER" => TokenKind::Number,
                _ => error("Unknown token kind"),
            };
            Lex { kind, lexeme: lx.to_string() }
        })
        .collect();

    let mut parser = Parser::new(tokens);
    parser.parse();
    fs::write(&args[2], "PARSED!!!\n").expect("Failed to write output");
}
```
