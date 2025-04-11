use crate::{ast::{StructInit, Type}, typechecker::{function_table::FTable, variable_table::VTable, struct_table::STable}};
use anyhow::Result;

pub fn check(struct_init: &StructInit, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {

    let struct_definition = s_table.get(&struct_init.name)
        .ok_or_else(|| anyhow::anyhow!("struct: {}, not defined", struct_init.name))?;

    // match struct_definition {
    //
    // }
    //
    Ok(Type::FP32)

    
    // expression_check::check(&type_cast.expression, vtable, ftable)?;
    // Ok(type_cast.target_type.clone())
}

// #[cfg(test)]
// pub mod tests {
//
//     use super::*;
//     use crate::ast::{Expression, Literal, TypeCast};
//     use crate::typechecker::expression_check;
//
//     #[test]
//     fn check_type_cast_should_return_type() {
//         let vtable = VTable::new();
//         let ftable = Ftable::new();
//
//         let expression = Expression::TypeCast(
//             TypeCast {
//                 expression: Box::new(Expression::Literal(Literal::Int(4))),
//                 target_type: Type::I64,
//             }
//         );
//         let result = expression_check::check(&expression, &vtable, &ftable).unwrap();
//         assert_eq!(result, Type::I64);
//     }
// }
//

