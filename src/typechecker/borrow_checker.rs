#[cfg(test)]
pub mod tests;

use crate::ast::*;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowState {
    // not borrowed
    Unborrowed,
    // immutably borrowed with count
    Shared(usize),
    // mutably borrowed
    Exclusive,
    // borrowed by a function call (add this new state)
    FunctionBorrowed(String), // Function name that borrowed it
}

pub struct BorrowChecker {
    borrows: HashMap<String, BorrowState>, // <var_name, borrow_state>
    current_function: Option<String>,      // keep track of current function being analyzed
    debug: bool,                           // debug mode flag
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            borrows: HashMap::new(),
            current_function: None,
            debug: false,
        }
    }

    // enable or disable debug mode
    pub fn set_debug_mode(&mut self, enable: bool) {
        self.debug = enable;
    }

    // print the current borrow state (useful for debugging)
    pub fn print_borrow_state(&self) {
        if !self.debug {
            return;
        }

        println!("Current Borrow State:");
        if self.borrows.is_empty() {
            println!("  [Empty]");
            return;
        }

        for (name, state) in &self.borrows {
            match state {
                BorrowState::Unborrowed => println!("  {} - Unborrowed", name),
                BorrowState::Shared(count) => println!("  {} - Shared {} times", name, count),
                BorrowState::Exclusive => println!("  {} - Exclusively borrowed", name),
                BorrowState::FunctionBorrowed(func) => {
                    println!("  {} - Borrowed by function {}", name, func)
                }
            }
        }
    }

    // set current function name
    pub fn set_current_function(&mut self, name: Option<String>) {
        self.current_function = name;
    }

    // check that structs don't have reference fields
    pub fn check_struct(&self, struct_def: &StructDef) -> Result<()> {
        for field in &struct_def.fields {
            if let Type::Reference(_, _) = field.field_type {
                return Err(anyhow!(
                    "Struct {} cannot have reference field {}. References are not allowed in struct definitions.",
                    struct_def.name,
                    field.name
                ));
            }
        }
        Ok(())
    }

    // check that functions don't return references
    pub fn check_function_return(&self, func: &FunctionDef) -> Result<()> {
        if let Some(Type::Reference(_, _)) = func.return_type {
            return Err(anyhow!(
                "Function {} cannot return a reference. Reference returns are not supported.",
                func.name
            ));
        }
        Ok(())
    }

    // borrow a variable immutably
    pub fn borrow_shared(&mut self, name: &str) -> Result<()> {
        if self.debug {
            println!("Trying to borrow {} immutably", name);
        }

        match self.borrows.get_mut(name) {
            // if unborrowed, mark as shared with count 1
            Some(BorrowState::Unborrowed) => {
                *self.borrows.get_mut(name).unwrap() = BorrowState::Shared(1);
                Ok(())
            }
            // if already shared, increment counter
            Some(BorrowState::Shared(count)) => {
                *count += 1;
                Ok(())
            }
            // if exclusively borrowed, fail!
            Some(BorrowState::Exclusive) => Err(anyhow!(
                "Cannot borrow {} as immutable because it's already borrowed as mutable",
                name
            )),
            // if borrowed by a function, fail!
            Some(BorrowState::FunctionBorrowed(func)) => Err(anyhow!(
                "Cannot borrow {} as immutable because it's already borrowed by function {}",
                name,
                func
            )),
            // if not in map yet, add it as shared
            None => {
                self.borrows
                    .insert(name.to_string(), BorrowState::Shared(1));
                Ok(())
            }
        }
    }

    // borrow a variable mutably
    pub fn borrow_exclusive(&mut self, name: &str) -> Result<()> {
        if self.debug {
            println!("Trying to borrow {} mutably", name);
        }

        match self.borrows.get(name) {
            // if unborrowed, mark as exclusive
            Some(BorrowState::Unborrowed) => {
                self.borrows
                    .insert(name.to_string(), BorrowState::Exclusive);
                Ok(())
            }
            // if already shared or exclusive, fail!
            Some(BorrowState::Shared(_)) => Err(anyhow!(
                "Cannot borrow {} as mutable because it's already borrowed as immutable",
                name
            )),
            Some(BorrowState::Exclusive) => Err(anyhow!(
                "Cannot borrow {} as mutable because it's already borrowed as mutable",
                name
            )),
            // if borrowed by a function, fail!
            Some(BorrowState::FunctionBorrowed(func)) => Err(anyhow!(
                "Cannot borrow {} as mutable because it's already borrowed by function {}",
                name,
                func
            )),
            // if not in map yet, add it as exclusive
            None => {
                self.borrows
                    .insert(name.to_string(), BorrowState::Exclusive);
                Ok(())
            }
        }
    }

    // borrow a variable for a function
    pub fn borrow_for_function(&mut self, name: &str, func_name: &str) -> Result<()> {
        if self.debug {
            println!("Trying to borrow {} for function {}", name, func_name);
        }

        match self.borrows.get(name) {
            // if unborrowed, mark as function borrowed
            Some(BorrowState::Unborrowed) => {
                self.borrows.insert(
                    name.to_string(),
                    BorrowState::FunctionBorrowed(func_name.to_string()),
                );
                Ok(())
            }
            // if already borrowed by any, fail!
            // Some(BorrowState::Shared(_)) => Err(anyhow!(
            //     "Cannot borrow {} for function {} because it's already borrowed immutably",
            //     name,
            //     func_name
            // )),
            Some(BorrowState::Exclusive) => Err(anyhow!(
                "Cannot borrow {} for function {} because it's already borrowed mutably",
                name,
                func_name
            )),
            Some(BorrowState::FunctionBorrowed(other_func)) => Err(anyhow!(
                "Cannot borrow {} for function {} because it's already borrowed by function {}",
                name,
                func_name,
                other_func
            )),
            // if not in map yet, add it as function borrowed
            None => {
                self.borrows.insert(
                    name.to_string(),
                    BorrowState::FunctionBorrowed(func_name.to_string()),
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }

    // check if a variable can be modified
    pub fn can_modify(&self, name: &str) -> Result<()> {
        match self.borrows.get(name) {
            // can't modify if shared or exclusively borrowed
            Some(BorrowState::Shared(_)) => {
                Err(anyhow!("Cannot modify {} while it is borrowed", name))
            }
            Some(BorrowState::Exclusive) => Err(anyhow!(
                "Cannot modify {} while it is mutably borrowed by something else",
                name
            )),
            Some(BorrowState::FunctionBorrowed(func)) => Err(anyhow!(
                "Cannot modify {} while it is borrowed by function {}",
                name,
                func
            )),
            // otherwise OK!
            _ => Ok(()),
        }
    }

    // check reference expression
    pub fn check_reference(&mut self, expr: &Expression, is_mutable: bool) -> Result<()> {
        if let Expression::Variable(name) = expr {
            // borrow appropriately based on mutability
            if is_mutable {
                self.borrow_exclusive(name)?;
            } else {
                self.borrow_shared(name)?;
            }
            Ok(())
        } else {
            // only variables can be referenced
            Err(anyhow!("Can only take references to variables"))
        }
    }

    // checks program for borrow checker violations!!!
    pub fn check_program(&mut self, program: &Program) -> Result<()> {
        // check all structs for reference fields
        for def in &program.definitions {
            if let TopLevelDefinition::Struct(struct_def) = def {
                self.check_struct(struct_def)?;
            }
        }

        // check functions
        for def in &program.definitions {
            if let TopLevelDefinition::Function(func) = def {
                self.check_function_return(func)?;

                // set current function for context
                self.set_current_function(Some(func.name.clone()));

                if self.debug {
                    println!("\nChecking function: {}", func.name);
                }

                // reset borrow state for each function
                self.borrows.clear();

                // check function body
                for stmt in &func.body {
                    self.check_statement(stmt)?;

                    if self.debug {
                        self.print_borrow_state();
                    }
                }

                // clear current function
                self.set_current_function(None);
            }
        }

        Ok(())
    }

    // check a statement for borrow checker violations
    pub fn check_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Throwaway(_, _) => {}
            Statement::VariableDeclaration(decl, _) => {
                self.check_expression(&decl.value)?;
                if let Type::Reference(_, is_mutable) = &decl.var_type {
                    if let Expression::Reference(_, ref_is_mutable) = decl.value {
                        if *is_mutable != ref_is_mutable {
                            return Err(anyhow!("Mismatched reference mutability"));
                        }
                    } else {
                        return Err(anyhow!("Expected reference expression for reference type"));
                    }
                }
            }

            Statement::Assignment(assign, _) => {
                match &assign.target {
                    Expression::Variable(var_name) => {
                        self.can_modify(var_name)?;
                    }
                    Expression::ArrayAccess(array_access) => {
                        self.can_modify(&array_access.array_name)?;
                        self.check_expression(&array_access.index)?;
                    }
                    Expression::StructFieldAccess(field_access) => {
                        match &*field_access.struct_expr {
                            Expression::Variable(var_name) => {
                                self.can_modify(var_name)?;
                            }
                            other => {
                                return Err(anyhow!(
                                    "Invalid assignment target: expected struct field on a variable, found `{:#?}`",
                                    other
                                ));
                            }
                        }
                    }
                    other => {
                        return Err(anyhow!(
                            "Invalid assignment target: expected variable or array access, found `{:#?}`",
                            other
                        ));
                    }
                }
                self.check_expression(&assign.value)?;
            }
            Statement::If(if_stmt, _) => {
                // check the condition
                self.check_expression(&if_stmt.condition)?;

                // check then-block statements
                for stmt in &if_stmt.then_block {
                    self.check_statement(stmt)?;
                }

                // check else-if blocks
                for else_if in &if_stmt.else_if_blocks {
                    self.check_expression(&else_if.condition)?;
                    for stmt in &else_if.body {
                        self.check_statement(stmt)?;
                    }
                }

                // check else block
                if let Some(else_block) = &if_stmt.else_block {
                    for stmt in else_block {
                        self.check_statement(stmt)?;
                    }
                }
            }
            Statement::While(while_stmt, _) => {
                // check the condition
                self.check_expression(&while_stmt.condition)?;

                // check body statements
                for stmt in &while_stmt.body {
                    self.check_statement(stmt)?;
                }
            }
            Statement::FunctionCall(func_call, _) => {
                if self.debug {
                    println!("Checking function call to {}", func_call.name);
                }

                // for each variable argument, mark it as borrowed by this function
                for arg in &func_call.args {
                    if let Expression::Variable(name) = arg {
                        // Borrow the variable for this function call
                        self.borrow_for_function(name, &func_call.name)?;
                    } else {
                        // For non-variable arguments, check them normally
                        self.check_expression(arg)?;
                    }
                }

                if self.debug {
                    println!("Completed function call to {}", func_call.name);
                }
            }
            Statement::Return(ret_stmt, _) => {
                // check return value if any
                if let Some(expr) = &ret_stmt.value {
                    self.check_expression(expr)?;
                }
            }
            // these don't involve expressions or variables
            Statement::Break(_) | Statement::Continue(_) => {}
            _ => {}
        }

        Ok(())
    }

    // check expressions for borrow checker violations
    pub fn check_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Literal(_) => Ok(()),
            Expression::Variable(name) => {
                // check if variable can be read (not exclusively borrowed)
                match self.borrows.get(name) {
                    Some(BorrowState::Exclusive) => {
                        Err(anyhow!("Cannot use {} while it is mutably borrowed", name))
                    }
                    Some(BorrowState::FunctionBorrowed(func)) => {
                        // if we're in the function that borrowed it, allow access
                        if let Some(current_func) = &self.current_function {
                            if current_func == func {
                                return Ok(());
                            }
                        }
                        Err(anyhow!(
                            "Cannot use {} while it is borrowed by function {}",
                            name,
                            func
                        ))
                    }
                    _ => Ok(()),
                }
            }
            Expression::BinaryOp(bin_op) => {
                self.check_expression(&bin_op.left)?;
                self.check_expression(&bin_op.right)?;
                Ok(())
            }
            Expression::UnaryOp(un_op) => {
                self.check_expression(&un_op.operand)?;
                Ok(())
            }
            Expression::FunctionCall(func_call) => {
                // handle function call expression similar to function call statement
                if self.debug {
                    println!("Checking function call expression to {}", func_call.name);
                }

                for arg in &func_call.args {
                    if let Expression::Variable(name) = arg {
                        self.borrow_for_function(name, &func_call.name)?;
                    } else {
                        self.check_expression(arg)?;
                    }
                }

                // similar simplification as in check_statement
                if self.debug {
                    println!("Completed function call expression to {}", func_call.name);
                }

                Ok(())
            }
            Expression::StructInit(struct_init) => {
                for field in &struct_init.fields {
                    self.check_expression(&field.value)?;
                }
                Ok(())
            }
            Expression::StructFieldAccess(field_access) => {
                self.check_expression(&field_access.struct_expr)?;
                Ok(())
            }
            Expression::ArrayAccess(array_access) => {
                // check if array can be read
                match self.borrows.get(&array_access.array_name) {
                    Some(BorrowState::Exclusive) => Err(anyhow!(
                        "Cannot access {} while it is mutably borrowed",
                        array_access.array_name
                    )),
                    Some(BorrowState::FunctionBorrowed(func)) => {
                        // if we're in the function that borrowed it, allow access
                        if let Some(current_func) = &self.current_function {
                            if current_func == func {
                                return Ok(());
                            }
                        }
                        Err(anyhow!(
                            "Cannot access {} while it is borrowed by function {}",
                            array_access.array_name,
                            func
                        ))
                    }
                    _ => {
                        self.check_expression(&array_access.index)?;
                        Ok(())
                    }
                }
            }
            Expression::ArrayLiteral(array_literal) => {
                for elem in &array_literal.elements {
                    self.check_expression(elem)?;
                }
                Ok(())
            }
            Expression::TypeCast(type_cast) => {
                self.check_expression(&type_cast.expression)?;
                Ok(())
            }
            Expression::Reference(expr, is_mutable) => {
                self.check_reference(expr, *is_mutable)?;
                Ok(())
            }
            Expression::Dereference(expr) => {
                // check the underlying expression
                self.check_expression(expr)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
