pub use interface::interface;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod interface;
mod history;
mod render;
