use crate::Location;

impl<F: Clone> Location<F> {
	pub fn as_primary_label(&self) -> codespan_reporting::diagnostic::Label<F> {
		codespan_reporting::diagnostic::Label::primary(self.file().clone(), self.span())
	}

	pub fn as_secondary_label(&self) -> codespan_reporting::diagnostic::Label<F> {
		codespan_reporting::diagnostic::Label::secondary(self.file().clone(), self.span())
	}
}

impl<F: Clone> Location<F> {
	pub fn into_primary_label(self) -> codespan_reporting::diagnostic::Label<F> {
		let (file, span) = self.into_parts();
		codespan_reporting::diagnostic::Label::primary(file, span)
	}

	pub fn into_secondary_label(self) -> codespan_reporting::diagnostic::Label<F> {
		let (file, span) = self.into_parts();
		codespan_reporting::diagnostic::Label::secondary(file, span)
	}
}
