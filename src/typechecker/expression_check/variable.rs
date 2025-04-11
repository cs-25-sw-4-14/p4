use crate::{ast::Type, typechecker::variable_table::VTable};
use anyhow::Result;

pub fn check(name: &str, vtable: &VTable) -> Result<Type> {
    if let Some(var) = vtable.get(name) {
        Ok(var.var_type.clone())
    } else {
        Err(anyhow::anyhow!("Variable {} not found in vtable", name))
    }
}

#[cfg(test)]
pub mod variable_tests {
    use super::*;
    use crate::{ast::{Expression, Literal, VariableDeclaration}, typechecker::function_table::Ftable};
    use crate::typechecker::expression_check;

    #[test]
    fn check_variable_should_return_type() {
        let mut vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::Variable("x".to_string());

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();

        vtable.variables.insert(
            "x".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type::I64,
                name: "x".to_string(),
                value: Expression::Literal(Literal::Int(4)),
            },
        );

        expression_check::check(&expression, &vtable, &ftable).unwrap();
    }

}
