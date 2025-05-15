use crate::ast::StructDef;
use crate::ast::{StructInit, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_struct_init(&self, struct_init: &StructInit) -> Result<Type> {
        let struct_definition = self.semantic_check_struct_defined_init(struct_init)?;
        self.semantic_check_field_types(struct_init, struct_definition)?;

        Ok(Type::Custom(struct_definition.name.clone()))
    }

    pub fn semantic_check_struct_defined_init(
        &self,
        struct_init: &StructInit,
    ) -> Result<&StructDef> {
        let struct_definition = self
            .s_table
            .get(&struct_init.name)
            .ok_or_else(|| anyhow::anyhow!("struct: {}, not defined", struct_init.name))?;

        Ok(struct_definition)
    }

    pub fn semantic_check_field_types(
        &self,
        struct_init: &StructInit,
        struct_definition: &StructDef,
    ) -> Result<()> {
        for field in 0..struct_definition.fields.len() {
            let expected_type = struct_definition.fields[field].field_type.clone();
            let actual_type = self.check_expression(&struct_init.fields[field].value)?;

            if expected_type != actual_type && actual_type != Type::Null {
                return Err(anyhow::anyhow!(
                    "Struct {} initialized with incorrect field types: {:?} expected, but found {:?}",
                    struct_definition.name,
                    expected_type,
                    actual_type
                ));
            }
        }

        Ok(())
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
        let typechecker = Typechecker::new(&debugger);

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
