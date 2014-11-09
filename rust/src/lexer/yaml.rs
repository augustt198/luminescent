use lexer::{TokenSource, Lexer, Token};

enum YamlState {
    Begin, Val
}

struct YamlLexer {
    state: YamlState,
    src: &'static TokenSource + 'static
}

impl YamlLexer {
    fn new(src: &'static TokenSource + 'static) -> YamlLexer {
        YamlLexer { src: src, state: Begin }
    }
}

impl Lexer for YamlLexer {
    fn next_token(&self) -> Option<Token> {
        None
    }
}
