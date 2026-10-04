use super::*;
/// <https://schema.org/typeOfGood>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypeOfGoodProperty {
	/// <https://schema.org/Product>
	Product(Product),
	/// <https://schema.org/Service>
	Service(Service),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
