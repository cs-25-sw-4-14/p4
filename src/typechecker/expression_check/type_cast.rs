use crate::ast::{Type, TypeCast};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_type_cast(&self, type_cast: &TypeCast) -> Result<Type> {
        self.check_expression(&type_cast.expression)?;
        Ok(type_cast.target_type.clone())
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::{
        ast::{Expression, Literal, TypeCast},
        debugger::Debugger,
    };

    #[test]
    fn check_type_cast_should_return_type() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let typecast = TypeCast {
            expression: Box::new(Expression::Literal(Literal::Int(4))),
            target_type: Type::I64,
        };
        let result = typechecker.check_type_cast(&typecast).unwrap();
        assert_eq!(result, Type::I64);
    }
}
