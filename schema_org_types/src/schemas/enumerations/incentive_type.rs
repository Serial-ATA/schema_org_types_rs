/// <https://schema.org/IncentiveType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IncentiveType {
	/// <https://schema.org/IncentiveTypeLoan>
	IncentiveTypeLoan,
	/// <https://schema.org/IncentiveTypeRebateOrSubsidy>
	IncentiveTypeRebateOrSubsidy,
	/// <https://schema.org/IncentiveTypeTaxCredit>
	IncentiveTypeTaxCredit,
	/// <https://schema.org/IncentiveTypeTaxDeduction>
	IncentiveTypeTaxDeduction,
	/// <https://schema.org/IncentiveTypeTaxWaiver>
	IncentiveTypeTaxWaiver,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for IncentiveType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				IncentiveType::IncentiveTypeLoan => {
					serializer.serialize_unit_variant("IncentiveType", 0u32, "IncentiveTypeLoan")
				}
				IncentiveType::IncentiveTypeRebateOrSubsidy => serializer.serialize_unit_variant(
					"IncentiveType",
					1u32,
					"IncentiveTypeRebateOrSubsidy",
				),
				IncentiveType::IncentiveTypeTaxCredit => serializer.serialize_unit_variant(
					"IncentiveType",
					2u32,
					"IncentiveTypeTaxCredit",
				),
				IncentiveType::IncentiveTypeTaxDeduction => serializer.serialize_unit_variant(
					"IncentiveType",
					3u32,
					"IncentiveTypeTaxDeduction",
				),
				IncentiveType::IncentiveTypeTaxWaiver => serializer.serialize_unit_variant(
					"IncentiveType",
					4u32,
					"IncentiveTypeTaxWaiver",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for IncentiveType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				IncentiveTypeLoan,
				IncentiveTypeRebateOrSubsidy,
				IncentiveTypeTaxCredit,
				IncentiveTypeTaxDeduction,
				IncentiveTypeTaxWaiver,
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
						"IncentiveTypeLoan" => Ok(Field::IncentiveTypeLoan),
						"IncentiveTypeRebateOrSubsidy" => Ok(Field::IncentiveTypeRebateOrSubsidy),
						"IncentiveTypeTaxCredit" => Ok(Field::IncentiveTypeTaxCredit),
						"IncentiveTypeTaxDeduction" => Ok(Field::IncentiveTypeTaxDeduction),
						"IncentiveTypeTaxWaiver" => Ok(Field::IncentiveTypeTaxWaiver),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"IncentiveTypeLoan" => Ok(Field::IncentiveTypeLoan),
						b"IncentiveTypeRebateOrSubsidy" => Ok(Field::IncentiveTypeRebateOrSubsidy),
						b"IncentiveTypeTaxCredit" => Ok(Field::IncentiveTypeTaxCredit),
						b"IncentiveTypeTaxDeduction" => Ok(Field::IncentiveTypeTaxDeduction),
						b"IncentiveTypeTaxWaiver" => Ok(Field::IncentiveTypeTaxWaiver),
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
				type Value = IncentiveType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema IncentiveType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::IncentiveTypeLoan, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveType::IncentiveTypeLoan)
						}
						(Field::IncentiveTypeRebateOrSubsidy, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveType::IncentiveTypeRebateOrSubsidy)
						}
						(Field::IncentiveTypeTaxCredit, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveType::IncentiveTypeTaxCredit)
						}
						(Field::IncentiveTypeTaxDeduction, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveType::IncentiveTypeTaxDeduction)
						}
						(Field::IncentiveTypeTaxWaiver, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveType::IncentiveTypeTaxWaiver)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"IncentiveTypeLoan",
				"IncentiveTypeRebateOrSubsidy",
				"IncentiveTypeTaxCredit",
				"IncentiveTypeTaxDeduction",
				"IncentiveTypeTaxWaiver",
			];
			deserializer.deserialize_enum("IncentiveType", VARIANTS, EnumerationVisitor)
		}
	}
}
