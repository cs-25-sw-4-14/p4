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
}

pub struct BorrowChecker {
    borrows: HashMap<String, BorrowState>, // <var_name, borrow_state>
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            borrows: HashMap::new(),
        }
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
            // if not in map yet, add it as exclusive
            None => {
                self.borrows
                    .insert(name.to_string(), BorrowState::Exclusive);
                Ok(())
            }
        }
    }

    // release an immutable borrow
    pub fn release_shared(&mut self, name: &str) -> Result<()> {
        match self.borrows.get_mut(name) {
            Some(BorrowState::Shared(1)) => {
                self.borrows
                    .insert(name.to_string(), BorrowState::Unborrowed);
                Ok(())
            }
            Some(BorrowState::Shared(count)) => {
                *count -= 1;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    // release a mutable borrow
    pub fn release_exclusive(&mut self, name: &str) -> Result<()> {
        match self.borrows.get(name) {
            Some(BorrowState::Exclusive) => {
                self.borrows
                    .insert(name.to_string(), BorrowState::Unborrowed);
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

                // reset borrow state for each function
                self.borrows.clear();

                // check function body
                for stmt in &func.body {
                    self.check_statement(stmt)?;
                }
            }
        }

        Ok(())
    }

    // check a statement for borrow checker violations
    pub fn check_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VariableDeclaration(decl, _) => {
                // check initialization expression
                self.check_expression(&decl.value)?;

                // check if it's a reference type
                if let Type::Reference(_, is_mutable) = &decl.var_type {
                    // check that the expression is actually a reference
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
                // check if the variable can be modified
                self.can_modify(&assign.target)?;

                // check the expression
                self.check_expression(&assign.value)?;
            }
            Statement::ArrayAssignment(assign, _) => {
                // check if the array can be modified
                self.can_modify(&assign.target.array_name)?;

                // check the index expression
                self.check_expression(&assign.target.index)?;

                // check the value expression
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
                // check arguments
                for arg in &func_call.args {
                    self.check_expression(arg)?;
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
                for arg in &func_call.args {
                    self.check_expression(arg)?;
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
                // Check the underlying expression
                self.check_expression(expr)?;
                Ok(())
            }
        }
    }
}
