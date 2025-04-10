// Re-export main typechecker
mod typechecker;
pub use typechecker::Typechecker;

// Include submodules
pub mod tables;
pub mod checks;
