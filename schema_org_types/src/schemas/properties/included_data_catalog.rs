use super::*;
/// <https://schema.org/includedDataCatalog>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub enum IncludedDataCatalogProperty {
	/// <https://schema.org/DataCatalog>
	DataCatalog(DataCatalog),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
