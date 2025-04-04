use antlr_rust::token::Token; use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

use antlr_rust::tree::ParseTreeVisitorCompat;
use crate::antlr_gen::lzyvisitor::LzyVisitorCompat;
use crate::antlr_gen::lzyparser::{LzyParserContextType, StructDefinitionContext, StructDefinitionContextExt};

pub struct StructTableVisitor<'input, > {
    pub table: HashMap<String, StructDefinitionContextExt<'input>>,
}

impl Display for StructTableVisitor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, _) in &self.table {
            writeln!(f, "Struct: {}", name)?;
        }
        Ok(())
    }
}

impl<'i> StructTableVisitor<'i> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for StructTableVisitor<'input> {
    type Node = LzyParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::new(()))
    }
}

impl<'input> LzyVisitorCompat<'input> for StructTableVisitor<'input> {
    fn visit_structDefinition(&mut self, ctx: &StructDefinitionContext<'input>) -> Self::Return {
        let name = ctx.id.clone().unwrap().get_text().to_string();
        self.table.insert(name, ctx.deref().clone());
    }
}


