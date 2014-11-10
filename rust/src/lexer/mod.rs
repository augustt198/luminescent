use lexer;

pub mod yaml;

pub trait Lexer {
    fn next_token(&mut self) -> Option<Token>;
}

pub struct Token {
    hi: uint,
    lo: uint,
    ty: Type,
    src: String
}

enum Type {
    LParen,     // (
    RParen,     // )

    LBrace,     // {
    RBrace,     // }

    LBracket,   // [
    RBracket,   // ]

    Plus,       // +
    Min,        // -
    Mult,       // *
    Div,        // /

    Colon,      // :
    Semi,       // ;

    StringLit,  // "..."
    Integer,    // [0-9]+
    Decimal,    // [0-9]{0,}.[0-9]+
    Boolean,    // true|false
    Keyword,    // varies
    Comment,    // varies

    Key,
    Value,

    Invalid
}

trait TokenSource<'a> {
    fn next_char(&mut self) -> Option<char>;
    fn peak(&self) -> Option<char>;
    fn pos(&self) -> uint;
}

struct StringReader {
    src: &'static str,
    pos: uint
}

impl<'a> TokenSource<'a> for StringReader {
    fn peak(&self) -> Option<char> {
        if self.pos < self.src.len() {
            Some(self.src.char_at(self.pos))
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let peak = self.peak();
        if peak.is_some() { self.pos += 1 }
        peak
    }

    fn pos(&self) -> uint { self.pos }
}

trait CharUtil {
    fn is_alpha(self) -> bool;
    fn is_digit(self) -> bool;
}

impl CharUtil for char {
    fn is_alpha(self) -> bool {
        (self >= 'a' && self <= 'z') || (self >= 'A' && self <= 'Z')
    }
    fn is_digit(self) -> bool {
        self >= '0' && self <= '9'
    }
}
