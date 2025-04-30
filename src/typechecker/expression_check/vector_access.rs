

use anyhow::Result;

pub fn check(
    vector_access: &VectorAccess,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,

) -> Result<Type> {
//check if vector is defined and has type vector
if let Some(var) = vtable.get(&vector_access.vector_name) {
    Ok(var.var_type.clone())
} else {
    Err(anyhow::anyhow!("Variable {} not found in vtable", name))
}

//check if index is int 
//check if index is in bounds

}


#cfg[(test)]
pub mod vector_access_tests {
    use super::*;
    use crate::ast::{Expression, Literal};
    use crate::typechecker::expression_check;

    #[test]
    fn test_vector_access() {

        v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type::Vector(Box::new(Type::I64)),
                name: "my_vector".to_string(),
                value: Expression::VectorLiteral(Box::new(VectorLiteral {
                    elements: vec![Expression::Literal(Literal::Int(0))],
                })),
            },
        );
       

        let expression = Expression::VectorAccess(VectorAccess {
            vector_name: "my_vector".to_string(),
            index: Expression::Literal(Literal::Int(0)),
        });

        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let result = expression_check::check(&expression, &v_table, &f_table, &s_table);
        assert!(result.is_ok());
    }
}