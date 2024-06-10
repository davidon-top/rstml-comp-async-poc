#![feature(async_closure)]

use async_trait::async_trait;
use bytes::BytesMut;
use rstml_component::{write_html, HtmlAsyncComponent, HtmlAsyncContent, HtmlFormatter};
use rstml_component_macro::write_async_html;

#[derive(HtmlAsyncComponent)]
struct AsyncSimple {}

#[async_trait]
impl HtmlAsyncContent for AsyncSimple {
	async fn fmt(self, formatter: &mut HtmlFormatter) -> std::fmt::Result {
		tokio::time::sleep(std::time::Duration::from_secs(1)).await;
		write_html! {formatter,
			<div></div>
		}
	}
}

#[derive(HtmlAsyncComponent)]
struct Nested {}

#[async_trait]
impl HtmlAsyncContent for Nested {
	async fn fmt(self, formatter: &mut HtmlFormatter) -> std::fmt::Result {
		write_async_html! {formatter,
			<AsyncSimple::await />
		}
	}
}

#[derive(HtmlAsyncComponent)]
struct AsyncChildren<C> where
 C: HtmlAsyncContent
{
	children: C
}

#[async_trait]
impl<C: HtmlAsyncContent> HtmlAsyncContent for AsyncChildren<C> {
	async fn fmt(self, formatter: &mut HtmlFormatter) -> std::fmt::Result {
		write_async_html! {formatter,
			<div>{self.children}</div>
		}
	}
}

#[derive(HtmlAsyncComponent)]
struct NestedAsyncChildren {}

#[async_trait]
impl HtmlAsyncContent for NestedAsyncChildren {
	async fn fmt(self, formatter: &mut HtmlFormatter) -> std::fmt::Result {
		write_async_html! {
			<AsyncChildren::await><Nested /></AsyncChildren::await>
		}
	}
}

#[tokio::test]
async fn async_formatter() {
	let html = async move |html: &mut rstml_component::HtmlFormatter| -> std::fmt::Result {
		html.write_async_content(AsyncSimple {}).await?;
		Ok(())
	};

	let mut buffer = BytesMut::new();
	let mut formatter = HtmlFormatter::new(&mut buffer);
	let formatter = &mut formatter;

	html(formatter).await.unwrap();
	let raw = buffer.as_ref();
	let as_str = std::str::from_utf8(raw).expect("invalid utf-8");
	assert_eq!(as_str, "<div></div>")
}
