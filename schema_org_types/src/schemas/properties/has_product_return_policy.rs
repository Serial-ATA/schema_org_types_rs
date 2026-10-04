use super::*;
/// <https://schema.org/hasProductReturnPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[deprecated = "This schema is superseded by <https://schema.org/hasMerchantReturnPolicy>."]
pub enum HasProductReturnPolicyProperty {
	/// <https://schema.org/ProductReturnPolicy>
	#[deprecated = "This schema is superseded by <https://schema.org/MerchantReturnPolicy>."]
	ProductReturnPolicy(ProductReturnPolicy),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
