mod jit;

#[derive(Default)]
pub struct Parser {
    jit: jit::Jit,
}

impl Parser {
    pub fn run(&self, tt: &rusjure_tokens::TokenTree) {
        self.jit.run(tt);
    }
}
