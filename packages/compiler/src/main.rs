use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use rusjure_compiler::{build_module, std_headers, link_std};

fn main() {
    // let program: Program = vec![
    //     Expression {
    //         first: Box::new(Term::Symbol("println".to_string())),
    //         params: vec![
    //             Term::String("Hello world!".to_string())
    //         ]
    //     }
    // ];

    let context = Context::create();
    let module = context.create_module("main");
    std_headers(&module, &context);
    build_module(&module, &context);
    let engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    link_std(&module, &engine);
    unsafe {
        type MainFn = unsafe extern "C" fn() -> ();
        let main: JitFunction<MainFn> = engine.get_function("main").unwrap();
        main.call();
    }
}
