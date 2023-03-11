use inkwell::context::Context;

pub struct ExecEngine {
    context: Context,
}

impl ExecEngine {
    pub fn new() -> Self {
        let context = Context::create();

        Self {
            context,
        }
    }
}
