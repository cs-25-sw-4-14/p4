use crate::{ast::{BinaryOperation, Type}, typechecker::{function_table::Ftable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(binary_operation: &BinaryOperation, vtable: &VTable, ftable: &Ftable) -> Result<Type> {
    let left_type = expression_check::check(&binary_operation.left, vtable, ftable)?;
    let right_type = expression_check::check(&binary_operation.right, vtable, ftable)?;

    if left_type == right_type {
        Ok(left_type)
    } else {
        Err(anyhow::anyhow!("Type mismatch: {:?} {:?} {:?}", left_type, binary_operation.operator, right_type))
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
