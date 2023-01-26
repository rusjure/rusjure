use std::ffi::{c_char, CStr, CString};
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use rusjure_lst::Expression;

extern fn std_addition(x: i32, y: i32) -> i32 {
    let res = x + y;
    println!("stdlib: {} + {} = {}", x, y, res);
    res
}

unsafe extern fn std_println(str: *const c_char) {
    unsafe {
        let str = CStr::from_ptr(str);
        let str = str.to_str().unwrap();
        println!("{}", str);
    }
}

pub fn std_module(context: &Context) -> Module {
    let buffer = MemoryBuffer::create_from_memory_range(rusjure_stdlib::BITCODE, "stdlib");
    let module = context.create_module_from_ir(buffer).unwrap();
    module
}

pub fn std_headers<'ctx>(module: &Module<'ctx>, context: &'ctx Context) {
    let void_t = context.void_type();

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

    let str = builder.build_global_string_ptr("Hello world!", "msg");

    let _ = builder.build_call(module.get_function("println").unwrap(), &[str.as_pointer_value().into()], "println");
    builder.build_return(None);
}
