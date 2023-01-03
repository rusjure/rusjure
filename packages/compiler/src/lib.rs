use std::ffi::{c_char, CStr};
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use rusjure_lst::Expression;

extern "C" {
    fn puts(s: *const c_char);
}

extern fn std_addition(x: i32, y: i32) -> i32 {
    let res = x + y;
    println!("std: {} + {} = {}", x, y, res);
    res
}

extern fn std_println(str: *const c_char) {
    unsafe {
        puts(str);
        println!();
    }
}

pub fn build_std(context: &Context, engine: &ExecutionEngine) {
    let module = context.create_module("std");

    let void_t = context.void_type();
    let i32_t = context.i32_type();

    let println_fn_t = void_t.fn_type(&[])
    let println_fn_v = module.add_function("println", println_fn_t, None);

    engine.add_module(&module).expect("TODO: panic message");
    engine.add_global_mapping(&fn_val, std_addition as usize);
}

pub fn build_module<'ctx>(context: &'ctx Context, _code: Expression) -> Module<'ctx> {
    let module_name = "addition";

    let module = context.create_module(module_name);

    let void_t = context.void_type();
    let fn_t = void_t.fn_type(&[], false);

    let fn_v = module.add_function("main", fn_t, None);
    let entry_b = context.append_basic_block(fn_v, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_b);

    let str = context.const_string(b"Hello world!", true);

    let _ = builder.build_call(module.get_function("println").unwrap(), &[str.into()], "println");
    builder.build_return(None);

    module
}
