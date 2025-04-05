use std::rc::Rc;

use antlr_rust::tree::ParseTreeVisitorCompat;

use crate::antlr_gen::lzyparser::ProgramContextAll;

pub mod function_table;
pub mod struct_table;
pub mod variable_table;
pub mod expression_checker;

pub struct TypeChecker {}

impl TypeChecker {
    pub fn check(ctx: Rc<ProgramContextAll<'_>>) {
        let mut function_table_visitor = function_table::FunctionTableVisitor::new();
        let mut struct_table_visitor = struct_table::StructTableVisitor::new();

        struct_table_visitor.visit(Rc::clone(&ctx).as_ref());
        function_table_visitor.visit(Rc::clone(&ctx).as_ref());
        println!("Struct Table:\n{}", struct_table_visitor);
        println!("Function Table:\n{}", function_table_visitor);

        let mut expression_checker = expression_checker::ExpressionCheckingVisitor::new(function_table_visitor);
        expression_checker.visit(Rc::clone(&ctx).as_ref());
    }
}
