use llvm_sys::bit_reader::LLVMParseBitcode2;
use llvm_sys::core::*;
use llvm_sys::execution_engine::{
    LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMExecutionEngineRef,
    LLVMFindFunction, LLVMLinkInMCJIT, LLVMRemoveModule, LLVMRunFunction,
};
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::*;
use llvm_sys::target::{
    LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget,
};
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
            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();
            LLVM_InitializeNativeAsmParser();
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

            LLVMLinkModules2(m, std);

            let j = {
                let mut j = 0x0 as LLVMExecutionEngineRef;
                let mut err = null_mut();

                LLVMCreateExecutionEngineForModule(&mut j, m, &mut err);
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
                    let mut params = iter;
                    if let TokenTree::Token(Token::Symbol(f)) = first {
                        match f.as_str() {
                            "defn" => {
                                let fn_name =
                                    CString::new(self.to_string(params.next().unwrap())).unwrap();
                                let fun = LLVMAddFunction(
                                    self.m,
                                    fn_name.as_ptr(),
                                    LLVMFunctionType(
                                        LLVMVoidTypeInContext(self.ctx),
                                        null_mut(),
                                        0,
                                        0,
                                    ),
                                );
                                let block_name = CString::new("entry").unwrap();
                                let block = LLVMAppendBasicBlock(fun, block_name.as_ptr());

                                LLVMPositionBuilderAtEnd(self.b, block);

                                self.build_tt_to_fun(params.next().unwrap());

                                LLVMBuildRetVoid(self.b);
                            }
                            _ => self.jit_llvm_fn(f),
                        }
                    }
                }
            },
            TokenTree::Sequence(_) => todo!(),
            TokenTree::Token(_) => todo!(),
        }
    }

    unsafe fn build_tt_to_fun(&self, tt: &TokenTree) {
        match tt {
            TokenTree::Form(form) => unsafe {
                let call_target = self.to_string(form.tokens.first().unwrap());
                self.build_fun_call(&call_target);
            },
            TokenTree::Sequence(_seq) => todo!(),
            TokenTree::Token(_token) => todo!(),
        }
    }

    unsafe fn build_fun_call(&self, target: &str) -> LLVMValueRef {
        let call_name = CString::new("").unwrap();
        let call_target_name = CString::new(target).unwrap();
        LLVMBuildCall2(
            self.b,
            LLVMFunctionType(LLVMVoidType(), null_mut(), 0, 0),
            LLVMGetNamedFunction(self.m, call_target_name.as_ptr()),
            null_mut(),
            0,
            call_name.as_ptr(),
        )
    }

    unsafe fn jit_llvm_fn(&self, f: &str) {
        unsafe {
            let f = CString::new(f).unwrap();
            let mut fun = null_mut();
            LLVMFindFunction(self.j, f.as_ptr(), &mut fun);
            if fun.is_null() {
                panic!("Function not found: {}", f.to_string_lossy());
            }
            LLVMRunFunction(self.j, fun, 0, null_mut());
        }
    }

    fn to_string(&self, tt: &TokenTree) -> String {
        match tt {
            TokenTree::Form(_) => todo!(),
            TokenTree::Sequence(_) => todo!(),
            TokenTree::Token(token) => match token {
                Token::Symbol(symbol) => symbol.to_string(),
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
            LLVMRemoveModule(self.j, self.m, &mut self.m, null_mut());
            LLVMDisposeExecutionEngine(self.j);
            LLVMDisposeBuilder(self.b);
            LLVMDisposeModule(self.m);
            LLVMContextDispose(self.ctx);
        }
    }
}
