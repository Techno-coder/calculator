use std::fmt;

#[derive(Copy, Clone)]
pub struct Span(pub usize, pub usize);

impl Span {
	pub fn byte_start(&self) -> usize {
		let Span(byte_start, _) = self;
		*byte_start
	}

	pub fn byte_end(&self) -> usize {
		let Span(_, byte_end) = self;
		*byte_end
	}
}

impl fmt::Debug for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Span(byte_start, byte_end) = self;
		write!(f, "Span({}, {})", byte_start, byte_end)
	}
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
	pub node: T,
	pub span: Span,
}

impl<T> Spanned<T> {
	pub fn new(node: T, span: Span) -> Self {
		Spanned { node, span }
	}

	pub fn map<R>(self, node: R) -> Spanned<R> {
		Spanned::new(node, self.span)
	}
}
