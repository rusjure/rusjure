use inkwell::context::Context;
use inkwell::module::Module;
use rusjure_ast::Program;

pub fn build_std(context: &Context) -> Module {
    let module = context.create_module("std");

    let i32_t = context.i32_type();
    let fn_t = i32_t.fn_type(&[i32_t.into(), i32_t.into()], false);

    let fn_val = module.add_function("+", fn_t, None);
    let entry_basic_block = context.append_basic_block(fn_val, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_basic_block);

    let x = fn_val.get_nth_param(0).unwrap().into_int_value();
    let y = fn_val.get_nth_param(1).unwrap().into_int_value();
    let ret = builder.build_int_add(x, y, "add");
    let return_instruction = builder.build_return(Some(&ret));

    module
}

pub fn build_module(context: &Context, code: Program) -> Module {
    let module_name = "addition";

    let module = context.create_module(module_name);

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);

    let fn_val = module.add_function("add", fn_type, None);
    let entry_basic_block = context.append_basic_block(fn_val, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_basic_block);

    let x = fn_val.get_nth_param(0).unwrap().into_int_value();
    let y = fn_val.get_nth_param(1).unwrap().into_int_value();

    let add_t = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    module.add_function("+", add_t, None);

    let ret = builder.build_call(module.get_function("+").unwrap(), &[x.into(), y.into()], "addition")
        .try_as_basic_value().unwrap_left();
    let return_instruction = builder.build_return(Some(&ret));

    module
}
