use llvm_sys::bit_reader::LLVMParseBitcode2;
use llvm_sys::core::*;
use llvm_sys::execution_engine::{
    LLVMCreateExecutionEngineForModule, LLVMExecutionEngineRef, LLVMFindFunction, LLVMRunFunction,
};
use llvm_sys::prelude::*;
use rusjure_tokens::{Form, Token, TokenTree};
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

#[allow(dead_code)]
pub struct Jit {
    ctx: LLVMContextRef,
    b: LLVMBuilderRef,
    m: LLVMModuleRef,
    j: LLVMExecutionEngineRef,
    std: LLVMModuleRef,
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
            let std = {
                let mut m = 0x0 as LLVMModuleRef;
                let buf = LLVMCreateMemoryBufferWithMemoryRangeCopy(
                    rusjure_corelib::BITCODE.as_ptr() as *const _,
                    rusjure_corelib::BITCODE.len(),
                    b"rusjure_core\0".as_ptr() as *const _,
                );
                LLVMParseBitcode2(buf, &mut m);
                m
            };
            let j = {
                let mut j = 0x0 as LLVMExecutionEngineRef;
                let mut err = null_mut();

                LLVMCreateExecutionEngineForModule(&mut j, std, &mut err);
                if !err.is_null() {
                    panic!(
                        "Failed to create execution engine: {}",
                        CStr::from_ptr(err).to_string_lossy()
                    );
                }
                j
            };

            Jit {
                ctx,
                b,
                m,
                j,
                std,
                s: BTreeMap::new(),
            }
        }
    }

    pub fn run(&self, tt: &TokenTree) {
        match tt {
            TokenTree::Form(f) => unsafe {
                let Form { quoted, tokens } = f;

                if *quoted {
                    todo!()
                } else {
                    let mut iter = tokens.iter();
                    let first = iter.next().unwrap();
                    let _params = iter;
                    if let TokenTree::Token(Token::Symbol(f)) = first {
                        let f = CString::new(f.as_str()).unwrap();
                        let mut fun = null_mut();
                        LLVMFindFunction(self.j, f.as_ptr(), &mut fun);
                        LLVMRunFunction(self.j, fun, 0, null_mut());
                    }
                }
            },
            TokenTree::Sequence(_) => todo!(),
            TokenTree::Token(_) => todo!(),
        }
    }

    #[allow(dead_code)]
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
