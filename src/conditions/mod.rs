mod parser;
mod ast;
pub mod gen_condition;  // теперь публичный

pub use parser::Parser;
pub use gen_condition::generate_condition;