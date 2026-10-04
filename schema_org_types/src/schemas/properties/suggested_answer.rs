use super::*;
/// <https://schema.org/suggestedAnswer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuggestedAnswerProperty {
	/// <https://schema.org/Answer>
	Answer(Answer),
	/// <https://schema.org/ItemList>
	ItemList(ItemList),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
