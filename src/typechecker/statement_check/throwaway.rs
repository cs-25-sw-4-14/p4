use crate::typechecker::expression_check;
use crate::{
    ast::Throwaway,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    throwaway: &Throwaway,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    _debugger: &crate::debugger::Debugger,
    l: usize,
) -> Result<()> {
    let expr = expression_check::check(&throwaway.expression, v_table, f_table, s_table);
    if expr.is_err() {
        return Err(anyhow::anyhow!(
            "{}: Error in thrown away expression: {:?}",
            _debugger.display(l),
            expr
        ));
    }
    Ok(())
}
