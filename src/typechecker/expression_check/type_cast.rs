
use crate::{ast::{Type, TypeCast}, typechecker::{function_table::Ftable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(type_cast: &TypeCast, vtable: &VTable, ftable: &Ftable) -> Result<Type> {
    expression_check::check(&type_cast.expression, vtable, ftable)?;
    Ok(type_cast.target_type.clone())
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{Expression, Literal, TypeCast};
    use crate::typechecker::expression_check;

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
        let result = expression_check::check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::I64);
    }
}


