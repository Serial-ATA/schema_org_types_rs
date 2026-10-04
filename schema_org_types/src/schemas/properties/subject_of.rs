use super::*;
/// <https://schema.org/subjectOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubjectOfProperty {
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/Event>
	Event(Event),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
