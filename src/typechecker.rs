use std::rc::Rc;

use antlr_rust::tree::ParseTreeVisitorCompat;

use crate::antlr_gen::lzyparser::ProgramContextAll;

pub mod function_table;
pub mod struct_table;
pub mod variable_table;

pub struct TypeChecker {}

impl TypeChecker {
    pub fn check(ctx: Rc<ProgramContextAll<'_>>) {
        let mut function_table_visitor = function_table::FunctionTableVisitor::new();
        let mut struct_table_visitor = struct_table::StructTableVisitor::new();

        struct_table_visitor.visit(Rc::clone(&ctx).as_ref());
        function_table_visitor.visit(Rc::clone(&ctx).as_ref());
        println!("Struct Table:\n{}", struct_table_visitor);
        println!("Function Table:\n{}", function_table_visitor);

        // TODO: Visit all function bodies to collect variable declarations
        
        // for (name, ctx) in &function_table_visitor.table {
        //     let mut variable_table_visitor = variable_table::VariableTableVisitor::new();
        //     ctx.get_children();
        //     variable_table_visitor.visit(ctx);
        //     println!("Function {} has variables:\n{}", name, variable_table_visitor);
        // }

        
    }
}
