use super::CodeGenerator;

impl<'ctx> CodeGenerator<'ctx> {
    /// Sets the current LLVM function being generated.
    /// This is important to know where new blocks and instructions should be inserted.
    pub fn set_current_function(&mut self, function: inkwell::values::FunctionValue<'ctx>) {
        self.current_function = Some(function);
    }

    /// Returns the current 'break' target block, if any.
    ///
    /// This is the block where `break` statements should jump to,
    /// typically the block right after a loop or switch statement.
    pub fn get_break_target(&self) -> Option<inkwell::basic_block::BasicBlock<'ctx>> {
        self.break_target_stack.last().copied()
    }

    /// Returns the current 'continue' target block, if any.
    ///
    /// This is the block where `continue` statements should jump to,
    /// typically the start or condition check block of a loop.
    pub fn get_continue_target(&self) -> Option<inkwell::basic_block::BasicBlock<'ctx>> {
        self.continue_target_stack.last().copied()
    }

    /// Call when entering a new function during code generation.
    ///
    /// Sets the current function context and clears:
    /// - the variable symbol table (since variables are scoped per function)
    /// - break and continue stacks (no active loops at function start)
    pub fn enter_function(&mut self, function: inkwell::values::FunctionValue<'ctx>) {
        self.current_function = Some(function);
        self.variables.clear();
        self.break_target_stack.clear();
        self.continue_target_stack.clear();
    }

    /// Call when exiting a function context.
    ///
    /// Clears current function and all related state to avoid stale data.
    pub fn exit_function(&mut self) {
        self.current_function = None;
        self.variables.clear();
        self.break_target_stack.clear();
        self.continue_target_stack.clear();
    }
}
