/// <https://schema.org/IncentiveStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IncentiveStatus {
	/// <https://schema.org/IncentiveStatusActive>
	IncentiveStatusActive,
	/// <https://schema.org/IncentiveStatusInDevelopment>
	IncentiveStatusInDevelopment,
	/// <https://schema.org/IncentiveStatusOnHold>
	IncentiveStatusOnHold,
	/// <https://schema.org/IncentiveStatusRetired>
	IncentiveStatusRetired,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for IncentiveStatus {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				IncentiveStatus::IncentiveStatusActive => serializer.serialize_unit_variant(
					"IncentiveStatus",
					0u32,
					"IncentiveStatusActive",
				),
				IncentiveStatus::IncentiveStatusInDevelopment => serializer.serialize_unit_variant(
					"IncentiveStatus",
					1u32,
					"IncentiveStatusInDevelopment",
				),
				IncentiveStatus::IncentiveStatusOnHold => serializer.serialize_unit_variant(
					"IncentiveStatus",
					2u32,
					"IncentiveStatusOnHold",
				),
				IncentiveStatus::IncentiveStatusRetired => serializer.serialize_unit_variant(
					"IncentiveStatus",
					3u32,
					"IncentiveStatusRetired",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for IncentiveStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				IncentiveStatusActive,
				IncentiveStatusInDevelopment,
				IncentiveStatusOnHold,
				IncentiveStatusRetired,
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
						"IncentiveStatusActive" => Ok(Field::IncentiveStatusActive),
						"IncentiveStatusInDevelopment" => Ok(Field::IncentiveStatusInDevelopment),
						"IncentiveStatusOnHold" => Ok(Field::IncentiveStatusOnHold),
						"IncentiveStatusRetired" => Ok(Field::IncentiveStatusRetired),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"IncentiveStatusActive" => Ok(Field::IncentiveStatusActive),
						b"IncentiveStatusInDevelopment" => Ok(Field::IncentiveStatusInDevelopment),
						b"IncentiveStatusOnHold" => Ok(Field::IncentiveStatusOnHold),
						b"IncentiveStatusRetired" => Ok(Field::IncentiveStatusRetired),
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
				type Value = IncentiveStatus;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema IncentiveStatus")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::IncentiveStatusActive, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveStatus::IncentiveStatusActive)
						}
						(Field::IncentiveStatusInDevelopment, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveStatus::IncentiveStatusInDevelopment)
						}
						(Field::IncentiveStatusOnHold, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveStatus::IncentiveStatusOnHold)
						}
						(Field::IncentiveStatusRetired, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IncentiveStatus::IncentiveStatusRetired)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"IncentiveStatusActive",
				"IncentiveStatusInDevelopment",
				"IncentiveStatusOnHold",
				"IncentiveStatusRetired",
			];
			deserializer.deserialize_enum("IncentiveStatus", VARIANTS, EnumerationVisitor)
		}
	}
}
