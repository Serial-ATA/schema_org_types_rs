use super::*;
/// <https://schema.org/infectiousAgentClass>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InfectiousAgentClassProperty {
	/// <https://schema.org/InfectiousAgentClass>
	InfectiousAgentClass(InfectiousAgentClass),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
