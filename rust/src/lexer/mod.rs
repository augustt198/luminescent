use lexer;

pub mod yaml;

pub trait Lexer {
    fn next_token(&self) -> Option<Token>;
}

pub struct Token {
    hi: uint,
    lo: uint,
    ty: Type
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
    Keyword,    // varies
    Comment,    // varies

    Key,
    Value
}

trait TokenSource {
    fn next_char(&mut self) -> Option<char>;
    fn peak(&self) -> Option<char>;
    fn pos(&self) -> uint;
}

struct StringReader {
    src: &'static str,
    pos: uint
}

impl TokenSource for StringReader {
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
