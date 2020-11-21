pub mod interpreter;
pub mod transpiler;
pub mod util;

pub use crate::interpreter::Interpreter;
pub use crate::transpiler::Transpiler;

pub const LOL_EXTENSION: &str = "lol";
pub const LOLC_EXTENSION: &str = "lolc";
