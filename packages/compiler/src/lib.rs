use std::ffi::{c_char, CStr, CString};
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use rusjure_lst::Expression;

extern fn std_addition(x: i32, y: i32) -> i32 {
    let res = x + y;
    println!("std: {} + {} = {}", x, y, res);
    res
}

unsafe extern fn std_println(str: *mut i8) {
    unsafe {
        println!("Foo");
        let str = CStr::from_ptr(str);
        println!("Bar");
        let str = str.to_str().unwrap();
        println!("Baz");
        println!("{}", str);
        println!("Xd");
        println!();
    }
}

pub fn std_headers<'ctx>(module: &Module<'ctx>, context: &'ctx Context) {
    let void_t = context.void_type();
    let i32_t = context.i32_type();

    let i8_t = context.i8_type();

    let ptr_i8_t = i8_t.ptr_type(AddressSpace::default());

    let println_fn_t = void_t.fn_type(&[ptr_i8_t.into()], false);
    let _ = module.add_function("println", println_fn_t, None);
}

pub fn link_std<'ctx>(module: &Module<'ctx>, engine: &ExecutionEngine) {
    let println_fn_v = module.get_function("println").unwrap();
    engine.add_global_mapping(&println_fn_v, std_println as usize);
}

pub fn build_module<'ctx>(module: &Module<'ctx>, context: &'ctx Context) {
    let void_t = context.void_type();
    let fn_t = void_t.fn_type(&[], false);

    let fn_v = module.add_function("main", fn_t, None);
    let entry_b = context.append_basic_block(fn_v, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_b);

    let str = context.const_string(b"Hello world!\n", true);

    let _ = builder.build_call(module.get_function("println").unwrap(), &[str.into()], "println");
    builder.build_return(None);
}
