use crate::{HtmlAsyncContent, HtmlContent};

pub trait HtmlComponent {
	type Content: HtmlContent;

	fn into_content(self) -> Self::Content;
}

pub trait HtmlAsyncComponent {
	type Content: HtmlAsyncContent;

	fn into_content(self) -> Self::Content;
}
