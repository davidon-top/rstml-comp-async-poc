use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Ident;
use syn::{parse::Parse, Expr, Token};

use crate::template::Template;

struct WriteHtmlExpr {
	writer: Expr,
	#[allow(dead_code)] // used for parsing
	comma: Token![,],
	template: Template,
	is_async: bool,
}

impl Parse for WriteHtmlExpr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(Self {
			writer: input.parse()?,
			comma: input.parse()?,
			template: input.parse()?,
			is_async: false,
		})
	}
}

impl ToTokens for WriteHtmlExpr {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self {
			writer,
			comma: _,
			template,
			is_async,
		} = self;

		let formatter = Ident::new("__html", Span::call_site());
		let template = template.with_formatter(&formatter);

		if *is_async {
			tokens.extend(quote!(async {
				let #formatter: &mut ::rstml_component::HtmlFormatter = #writer.as_mut();
				#formatter.write_async_content(
					async |#formatter: &mut ::rstml_component::HtmlFormatter| -> ::std::fmt::Result {
						#template
						Ok(())
					},
				).await
			}.await));
		} else {
			tokens.extend(quote!({
				let #formatter: &mut ::rstml_component::HtmlFormatter = #writer.as_mut();
				#formatter.write_content(
					|#formatter: &mut ::rstml_component::HtmlFormatter| -> ::std::fmt::Result {
						#template
						Ok(())
					},
				)
			}));
		}
	}
}

pub fn write_html(input: TokenStream, is_async: bool) -> TokenStream {
	let mut expr: WriteHtmlExpr = match syn::parse2(input) {
		Ok(expr) => expr,
		Err(err) => return err.to_compile_error(),
	};

	expr.is_async = is_async;
	expr.into_token_stream()
}

struct HtmlExpr {
	template: Template,
	should_move: bool,
	should_async: bool,
}

impl Parse for HtmlExpr {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let template = input.parse()?;

		Ok(Self {
			template,
			should_move: false,
			should_async: false,
		})
	}
}

impl ToTokens for HtmlExpr {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let formatter = Ident::new("__html", Span::call_site());
		let template = self.template.with_formatter(&formatter);
		let move_token = self.should_move.then(|| quote!(move));
		let async_token = self.should_async.then(|| quote!(async));

		tokens.extend(quote! {
			#async_token #move_token |#formatter: &mut ::rstml_component::HtmlFormatter| -> ::std::fmt::Result {
				#template
				Ok(())
			}
		})
	}
}

pub fn html(input: TokenStream, should_move: bool, should_async: bool) -> TokenStream {
	let mut template: HtmlExpr = match syn::parse2(input) {
		Ok(expr) => expr,
		Err(err) => return err.to_compile_error(),
	};

	template.should_move = should_move;
	template.should_async = should_async;
	template.into_token_stream()
}
