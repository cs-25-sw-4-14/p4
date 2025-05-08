<<<<<<< HEAD
use crate::typechecker::expression_check;
use crate::{
    ast::{VariableDeclaration, Type, Expression},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
    
};
=======
use crate::ast::VariableDeclaration;
use crate::typechecker::Typechecker;
>>>>>>> main
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration,
        l: usize,
    ) -> Result<()> {
        // Check if the type of the value matches the declared type
        let value_type = self.check_expression(&declaration.value)?;
        if value_type != declaration.var_type {
            return Err(anyhow::anyhow!(
                "{}: Type mismatch: expected {:?}, found {:?}",
                self.debugger.display(l),
                declaration.var_type,
                value_type
            ));
        }

        self.semantic_check_variable_already_declared_variable_declaration(declaration)?;

        self.v_table
            .variables
            .insert(declaration.name.clone(), declaration.clone());

        Ok(())
    }
<<<<<<< HEAD
    semantic_check_variable_already_declared(declaration, v_table)?;
    
    
    match &declaration.value {
        Expression::VectorLiteral(args)  => {
            let length= args.elements.len() as u64;

            let var_type = match &declaration.var_type {
                Type::Vector(t, _) => Type::Vector(t.clone(), length),
                _ => unreachable!(),
            };

        

            let vec_decl = VariableDeclaration {
                trivial: None,
                mutable: declaration.mutable,
                var_type,
                name: declaration.name.clone(),
                value: declaration.value.clone(),
            };
            v_table
                .variables
                .insert(vec_decl.name.clone(), vec_decl);
            return Ok(())
            

        }
        _ => {
            v_table
            .variables
            .insert(declaration.name.clone(), declaration.clone());
    
            Ok(())
        }
    }

    


   
}

pub fn semantic_check_variable_already_declared<'a>(
    declaration: &'a VariableDeclaration,
    v_table: &'a VTable,
) -> Result<()> {
    // Check if the variable is already declared in the current scope
    if v_table.variables.contains_key(&declaration.name) {
        return Err(anyhow::anyhow!(
            "Variable {} is already declared",
            declaration.name
        ));
=======

    pub fn semantic_check_variable_already_declared_variable_declaration(
        &self,
        declaration: &VariableDeclaration,
    ) -> Result<()> {
        // Check if the variable is already declared in the current scope
        if self.v_table.variables.contains_key(&declaration.name) {
            return Err(anyhow::anyhow!(
                "Variable {} is already declared",
                declaration.name
            ));
        }
        Ok(())
>>>>>>> main
    }
}
