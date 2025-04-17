use crate::{ast::{StructFieldAccess, Type}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;
use crate::ast::StructDef;
pub fn check(struct_field_access: &StructFieldAccess, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {

    let _ = semantic_check_struct_defined(struct_field_access, v_table,f_table,s_table)?;
    let resulting_type = semantic_check_field_access_type(struct_field_access, v_table, f_table, s_table)?;

    Ok(resulting_type)
}

pub fn semantic_check_struct_defined<'a>(struct_field_access: &'a StructFieldAccess, v_table: &'a VTable, f_table: &'a FTable, s_table: &'a STable) -> Result<&'a StructDef> {
    let expr_type = expression_check::check(&struct_field_access.struct_expr, v_table, f_table, s_table)?;

    // Check if the expression type is a struct
    let Type::Struct(name) = expr_type else {
        return Err(anyhow::anyhow!("struct field access must be on a struct type"));
    };

    // Check if the struct is defined in the symbol table
    let struct_definition = s_table.structs.get(&name)
        .ok_or_else(|| {anyhow::anyhow!("struct: {}, not defined", name)})?;

    Ok(struct_definition)
}

pub fn semantic_check_field_access_type<'a>(struct_field_access: &'a StructFieldAccess, v_table: &'a VTable, f_table: &'a FTable, s_table: &'a STable) -> Result<Type> {
    let struct_definition = semantic_check_struct_defined(struct_field_access, v_table, f_table, s_table)?;

    // Check if the field is defined in the struct
    let field_type = struct_definition.fields.iter()
        .find(|field| field.name == struct_field_access.field_name)
        .ok_or_else(|| {anyhow::anyhow!("field: {}, not defined in struct: {}", struct_field_access.field_name, struct_definition.name)})?
        .field_type
        .clone();

    Ok(field_type)
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

