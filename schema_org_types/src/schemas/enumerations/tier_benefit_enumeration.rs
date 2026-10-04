/// <https://schema.org/TierBenefitEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum TierBenefitEnumeration {
	/// <https://schema.org/TierBenefitLoyaltyPoints>
	TierBenefitLoyaltyPoints,
	/// <https://schema.org/TierBenefitLoyaltyPrice>
	TierBenefitLoyaltyPrice,
	/// <https://schema.org/TierBenefitLoyaltyReturns>
	TierBenefitLoyaltyReturns,
	/// <https://schema.org/TierBenefitLoyaltyShipping>
	TierBenefitLoyaltyShipping,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for TierBenefitEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				TierBenefitEnumeration::TierBenefitLoyaltyPoints => serializer
					.serialize_unit_variant(
						"TierBenefitEnumeration",
						0u32,
						"TierBenefitLoyaltyPoints",
					),
				TierBenefitEnumeration::TierBenefitLoyaltyPrice => serializer
					.serialize_unit_variant(
						"TierBenefitEnumeration",
						1u32,
						"TierBenefitLoyaltyPrice",
					),
				TierBenefitEnumeration::TierBenefitLoyaltyReturns => serializer
					.serialize_unit_variant(
						"TierBenefitEnumeration",
						2u32,
						"TierBenefitLoyaltyReturns",
					),
				TierBenefitEnumeration::TierBenefitLoyaltyShipping => serializer
					.serialize_unit_variant(
						"TierBenefitEnumeration",
						3u32,
						"TierBenefitLoyaltyShipping",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for TierBenefitEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				TierBenefitLoyaltyPoints,
				TierBenefitLoyaltyPrice,
				TierBenefitLoyaltyReturns,
				TierBenefitLoyaltyShipping,
			}
			struct FieldVisitor;
			impl<'de> de::Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("variant identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						"TierBenefitLoyaltyPoints" => Ok(Field::TierBenefitLoyaltyPoints),
						"TierBenefitLoyaltyPrice" => Ok(Field::TierBenefitLoyaltyPrice),
						"TierBenefitLoyaltyReturns" => Ok(Field::TierBenefitLoyaltyReturns),
						"TierBenefitLoyaltyShipping" => Ok(Field::TierBenefitLoyaltyShipping),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"TierBenefitLoyaltyPoints" => Ok(Field::TierBenefitLoyaltyPoints),
						b"TierBenefitLoyaltyPrice" => Ok(Field::TierBenefitLoyaltyPrice),
						b"TierBenefitLoyaltyReturns" => Ok(Field::TierBenefitLoyaltyReturns),
						b"TierBenefitLoyaltyShipping" => Ok(Field::TierBenefitLoyaltyShipping),
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_variant(value, VARIANTS))
						}
					}
				}
			}
			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_identifier(FieldVisitor)
				}
			}
			struct EnumerationVisitor;
			impl<'de> Visitor<'de> for EnumerationVisitor {
				type Value = TierBenefitEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema TierBenefitEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::TierBenefitLoyaltyPoints, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(TierBenefitEnumeration::TierBenefitLoyaltyPoints)
						}
						(Field::TierBenefitLoyaltyPrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(TierBenefitEnumeration::TierBenefitLoyaltyPrice)
						}
						(Field::TierBenefitLoyaltyReturns, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(TierBenefitEnumeration::TierBenefitLoyaltyReturns)
						}
						(Field::TierBenefitLoyaltyShipping, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(TierBenefitEnumeration::TierBenefitLoyaltyShipping)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"TierBenefitLoyaltyPoints",
				"TierBenefitLoyaltyPrice",
				"TierBenefitLoyaltyReturns",
				"TierBenefitLoyaltyShipping",
			];
			deserializer.deserialize_enum("TierBenefitEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}
