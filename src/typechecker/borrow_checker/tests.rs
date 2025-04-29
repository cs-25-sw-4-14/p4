use super::*;
use crate::ast::{FunctionDef, StructField};

#[test]
fn test_struct_with_references() {
    let checker = BorrowChecker::new();

    // create a struct with no reference fields - should be OK
    let valid_struct = StructDef {
        name: "ValidStruct".to_string(),
        fields: vec![
            StructField {
                name: "field1".to_string(),
                field_type: Type::I64,
            },
            StructField {
                name: "field2".to_string(),
                field_type: Type::Bool,
            },
        ],
    };

    assert!(checker.check_struct(&valid_struct).is_ok());

    // create a struct with a reference field - should fail
    let invalid_struct = StructDef {
        name: "InvalidStruct".to_string(),
        fields: vec![
            StructField {
                name: "field1".to_string(),
                field_type: Type::I64,
            },
            StructField {
                name: "ref_field".to_string(),
                field_type: Type::Reference(Box::new(Type::I64), false),
            },
        ],
    };

    assert!(checker.check_struct(&invalid_struct).is_err());
}

#[test]
fn test_function_return_reference() {
    let checker = BorrowChecker::new();

    // function that doesn't return a reference - should be OK
    let valid_func = FunctionDef {
        name: "valid_func".to_string(),
        return_type: Some(Type::I64),
        params: vec![],
        body: vec![],
    };

    assert!(checker.check_function_return(&valid_func).is_ok());

    // function that returns a reference - should fail
    let invalid_func = FunctionDef {
        name: "invalid_func".to_string(),
        return_type: Some(Type::Reference(Box::new(Type::I64), false)),
        params: vec![],
        body: vec![],
    };

    assert!(checker.check_function_return(&invalid_func).is_err());
}

#[test]
fn test_borrow_shared() {
    let mut checker = BorrowChecker::new();

    // first shared borrow should succeed
    assert!(checker.borrow_shared("x").is_ok());

    // second shared borrow should also succeed
    assert!(checker.borrow_shared("x").is_ok());

    // verify the counter is 2
    match checker.borrows.get("x") {
        Some(BorrowState::Shared(count)) => assert_eq!(*count, 2),
        _ => panic!("Expected shared borrow state with count 2"),
    }

    // exclusive borrow after shared borrows should fail
    assert!(checker.borrow_exclusive("x").is_err());
}

#[test]
fn test_check_reference() {
    let mut checker = BorrowChecker::new();

    // can take immutable reference to variable
    let var_expr = Expression::Variable("x".to_string());
    assert!(checker.check_reference(&var_expr, false).is_ok());

    // can't take mutable reference after immutable reference
    assert!(checker.check_reference(&var_expr, true).is_err());
}

#[test]
fn test_function_call_borrows() {
    let mut checker = BorrowChecker::new();
    checker.set_debug_mode(true);

    // set up a variable in the borrow state
    checker
        .borrows
        .insert("a".to_string(), BorrowState::Unborrowed);

    // create function calls to borrow1 and borrow2
    let func_call1 = FunctionCall {
        name: "borrow1".to_string(),
        args: vec![Expression::Variable("a".to_string())],
    };

    let func_call2 = FunctionCall {
        name: "borrow2".to_string(),
        args: vec![Expression::Variable("a".to_string())],
    };

    // first function call should succeed
    let stmt1 = Statement::FunctionCall(func_call1, 0);
    assert!(checker.check_statement(&stmt1).is_ok());

    // at this point 'a' should be borrowed by borrow1
    match checker.borrows.get("a") {
        Some(BorrowState::FunctionBorrowed(func)) => {
            assert_eq!(func, "borrow1");
        }
        _ => panic!("Expected 'a' to be borrowed by borrow1"),
    }

    // second function call should fail because 'a' is already borrowed
    let stmt2 = Statement::FunctionCall(func_call2, 0);
    assert!(checker.check_statement(&stmt2).is_err());

    // print the current borrow state
    checker.print_borrow_state();
}
