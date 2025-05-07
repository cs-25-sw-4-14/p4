use crate::ast::StructDef;
use crate::ast::{StructFieldAccess, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_struct_field_access(
        &'a self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<Type> {
        let _struct_definition = self.semantic_check_struct_defined(struct_field_access)?;
        let resulting_type = self.semantic_check_field_access_type(struct_field_access)?;

        Ok(resulting_type)
    }

    pub fn semantic_check_struct_defined(
        &'a self,
        struct_field_access: &'a StructFieldAccess,
    ) -> Result<&'a StructDef> {
        let expr_type = self.check_expression(&struct_field_access.struct_expr)?;

        // Check if the expression type is a struct
        let Type::Custom(name) = expr_type else {
            return Err(anyhow::anyhow!(
                "struct field access must be on a struct type"
            ));
        };

        // Check if the struct is defined in the symbol table
        let struct_definition = self
            .s_table
            .structs
            .get(&name)
            .ok_or_else(|| anyhow::anyhow!("struct: {}, not defined", name))?;

        Ok(struct_definition)
    }

    pub fn semantic_check_field_access_type(
        &'a self,
        struct_field_access: &'a StructFieldAccess,
    ) -> Result<Type> {
        let struct_definition = self.semantic_check_struct_defined(struct_field_access)?;

        // Check if the field is defined in the struct
        let field_type = struct_definition
            .fields
            .iter()
            .find(|field| field.name == struct_field_access.field_name)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "field: {}, not defined in struct: {}",
                    struct_field_access.field_name,
                    struct_definition.name
                )
            })?
            .field_type
            .clone();

        Ok(field_type)
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{
        Expression, Literal, StructDef, StructField, StructFieldInit, StructInit,
        VariableDeclaration,
    };
    use crate::debugger::Debugger;

    #[test]
    fn check_struct_field_access_should_be_ok_if_in_s_table() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::StructFieldAccess(StructFieldAccess {
            struct_expr: Box::new(Expression::Variable("test".to_string())),
            field_name: "ey".to_string(),
        });

        typechecker.v_table.variables.insert(
            "test".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type::Custom("Test".to_string()),
                name: "test".to_string(),
                value: Expression::StructInit(StructInit {
                    name: "Test".to_string(),
                    fields: vec![StructFieldInit {
                        name: String::from("ey"),
                        value: Expression::Literal(Literal::Int(4)),
                    }],
                }),
            },
        );

        typechecker.s_table.structs.insert(
            "Test".to_string(),
            StructDef {
                name: "Test".to_string(),
                fields: vec![StructField {
                    name: String::from("ey"),
                    field_type: Type::I64,
                }],
            },
        );

        let returned_type = typechecker.check_expression(&expression).unwrap();

        assert_eq!(returned_type, Type::I64);
    }
}
