use super::*;
/// <https://schema.org/isVariantOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsVariantOfProperty {
	/// <https://schema.org/ProductGroup>
	ProductGroup(ProductGroup),
	/// <https://schema.org/ProductModel>
	ProductModel(ProductModel),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
