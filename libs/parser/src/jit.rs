use llvm_sys::core::*;
use llvm_sys::prelude::*;
use rusjure_tokens::{Form, Token, TokenTree};
use std::collections::BTreeMap;
use std::io::BufWriter;
use std::io::Write;

#[allow(dead_code)]
pub struct Jit {
    ctx: LLVMContextRef,
    b: LLVMBuilderRef,
    m: LLVMModuleRef,
    s: BTreeMap<String, ()>,
}

impl Default for Jit {
    fn default() -> Self {
        Self::new()
    }
}

impl Jit {
    pub fn new() -> Jit {
        unsafe {
            let ctx = LLVMContextCreate();
            let b = LLVMCreateBuilderInContext(ctx);
            let m = LLVMModuleCreateWithNameInContext(b"rusjure\0".as_ptr() as *const _, ctx);

            Jit {
                ctx,
                b,
                m,
                s: BTreeMap::new(),
            }
        }
    }

    pub fn run(&self, tt: &TokenTree) {
        match tt {
            TokenTree::Form(f) => {
                let Form { quoted, tokens } = f;

                if *quoted {
                    todo!()
                } else {
                    let mut iter = tokens.iter();
                    let first = iter.next().unwrap();
                    let params = iter;
                    if let TokenTree::Token(Token::Symbol(f)) = first {
                        if f == "println" {
                            let mut buf = BufWriter::new(std::io::stdout());
                            for p in params {
                                write!(buf, "{}", self.to_string(p)).unwrap();
                            }
                            writeln!(buf).unwrap();
                        }
                    }
                }
            }
            TokenTree::Sequence(_) => todo!(),
            TokenTree::Token(_) => todo!(),
        }
    }

    fn to_string(&self, tt: &TokenTree) -> String {
        match tt {
            TokenTree::Form(_) => todo!(),
            TokenTree::Sequence(_) => todo!(),
            TokenTree::Token(token) => match token {
                Token::Symbol(_) => todo!(),
                Token::String(str) => str.to_string(),
                Token::Number(_) => todo!(),
                Token::Float(_) => todo!(),
            },
        }
    }
}

impl Drop for Jit {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.ctx);
        }
    }
}
