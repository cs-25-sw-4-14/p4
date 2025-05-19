use crate::ast::BaseType;
use crate::ast::{StructFieldAccess, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_struct_field_access(
        &self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<Type> {
        let access_type = self.check_expression(&struct_field_access.struct_expr)?;
        let field_name = struct_field_access.field.clone();

        Ok(match access_type.base {
            BaseType::Custom(name) => {
                if let Some(struct_def) = self.s_table.structs.get(&name) {
                    let field = struct_def.fields.iter().find(|f| f.name == field_name);

                    match field {
                        Some(field) => field.field_type.clone(),
                        None => {
                            return Err(anyhow::anyhow!(
                                "Field {} not found in struct {}",
                                field_name,
                                name
                            ));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Struct {} not found", name));
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Struct field access on non-struct type: {:?}",
                    access_type
                ));
            }
        })
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::ast::{
        Expression, Literal, Ownership, StructDef, StructField, StructFieldInit, StructInit,
        VariableDeclaration,
    };
    use crate::debugger::Debugger;

    #[test]
    fn check_struct_field_access_should_be_ok_if_in_s_table() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::StructFieldAccess(StructFieldAccess {
            struct_expr: Box::new(Expression::Variable("test".to_string())),
            field: "ey".to_string(),
        });

        typechecker.v_table.variables.insert(
            "test".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type {
                    base: BaseType::Custom("Test".to_string()),
                    ownership: Ownership::Owned,
                },
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
                    field_type: Type {
                        base: BaseType::I64,
                        ownership: Ownership::Owned,
                    },
                }],
            },
        );

        let returned_type = typechecker.check_expression(&expression).unwrap();

        assert_eq!(
            returned_type,
            Type {
                base: BaseType::I64,
                ownership: Ownership::Owned,
            }
        );
    }
}
