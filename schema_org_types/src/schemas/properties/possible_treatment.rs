use super::*;
/// <https://schema.org/possibleTreatment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PossibleTreatmentProperty {
	/// <https://schema.org/Drug>
	Drug(Drug),
	/// <https://schema.org/DrugClass>
	DrugClass(DrugClass),
	/// <https://schema.org/LifestyleModification>
	LifestyleModification(LifestyleModification),
	/// <https://schema.org/MedicalTherapy>
	MedicalTherapy(MedicalTherapy),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for PossibleTreatmentProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PossibleTreatmentProperty::Drug(ref inner) => inner.serialize(serializer),
				PossibleTreatmentProperty::DrugClass(ref inner) => inner.serialize(serializer),
				PossibleTreatmentProperty::LifestyleModification(ref inner) => {
					inner.serialize(serializer)
				}
				PossibleTreatmentProperty::MedicalTherapy(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				PossibleTreatmentProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for PossibleTreatmentProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<Drug as Deserialize>::deserialize(deserializer),
				PossibleTreatmentProperty::Drug,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<DrugClass as Deserialize>::deserialize(deserializer),
				PossibleTreatmentProperty::DrugClass,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<LifestyleModification as Deserialize>::deserialize(deserializer),
				PossibleTreatmentProperty::LifestyleModification,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<MedicalTherapy as Deserialize>::deserialize(deserializer),
				PossibleTreatmentProperty::MedicalTherapy,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				PossibleTreatmentProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property possibleTreatment or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property possibleTreatment";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
