use super::*;
/// <https://schema.org/coursePrerequisites>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoursePrerequisitesProperty {
	/// <https://schema.org/AlignmentObject>
	AlignmentObject(AlignmentObject),
	/// <https://schema.org/Course>
	Course(Course),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
