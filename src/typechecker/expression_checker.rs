use antlr_rust::tree::ParseTreeVisitorCompat;
use crate::antlr_gen::lzyparser::LzyParserContextType;
use crate::antlr_gen::lzyvisitor::LzyVisitorCompat;
use super::function_table::FunctionTableVisitor;

pub struct ExpressionCheckingVisitor<'input, > {
    pub function_visitor: FunctionTableVisitor<'input>,
}

impl<'i> ExpressionCheckingVisitor<'i> {
    pub fn new(function_visitor: FunctionTableVisitor<'i>) -> Self {
        Self {function_visitor}
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for ExpressionCheckingVisitor<'input> {
    type Node = LzyParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::new(()))
    }
}

impl<'input> LzyVisitorCompat<'input> for ExpressionCheckingVisitor<'input> {
    fn visit_expr(&mut self,_ctx: &crate::antlr_gen::lzyparser::ExprContext<'input>) -> Self::Return {
        // TODO: Check if the expression is valid
        println!("Checking expression...");
    }
}


