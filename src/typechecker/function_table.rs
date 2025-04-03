use antlr_rust::token::Token; use std::collections::HashMap;
use std::fmt::Display;

use antlr_rust::tree::ParseTreeVisitorCompat;
use crate::antlr_gen::lzyvisitor::LzyVisitorCompat;
use crate::{FunctionContextExt, LzyParserContextType};
use crate::antlr_gen::lzyparser::FunctionContext;

pub struct FunctionTableVisitor<'i> {
    pub table: HashMap<String,FunctionContextExt<'i>>,
    current_function: Option<String>,
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
            current_function: None,
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

        self.table.insert(
            name.clone(),
            (*ctx).clone()
        );
        self.current_function = Some(name);
    }
}

