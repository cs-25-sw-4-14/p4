use super::CodeGenerator;

impl<'ctx> CodeGenerator<'ctx> {
    // Set the current function being generated
    pub fn set_current_function(&mut self, function: inkwell::values::FunctionValue<'ctx>) {
        self.current_function = Some(function);
    }

    // Get the current break target, if any
    pub fn get_break_target(&self) -> Option<inkwell::basic_block::BasicBlock<'ctx>> {
        self.break_target_stack.last().copied()
    }

    // Get the current continue target, if any
    pub fn get_continue_target(&self) -> Option<inkwell::basic_block::BasicBlock<'ctx>> {
        self.continue_target_stack.last().copied()
    }

    // This function should be called at the start of your compile_function method
    pub fn enter_function(&mut self, function: inkwell::values::FunctionValue<'ctx>) {
        self.current_function = Some(function);
        self.break_target_stack.clear();
        self.continue_target_stack.clear();
    }

    // This function should be called at the end of your compile_function method
    pub fn exit_function(&mut self) {
        self.current_function = None;
        self.break_target_stack.clear();
        self.continue_target_stack.clear();
    }
}
