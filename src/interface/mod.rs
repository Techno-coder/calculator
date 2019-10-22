pub use basic::{basic, evaluate, evaluate_direct};
pub use interface::interface;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub const PROMPT: &str = ">> ";

mod interface;
mod history;
mod render;
mod check;
mod basic;
