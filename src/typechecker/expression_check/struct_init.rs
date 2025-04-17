use crate::{ast::{StructInit, Type}, typechecker::{function_table::FTable, variable_table::VTable, struct_table::STable}};
use anyhow::Result;
use crate::typechecker::expression_check;
use crate::ast::StructDef;

pub fn check(struct_init: &StructInit, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {

    let struct_definition = semantic_check_struct_defined(struct_init, s_table)?;
    semantic_check_field_types(struct_init, struct_definition, v_table, f_table, s_table)?;
    
    Ok(Type::Struct(struct_definition.name.clone()))
}

pub fn semantic_check_struct_defined<'a>(struct_init: &'a StructInit, s_table: &'a STable) -> Result<&'a StructDef> {
    let struct_definition = s_table.get(&struct_init.name)
        .ok_or_else(|| {anyhow::anyhow!("struct: {}, not defined", struct_init.name)})?;

    Ok(struct_definition)
}

pub fn semantic_check_field_types<'a>(struct_init: &'a StructInit, struct_definition: &'a StructDef, v_table: &'a VTable, f_table: &'a FTable, s_table: &'a STable) -> Result<()> {

    for field in 0..struct_definition.fields.len() {
        let expected_type = struct_definition.fields[field].field_type.clone();
        let actual_type = expression_check::check(&struct_init.fields[field].value, v_table, f_table, s_table)?;

        if expected_type != actual_type {
            return Err(anyhow::anyhow!("Struct {} initialized with incorrect field types", struct_definition.name));
        }
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{Expression, Literal, StructDef, StructField, StructFieldInit};
    use crate::typechecker::expression_check;

    #[test]
    fn check_struct_init_should_fail_if_not_in_s_table() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::StructInit(
            StructInit {
                name: "Test".to_string(),
                fields: vec![
                    StructFieldInit { name: String::from("ey"), value: Expression::Literal(Literal::Int(4)) },
                ],
            }
        );

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }

    #[test]
    fn check_struct_init_should_be_ok_if_in_s_table() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let mut s_table = STable::new();

        let expression = Expression::StructInit(
            StructInit {
                name: "Test".to_string(),
                fields: vec![
                    StructFieldInit { name: String::from("ey"), value: Expression::Literal(Literal::Int(4)) },
                ],
            }
        );

        s_table.structs.insert(
            "Test".to_string(),
            StructDef {
                name: "Test".to_string(),
                fields: vec![
                    StructField { name: String::from("ey"), field_type: Type::I64 },
                ],
            },
        );

        let returned_type = expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap();

        assert_eq!(returned_type, Type::Struct("Test".to_string()));
        
    }
}


