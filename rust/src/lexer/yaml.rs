use lexer::{TokenSource, Lexer, Token, CharUtil, Type};

enum YamlState {
    Begin, Val
}

struct YamlLexer<'a> {
    state: YamlState,
    src: &'a mut TokenSource<'a> + 'a
}

impl<'a> YamlLexer<'a> {
    fn new(src: &'a mut TokenSource) -> YamlLexer<'a> {
        YamlLexer { src: src, state: Begin }
    }

    fn next_key(&mut self) -> Option<Token> {
        let mut string = String::new();
        let hi = self.src.pos();
        let mut lo = self.src.pos();
        loop {
            let c = match self.src.peak() {
                Some(chr) => chr,
                None => return None
            };

            if c.is_alpha() || c == ':' {
                string.push(c);
                self.src.next_char(); // consume peaked char
                if c == ':' {
                    break;
                }
                lo += 1;
            } else {
                break;
            }
        }
        self.state = YamlState::Val;
        Some(Token {
            hi: hi,
            lo: lo,
            ty: Type::Key,
            src: string
        })
    }

    fn next_val(&mut self) -> Option<Token> {
        None
    }
}

impl<'a> Lexer for YamlLexer<'a> {
    fn next_token(&mut self) -> Option<Token> {
        loop {
            let c = match self.src.next_char() {
                Some(chr) => chr,
                None => return None
            };

            match self.state {
                Begin   => return self.next_key(),
                Val     => return self.next_val()
            }
        }
    }


}
