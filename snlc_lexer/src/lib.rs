//! Low Level SNLC Lexer

mod cursor;

pub use crate::cursor::Cursor;

use crate::cursor::EOF_CHAR;
use crate::LiteralKind::*;
use crate::TokenKind::*;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

// Enum representing common lexeme types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Comment  `{` comment inner `}`
    Comment { terminated: bool },
    // Any whitespace character sequence.
    Whitespace,
    Ident,
    Literal { kind: LiteralKind },

    // Single character tokens
    OpenParen,    // (
    CloseParen,   // )
    OpenBracket,  // [
    CloseBracket, // ]

    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Semicolon, // ;
    Dot,       // .
    Comma,     // ,
    Colon,     // :

    // Single or double characters tokens
    Less,       // <
    LessEq,     // <=
    Eq,         // =
    Assign,     // :=
    UnderRange, // ..

    Unknown,
    Eof,
}

// Enum representing the literal types supported by the lexer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Integer,
    Char { terminated: bool },
}

pub fn tokensize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
    // This is XID_Start OR '_' (which formally is not a XID_Start).
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

/// The passed string is lexically an identifier.
pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(start) = chars.next() {
        is_id_start(start) && chars.all(is_id_continue)
    } else {
        false
    }
}

impl Cursor<'_> {
    /// Prases a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let token_kind = match first_char {
            // comment
            '{' => self.comment(),
            // whitespace
            c if is_whitespace(c) => self.whitespace(),
            // identifier
            c if is_id_start(c) => self.ident(),
            // assign
            ':' => match self.first() {
                '=' => {
                    self.bump();
                    Assign
                }
                _ => Colon,
            },
            // Less LessEq
            '<' => match self.first() {
                '=' => {
                    self.bump();
                    LessEq
                }
                _ => Less,
            },
            // Dot or UnderRange
            '.' => match self.first() {
                '.' => {
                    self.bump();
                    UnderRange
                }
                _ => Dot,
            },
            // Eq

            // literal
            // int literal
            c if c.is_ascii_digit() => self.int_or_unknown(),
            // char literal
            '\'' => self.char_or_string(),
            // Single character token
            '=' => Eq,
            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenBracket,
            ']' => CloseBracket,

            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            ';' => Semicolon,
            ',' => Comma,
            _ => Unknown,
        };

        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }
    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn comment(&mut self) -> TokenKind {
        self.eat_while(|c| c != '}');

        let last_char = self.bump().unwrap_or(EOF_CHAR);
        let is_terminated = matches!(last_char, '}');

        Comment {
            terminated: is_terminated,
        }
    }

    fn ident(&mut self) -> TokenKind {
        self.eat_while(is_id_continue);
        Ident
    }

    fn char_or_string(&mut self) -> TokenKind {
        match (self.first(), self.second()) {
            (_, '\'') => {
                // eat literal char
                self.bump();
                // eat single quoted
                self.bump();
                Literal {
                    kind: Char { terminated: true },
                }
            }
            (_, _) => {
                // eat literal char
                self.bump();
                Literal {
                    kind: Char { terminated: false },
                }
            }
        }
    }

    fn int_or_unknown(&mut self) -> TokenKind {
        self.eat_while(|c| c.is_ascii_digit());
        Literal { kind: Integer }
    }
}
