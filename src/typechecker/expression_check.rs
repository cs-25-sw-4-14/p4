use crate::ast::{Expression, Type, UnaryOperator};
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
        Expression::BinaryOp(binary_operation) => {
            let left_type = check(&binary_operation.left, vtable, ftable)?;
            let right_type = check(&binary_operation.right, vtable, ftable)?;

            if left_type == right_type {
                Ok(left_type)
            } else {
                Err(anyhow::anyhow!("Type mismatch: {:?} {:?} {:?}", left_type, binary_operation.operator, right_type))
            }
        },
        Expression::UnaryOp(unary_op) => {
            let operand_type = check(&unary_op.operand, vtable, ftable)?;
            match unary_op.operator {
                UnaryOperator::Negate => {
                    match operand_type {
                        Type::I8 | Type::I16 | Type::I32 | Type::I64 |
                        Type::FP8 | Type::FP16 | Type::FP32 | Type::FP64 => Ok(operand_type),
                        _ => Err(anyhow::anyhow!(
                            "Unary '-' not supported for type {:?}",
                            operand_type
                        )),
                    }
                }
                UnaryOperator::Not => {
                    if operand_type == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Err(anyhow::anyhow!(
                            "Unary '!' expects a Bool, but found {:?}",
                            operand_type
                        ))
                    }
                }
            }
        }
        // TODO: Thorbjoern
        Expression::FunctionCall(function_call) => {
            // Check if the function is defined in the function table
            let func = ftable.get(&function_call.name)
                .ok_or_else(|| anyhow::anyhow!("function: {}, not defined", function_call.name))?;

            let return_type = func.def.return_type.clone()
                .ok_or_else(|| anyhow::anyhow!("Function {} has no return type and is therefore not an expression", function_call.name))?;

            // check if the number of arguments is correct
            if func.def.params.len() != function_call.args.len() {
                return Err(anyhow::anyhow!("Function {} called with incorrect number of arguments", function_call.name));
            }

             //check if the types are the same
             for i in 0..func.def.params.len() {
                let expected_type = func.def.params[i].param_type.clone();
                let actual_type = check(&function_call.args[i], vtable, ftable)?;
                 if expected_type != actual_type {
                     return Err(anyhow::anyhow!("Function {} called with incorrect argument types", function_call.name));
                }
             }

            println!("Function {} called with correct argument types", function_call.name);
            Ok(return_type)
        }
        // TODO: Marck
        Expression::StructInit(struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(struct_field_access) => todo!(),
        // TODO: Rasmus
        Expression::ArrayAccess(array_access) => todo!(),
        Expression::ArrayLiteral(array_literal) => {
            if array_literal.elements.is_empty() {
                return Err(anyhow::anyhow!("Array literal is empty"));
            }

            let first_type = check(&array_literal.elements[0], vtable, ftable)?;
            
            for element in &array_literal.elements[1..] {
                let element_type = check(element, vtable, ftable)?;
                if element_type != first_type {
                    return Err(anyhow::anyhow!("Array elements have different types: {:?} and {:?}", first_type, element_type));
                }
            }
            Ok(Type::Array(Box::new(first_type), array_literal.elements.len() as i64))
        },
        Expression::TypeCast(type_cast) => {
            check(&type_cast.expression, vtable, ftable)?;
            Ok(type_cast.target_type.clone())
        },
    }
}

#[cfg(test)]
pub mod literal_tests {
    use super::*;
    use crate::ast::Literal;

    #[test]
    fn check_literal_should_return_type() {
        let tests = vec![
            (Expression::Literal(Literal::Char('b')), Type::Char),
            (Expression::Literal(Literal::String("test".to_string())), Type::String),
            (Expression::Literal(Literal::Bool(true)), Type::Bool),
            (Expression::Literal(Literal::Float(0.1)), Type::FP64),
            (Expression::Literal(Literal::Int(0)), Type::I64),
        ];

        let vtable = VTable::new();
        let ftable = Ftable::new();

        for (expression, expected_type) in tests {
            let result = check(&expression, &vtable, &ftable).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}

#[cfg(test)]
pub mod variable_tests {
    use super::*;
    use crate::ast::{Literal, VariableDeclaration};

    #[test]
    fn check_variable_should_return_type() {
        let mut vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::Variable("x".to_string());

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

#[cfg(test)]
pub mod binary_op_tests {
    use super::*;
    use crate::ast::{BinaryOperation, BinaryOperator, Literal};

    #[test]
    fn check_binary_op_types_match() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::BinaryOp(
            BinaryOperation {
                left: Box::new(Expression::Literal(Literal::Int(4))),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Literal(Literal::Int(0))),
            }
        );

        check(&expression, &vtable, &ftable).unwrap();
    }

    #[test]
    fn check_binary_op_types_not_match() {
        let vtable = VTable::new();
        let ftable = Ftable::new();


        let expression = Expression::BinaryOp(
            BinaryOperation {
                left: Box::new(Expression::Literal(Literal::Int(4))),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Literal(Literal::Float(0.0))),
            }
        );

        check(&expression, &vtable, &ftable).unwrap_err();
    }

}

#[cfg(test)]
pub mod array_literal_tests {
    use super::*;
    use crate::ast::{ArrayLiteral, Literal};

    #[test]
    fn check_array_literal_should_return_type() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::ArrayLiteral(
            Box::new(ArrayLiteral {
                elements: vec![
                    Expression::Literal(Literal::Int(1)),
                    Expression::Literal(Literal::Int(2)),
                    Expression::Literal(Literal::Int(3)),
                ],
            })
        );

        let result = check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::Array(Box::new(Type::I64), 3));
    }

    #[test]
    fn check_array_literal_should_return_type_mixed() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::ArrayLiteral(
            Box::new(ArrayLiteral {
                elements: vec![
                    Expression::Literal(Literal::Int(1)),
                    Expression::Literal(Literal::Float(2.0)),
                    Expression::Literal(Literal::Int(3)),
                ],
            })
        );

        check(&expression, &vtable, &ftable).unwrap_err();
    }

#[test]
    fn check_array_literal_empty() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::ArrayLiteral(
            Box::new(ArrayLiteral {
                elements: vec![],
            })
        );

        check(&expression, &vtable, &ftable).unwrap_err();
    }

}

#[cfg(test)]
pub mod type_cast_tests {

    use super::*;
    use crate::ast::{Literal, TypeCast};

    #[test]
    fn check_type_cast_should_return_type() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::TypeCast(
            TypeCast {
                expression: Box::new(Expression::Literal(Literal::Int(4))),
                target_type: Type::I64,
            }
        );
        let result = check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::I64);
    }
}

#[cfg(test)]
pub mod function_call_tests {
    use super::*;
    use crate::ast::{Expression, FunctionDef, Literal, Param, Type, FunctionCall};
    use crate::lossy::ExpressionParser;
    use crate::typechecker::function_table::FunctionEntry;

    #[test]
    fn check_function_should_err_on_function_not_defined() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let vtable = VTable::new();
        let ftable = Ftable::new();

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_function_should_have_return_type() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let vtable = VTable::new();
        let mut ftable = Ftable::new();

        ftable.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_length() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });
        let vtable = VTable::new();
        let mut ftable = Ftable::new();

        ftable.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![ Param { name: String::from("a"), param_type: Type::String }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_type() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![Expression::Literal(Literal::Int(1))],
        });

        let vtable = VTable::new();
        let mut ftable = Ftable::new();

        ftable.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![ Param { name: String::from("a"), param_type: Type::I32 }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        check(&expression, &vtable, &ftable).unwrap_err();
    }
}

#[cfg(test)]
pub mod unary_tests {
    use super::*;
    use crate::ast::{Expression, Literal, Type, UnaryOperation};

    #[test]
    fn check_unary_op_negate_ok() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::Literal(Literal::Int(5))),
            }
        );

        let vtable = VTable::new();
        let ftable = Ftable::new();

        let result = check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::I64);
    }

    #[test]
    fn check_unary_op_negate_err() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::Literal(Literal::Bool(false))),
            }
        );

        let vtable = VTable::new();
        let ftable = Ftable::new();

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_unary_op_not_ok() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::Literal(Literal::Bool(false))),
            }
        );

        let vtable = VTable::new();
        let ftable = Ftable::new();

        let result = check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::Bool);
    }

    #[test]
    fn check_unary_op_not_err() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::Literal(Literal::Int(30))),
            }
        );

        let vtable = VTable::new();
        let ftable = Ftable::new();

        check(&expression, &vtable, &ftable).unwrap_err();
    }

}

