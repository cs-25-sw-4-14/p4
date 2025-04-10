pub mod expression_check;
pub mod statement_check;

// Re-export common functions for easier access
pub use expression_check::{types_match, can_convert_literal, is_numeric_type, is_integer_type};
