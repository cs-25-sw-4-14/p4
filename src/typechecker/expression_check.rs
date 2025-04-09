use crate::ast::{Expression, Type};
use super::{function_table::Ftable, variable_table::VTable};
use anyhow::Result;

pub fn check(expression: &Expression, vtable: &VTable, ftable: &Ftable) -> Result<Type> {

    match expression {
        Expression::Literal(literal) => Ok(Type::from(literal.clone())),
        Expression::Variable(name) => {
            if let Some(var) = vtable.get(name) {
                Ok(var.var_type.clone())
            } else {
                Err(anyhow::anyhow!("Variable {} not found in vtable", name))
            }
        }
        // TODO: Mads
        Expression::BinaryOp(binary_operation) => todo!(),
        // TODO: Kristoffer
        Expression::UnaryOp(unary_operation) => todo!(),
        // TODO: Thorbjoern
        Expression::FunctionCall(function_call) => todo!(),
        // TODO: Marck
        Expression::StructInit(struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(struct_field_access) => todo!(),
        // TODO: Rasmus
        Expression::ArrayAccess(array_access) => todo!(),
        // TODO: Mads
        Expression::ArrayLiteral(array_literal) => todo!(),
        // TODO: Mads
        Expression::TypeCast(type_cast) => todo!(),
    }
}

#[cfg(test)]
pub mod literal_tests {
    use super::*;
    use crate::lossy::ExpressionParser;

    #[test]
    fn check_literal_should_return_type() {
        let tests = vec![
            ("'r'", Type::Char),
            ("\"test\"", Type::String),
            ("true", Type::Bool),
            ("false", Type::Bool),
            ("4", Type::I64),
            ("4.2", Type::FP64),
        ];

        let vtable = VTable::new();
        let ftable = Ftable::new();

        for (input, expected_type) in tests {
            let expression = ExpressionParser::new()
                .parse(input)
                .expect("Failed to parse: ");


            let result = check(&expression, &vtable, &ftable).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}

#[cfg(test)]
pub mod variable_tests {
    use super::*;
    use crate::{ast::{Literal, VariableDeclaration}, lossy::ExpressionParser};

    #[test]
    fn check_variable_should_return_type() {
        let mut vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = ExpressionParser::new()
            .parse("x")
            .expect("Failed to parse: ");

        check(&expression, &vtable, &ftable).unwrap_err();

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

        check(&expression, &vtable, &ftable).unwrap();
    }

}


