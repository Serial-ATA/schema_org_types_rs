use quote::{__private::TokenStream, ToTokens, TokenStreamExt, quote};

#[derive(Debug, Clone)]
pub enum Feature {
	Name(String),
	All(Vec<Feature>),
}

impl ToTokens for Feature {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(match self {
			Feature::Name(name) => quote!(feature = #name),
			Feature::All(features) => quote!(all(#(#features),*)),
		});
	}
}

impl Feature {
	pub fn as_cfg_attribute(&self) -> TokenStream {
		let features_cfg = self.to_token_stream();
		quote!(
			#[cfg(any(#features_cfg, doc))]
		)
	}
}
