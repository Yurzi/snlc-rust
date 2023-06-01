use snlc_lexer::Cursor;
use snlc_lexer;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: u32,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident,
    Keyword,
    Delimiter,
    Literal,
    Whitespace,
    BinOp,
    Invisiable,
}

impl Token {
    pub fn new(kind: TokenKind, pos: u32, lexeme: String) -> Token {
        Token {
            kind,
            pos,
            lexeme,
        }
    }

    pub fn to_str(tokens: Vec<Token>) -> String {
        let mut res = String::new();
        for token in tokens {
            res.push_str(&token.lexeme);
        }
        res
    }

    pub fn from_str(src: &str) -> Vec<Self> {
        let mut cursor = Cursor::new(src);
        let mut curr_pos: usize = 0;
        let mut tokens = Vec::new();
        loop {
            let lex_token = cursor.advance_token();
            if lex_token.kind == snlc_lexer::TokenKind::Eof {
                break;
            }
            // if is comment skip
            match lex_token.kind {
                snlc_lexer::TokenKind::Comment { terminated: _ } => {
                    curr_pos += lex_token.len as usize;
                    continue;
                }
                _ => {}
            }
            // convert
            let lexeme = src[curr_pos..curr_pos + lex_token.len as usize].to_string();
            let mut kind = Token::from_lex_tokenkind(lex_token.kind);
            if kind == TokenKind::Ident && is_keyword(lexeme.as_str()) {
                kind = TokenKind::Keyword;
            }
            let token = Token::new(kind, curr_pos as u32, lexeme);
            tokens.push(token);
            curr_pos += lex_token.len as usize;
        }

        tokens
    }

    fn from_lex_tokenkind(lex_kind: snlc_lexer::TokenKind) -> TokenKind {
        let res = match lex_kind {
            // if is a operator
            snlc_lexer::TokenKind::Plus => TokenKind::BinOp,
            snlc_lexer::TokenKind::Minus => TokenKind::BinOp,
            snlc_lexer::TokenKind::Star => TokenKind::BinOp,
            snlc_lexer::TokenKind::Slash => TokenKind::BinOp,
            snlc_lexer::TokenKind::Less => TokenKind::BinOp,
            snlc_lexer::TokenKind::LessEq => TokenKind::BinOp,
            snlc_lexer::TokenKind::Eq => TokenKind::BinOp,
            snlc_lexer::TokenKind::Assign => TokenKind::BinOp,
            // if is a delimiter
            snlc_lexer::TokenKind::OpenParen => TokenKind::Delimiter,
            snlc_lexer::TokenKind::CloseParen => TokenKind::Delimiter,
            snlc_lexer::TokenKind::OpenBracket => TokenKind::Delimiter,
            snlc_lexer::TokenKind::CloseBracket => TokenKind::Delimiter,
            snlc_lexer::TokenKind::Semicolon => TokenKind::Delimiter,
            snlc_lexer::TokenKind::Dot => TokenKind::Delimiter,
            snlc_lexer::TokenKind::Comma => TokenKind::Delimiter,
            snlc_lexer::TokenKind::Colon => TokenKind::Delimiter,
            snlc_lexer::TokenKind::UnderRange => TokenKind::Delimiter,
            // if is a literal
            snlc_lexer::TokenKind::Literal { kind: _ } => TokenKind::Literal,
            // if is a whitespace
            snlc_lexer::TokenKind::Whitespace => TokenKind::Whitespace,
            // if is a ident
            snlc_lexer::TokenKind::Ident => TokenKind::Ident,
            _ => TokenKind::Invisiable,
        };
        res
    }
}

fn is_keyword(lexeme: &str) -> bool {
    let res = match lexeme {
        "program" => true,
        "begin" => true,
        "end" => true,
        "procedure" => true,
        "return" => true,
        "type" => true,
        "var" => true,
        "if" => true,
        "then" => true,
        "else" => true,
        "fi" => true,
        "while" => true,
        "do" => true,
        "endwh" => true,
        "char" => true,
        "integer" => true,
        "record" => true,
        "array" => true,
        "of" => true,
        "read" => true,
        "write" => true,
        _ => false,
    };
    res
}