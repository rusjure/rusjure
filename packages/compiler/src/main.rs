use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use rusjure_ast::{Expression, Program, Term};
use rusjure_compiler::{build_module, build_std};

fn main() {
    let program: Program = vec![
        Expression {
            first: Box::new(Term::Symbol("println".to_string())),
            params: vec![
                Term::String("Hello world!".to_string())
            ]
        }
    ];

    let context = Context::create();
    let module = build_module(&context, program);
    let engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    build_std(&context, &engine);
    unsafe {
        type Addition = unsafe extern "C" fn(i32, i32) -> i32;
        let add: JitFunction<Addition> = engine.get_function("add").unwrap();
        let x = 512;
        let y = 1024;
        assert_eq!(add.call(x, y), x + y);
    }
}
