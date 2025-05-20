use crate::ast::{StructInit, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_struct_init(&mut self, struct_init: &StructInit) -> Result<Type> {
        // Step 1: Get struct definition and clone needed info
        let struct_definition = self
            .s_table
            .get(&struct_init.name)
            .ok_or_else(|| anyhow::anyhow!("struct: {}, not defined", struct_init.name))?;

        let struct_name = struct_definition.name.clone();
        let expected_fields = struct_definition
            .fields
            .iter()
            .map(|f| f.field_type.clone())
            .collect::<Vec<_>>();

        // Step 2: Check field types against expected
        if expected_fields.len() != struct_init.fields.len() {
            return Err(anyhow::anyhow!(
                "Struct {} initialized with incorrect number of fields (expected {}, got {})",
                struct_name,
                expected_fields.len(),
                struct_init.fields.len()
            ));
        }

        for (i, field_init) in struct_init.fields.iter().enumerate() {
            let actual_type = self.check_expression(&field_init.value)?;
            let expected_type = &expected_fields[i];
            if actual_type != *expected_type && actual_type != Type::Null {
                return Err(anyhow::anyhow!(
                    "Struct {} initialized with incorrect type for field '{}': expected {:?}, got {:?}",
                    struct_name,
                    field_init.name,
                    expected_type,
                    actual_type
                ));
            }
        }

        Ok(Type::Custom(struct_name))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{Expression, Literal, StructDef, StructField, StructFieldInit};
    use crate::debugger::Debugger;

    #[test]
    fn check_struct_init_should_fail_if_not_in_s_table() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::StructInit(StructInit {
            name: "Test".to_string(),
            fields: vec![StructFieldInit {
                name: String::from("ey"),
                value: Expression::Literal(Literal::Int(4)),
            }],
        });

        typechecker.check_expression(&expression).unwrap_err();
    }

    #[test]
    fn check_struct_init_should_be_ok_if_in_s_table() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::StructInit(StructInit {
            name: "Test".to_string(),
            fields: vec![StructFieldInit {
                name: String::from("ey"),
                value: Expression::Literal(Literal::Int(4)),
            }],
        });

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

        assert_eq!(returned_type, Type::Custom("Test".to_string()));
    }
}
