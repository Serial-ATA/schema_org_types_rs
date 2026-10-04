use super::*;
/// <https://schema.org/answerExplanation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AnswerExplanationProperty {
	/// <https://schema.org/Comment>
	Comment(Comment),
	/// <https://schema.org/WebContent>
	WebContent(WebContent),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
