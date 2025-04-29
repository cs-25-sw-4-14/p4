
use crate::{
    ast::{VectorLiteral, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use crate::typechecker::expression_check;
use anyhow::Result;

pub fn check( 
    vector_literal: &Box<VectorLiteral>,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,) 
    -> Result<Type> {

        let vector_type = expression_check::check(
            &vector_literal.elements[0],
            v_table,
            f_table,
            s_table,
        )?;
        Ok(vector_type)

    

}