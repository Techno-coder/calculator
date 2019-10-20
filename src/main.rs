mod span;
mod lexer;
mod error;
mod token;
mod coalesce;
mod parse;
mod node;
mod interface;
mod coalescence;
mod context;

fn main() {
	println!("Hello, world!");
	if let Err(error) = interface::interface() {
		eprintln!("Error: {}", error);
	}
}
