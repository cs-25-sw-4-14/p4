use crate::{ast::Type, typechecker::Typechecker};
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_variable(&self, name: &str) -> Result<Type> {
        if let Some(var) = self.v_table.get(name) {
            Ok(var.variable_declaration.var_type.clone())
        } else {
            Err(anyhow::anyhow!("Variable {} not found in vtable", name))
        }
    }
}

#[cfg(test)]
pub mod variable_tests {
    use super::*;
    use crate::ast::{Expression, Literal, VariableDeclaration};
    use crate::debugger::Debugger;
    use crate::typechecker::Typechecker;
    use crate::typechecker::variable_table::Inner;

    #[test]
    fn check_variable_should_return_type() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::Variable("x".to_string());

        typechecker.v_table.variables.insert(
            "x".to_string(),
            Inner {
                variable_declaration: VariableDeclaration {
                    trivial: None,
                    mutable: None,
                    var_type: Type::I64,
                    name: "x".to_string(),
                    value: Expression::Literal(Literal::Int(4)),
                },
                immut_count: 0,
                mut_active: false,
            },
        );

        typechecker.check_expression(&expression).unwrap();
    }
}
