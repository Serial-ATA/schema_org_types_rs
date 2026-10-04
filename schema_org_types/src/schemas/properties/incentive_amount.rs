use super::*;
/// <https://schema.org/incentiveAmount>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IncentiveAmountProperty {
	/// <https://schema.org/LoanOrCredit>
	LoanOrCredit(LoanOrCredit),
	/// <https://schema.org/QuantitativeValue>
	QuantitativeValue(QuantitativeValue),
	/// <https://schema.org/UnitPriceSpecification>
	UnitPriceSpecification(UnitPriceSpecification),
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
	impl Serialize for IncentiveAmountProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				IncentiveAmountProperty::LoanOrCredit(ref inner) => inner.serialize(serializer),
				IncentiveAmountProperty::QuantitativeValue(ref inner) => {
					inner.serialize(serializer)
				}
				IncentiveAmountProperty::UnitPriceSpecification(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(all(feature = "fallible", feature = "serde"))]
				IncentiveAmountProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for IncentiveAmountProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<LoanOrCredit as Deserialize>::deserialize(deserializer),
				IncentiveAmountProperty::LoanOrCredit,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<QuantitativeValue as Deserialize>::deserialize(deserializer),
				IncentiveAmountProperty::QuantitativeValue,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<UnitPriceSpecification as Deserialize>::deserialize(deserializer),
				IncentiveAmountProperty::UnitPriceSpecification,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				IncentiveAmountProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property incentiveAmount or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property incentiveAmount";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
