use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;

pub fn std_module(context: &Context) -> Module {
    let buffer = MemoryBuffer::create_from_memory_range(rusjure_stdlib::BITCODE, "stdlib");
    let module = context.create_module_from_ir(buffer).unwrap();
    module
}

pub fn build_module<'ctx>(module: &Module<'ctx>, context: &'ctx Context) {
    // let void_t = context.void_type();
    let i64_t = context.i64_type();
    let fn_t = i64_t.fn_type(&[i64_t.into(), i64_t.into()], false);

    let fn_v = module.add_function("main", fn_t, None);
    let entry_b = context.append_basic_block(fn_v, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_b);

    // let str = builder.build_global_string_ptr("Hello world!", "msg");

    // let _ = builder.build_call(module.get_function("println").unwrap(), &[str.as_pointer_value().into()], "println");
    let x = fn_v.get_nth_param(0).unwrap();
    let y = fn_v.get_nth_param(1).unwrap();

    let res = builder.build_call(module.get_function("addition").unwrap(), &[x.into(), y.into()], "add_nums");
    builder.build_return(Some(&res.try_as_basic_value().unwrap_left()));
}
