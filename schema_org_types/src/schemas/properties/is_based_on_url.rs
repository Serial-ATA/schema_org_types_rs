use super::*;
/// <https://schema.org/isBasedOnUrl>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
pub enum IsBasedOnUrlProperty {
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/Product>
	Product(Product),
	/// <https://schema.org/URL>
	Url(Url),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
