use crate::{ast::{StructFieldAccess, Type}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(struct_field_access: &StructFieldAccess, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {

    let expr_type = expression_check::check(&struct_field_access.struct_expr, v_table, f_table, s_table)?;

    let struct_definition = match expr_type {
        Type::Struct(name) => {
            match s_table.structs.get(&name) {
                Some(struct_type) => struct_type,
                None => {
                    return Err(anyhow::anyhow!("struct: {}, not defined", name));
                }
            }
        }
        _ => {
            return Err(anyhow::anyhow!("cannot access field of non-struct type"));
        }
    };

    let resulting_type = struct_definition.fields
        .iter()
        .find(|field| field.name == struct_field_access.field_name)
        .ok_or(anyhow::anyhow!(
            "Field `{}` not found in struct `{}`",
            struct_field_access.field_name,
            struct_definition.name
        ))?
        .field_type
        .clone();

    Ok(resulting_type)
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{Expression, Literal, StructDef, StructField, StructFieldInit, StructInit, VariableDeclaration};
    use crate::typechecker::expression_check;

    #[test]
    fn check_struct_field_access_should_be_ok_if_in_s_table() {
        let mut v_table = VTable::new();
        let f_table = FTable::new();
        let mut s_table = STable::new();

        let expression = Expression::StructFieldAccess(
            StructFieldAccess {
                struct_expr: Box::new(Expression::Variable("test".to_string())),
                field_name: "ey".to_string(),
            }
        );

        v_table.variables.insert(
            "test".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type::Struct("Test".to_string()),
                name: "test".to_string(),
                value: Expression::StructInit(
                    StructInit {
                        name: "Test".to_string(),
                        fields: vec![
                            StructFieldInit {
                                name: String::from("ey"),
                                value: Expression::Literal(Literal::Int(4)),
                            },
                        ],
                    }
                ),

            }
        );

        s_table.structs.insert(
            "Test".to_string(),
            StructDef {
                name: "Test".to_string(),
                fields: vec![
                    StructField { name: String::from("ey"), field_type: Type::I64 },
                ],
            }
        );

        let returned_type = expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap();

        assert_eq!(returned_type, Type::I64);
    }

}

