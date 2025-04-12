
use crate::{ast::{Type, TypeCast}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(type_cast: &TypeCast, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {
    expression_check::check(&type_cast.expression, v_table, f_table, s_table)?;
    Ok(type_cast.target_type.clone())
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{Expression, Literal, TypeCast};
    use crate::typechecker::expression_check;
    use crate::typechecker::struct_table::STable;

    #[test]
    fn check_type_cast_should_return_type() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::TypeCast(
            TypeCast {
                expression: Box::new(Expression::Literal(Literal::Int(4))),
                target_type: Type::I64,
            }
        );
        let result = expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap();
        assert_eq!(result, Type::I64);
    }
}


