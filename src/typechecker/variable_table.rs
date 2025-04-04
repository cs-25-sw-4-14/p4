use antlr_rust::token::Token; use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

use antlr_rust::tree::ParseTreeVisitorCompat;
use crate::antlr_gen::lzyparser::{DeclarationContext, DeclarationContextExt, LzyParserContextType};
use crate::antlr_gen::lzyvisitor::LzyVisitorCompat;

pub struct VariableTableVisitor<'input, > {
    pub table: HashMap<String, DeclarationContextExt<'input>>,
}

impl Display for VariableTableVisitor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, _) in &self.table {
            writeln!(f, "Variable: {}", name)?;
        }
        Ok(())
    }
}

impl<'i> VariableTableVisitor<'i> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for VariableTableVisitor<'input> {
    type Node = LzyParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::new(()))
    }
}

impl<'input> LzyVisitorCompat<'input> for VariableTableVisitor<'input> {
    fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) -> Self::Return {
        let name = ctx.id.clone().unwrap().get_text().to_string();
        self.table.insert(name, ctx.deref().clone());
    }
}


