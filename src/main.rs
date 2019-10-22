mod span;
mod lexer;
mod error;
mod token;
mod coalesce;
mod parse;
mod node;
mod item;
mod interface;
mod coalescence;
mod context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let argument = std::env::args().nth(1);
	match argument.as_ref().map(String::as_str) {
		Some("-b") | Some("--basic") => interface::basic()?,
		Some("-e") | Some("--evaluate") => interface::evaluate_direct()?,
		_ => if let Err(_) = interface::interface() {
			interface::basic()?;
		},
	}
	Ok(())
}
