use antlr_rust::token::Token; use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

use antlr_rust::tree::ParseTreeVisitorCompat;
use crate::antlr_gen::lzyvisitor::LzyVisitorCompat;
use crate::antlr_gen::lzyparser::{FunctionContext, FunctionContextExt, LzyParserContextType};


pub struct FunctionEntry<'i> {
    pub ctx: FunctionContextExt<'i>,
}

pub struct FunctionTableVisitor<'i> {
    pub table: HashMap<String,FunctionContextExt<'i>>,
}

impl Display for FunctionTableVisitor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, _) in &self.table {
            writeln!(f, "Function: {}", name)?;
        }
        Ok(())
    }
}

impl<'i> FunctionTableVisitor<'i> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for FunctionTableVisitor<'input> {
    type Node = LzyParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::new(()))
    }
}

impl<'input> LzyVisitorCompat<'input> for FunctionTableVisitor<'input> {
    fn visit_function(&mut self, ctx: &FunctionContext<'input>) -> Self::Return {
        let name = ctx.id.clone().unwrap().get_text().to_string();
        self.table.insert(name, ctx.deref().clone());
    }
}

