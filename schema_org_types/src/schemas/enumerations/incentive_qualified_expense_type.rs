/// <https://schema.org/IncentiveQualifiedExpenseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IncentiveQualifiedExpenseType {
	/// <https://schema.org/IncentiveQualifiedExpenseTypeGoodsOnly>
	IncentiveQualifiedExpenseTypeGoodsOnly,
	/// <https://schema.org/IncentiveQualifiedExpenseTypeGoodsOrServices>
	IncentiveQualifiedExpenseTypeGoodsOrServices,
	/// <https://schema.org/IncentiveQualifiedExpenseTypeServicesOnly>
	IncentiveQualifiedExpenseTypeServicesOnly,
	/// <https://schema.org/IncentiveQualifiedExpenseTypeUtilityBill>
	IncentiveQualifiedExpenseTypeUtilityBill,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for IncentiveQualifiedExpenseType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeGoodsOnly => serializer
					.serialize_unit_variant(
						"IncentiveQualifiedExpenseType",
						0u32,
						"IncentiveQualifiedExpenseTypeGoodsOnly",
					),
				IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeGoodsOrServices => {
					serializer.serialize_unit_variant(
						"IncentiveQualifiedExpenseType",
						1u32,
						"IncentiveQualifiedExpenseTypeGoodsOrServices",
					)
				}
				IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeServicesOnly => {
					serializer.serialize_unit_variant(
						"IncentiveQualifiedExpenseType",
						2u32,
						"IncentiveQualifiedExpenseTypeServicesOnly",
					)
				}
				IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeUtilityBill => {
					serializer.serialize_unit_variant(
						"IncentiveQualifiedExpenseType",
						3u32,
						"IncentiveQualifiedExpenseTypeUtilityBill",
					)
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for IncentiveQualifiedExpenseType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				IncentiveQualifiedExpenseTypeGoodsOnly,
				IncentiveQualifiedExpenseTypeGoodsOrServices,
				IncentiveQualifiedExpenseTypeServicesOnly,
				IncentiveQualifiedExpenseTypeUtilityBill,
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
						"IncentiveQualifiedExpenseTypeGoodsOnly" => {
							Ok(Field::IncentiveQualifiedExpenseTypeGoodsOnly)
						}
						"IncentiveQualifiedExpenseTypeGoodsOrServices" => {
							Ok(Field::IncentiveQualifiedExpenseTypeGoodsOrServices)
						}
						"IncentiveQualifiedExpenseTypeServicesOnly" => {
							Ok(Field::IncentiveQualifiedExpenseTypeServicesOnly)
						}
						"IncentiveQualifiedExpenseTypeUtilityBill" => {
							Ok(Field::IncentiveQualifiedExpenseTypeUtilityBill)
						}
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"IncentiveQualifiedExpenseTypeGoodsOnly" => {
							Ok(Field::IncentiveQualifiedExpenseTypeGoodsOnly)
						}
						b"IncentiveQualifiedExpenseTypeGoodsOrServices" => {
							Ok(Field::IncentiveQualifiedExpenseTypeGoodsOrServices)
						}
						b"IncentiveQualifiedExpenseTypeServicesOnly" => {
							Ok(Field::IncentiveQualifiedExpenseTypeServicesOnly)
						}
						b"IncentiveQualifiedExpenseTypeUtilityBill" => {
							Ok(Field::IncentiveQualifiedExpenseTypeUtilityBill)
						}
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
				type Value = IncentiveQualifiedExpenseType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema IncentiveQualifiedExpenseType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::IncentiveQualifiedExpenseTypeGoodsOnly, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(
                                IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeGoodsOnly,
                            )
						}
						(Field::IncentiveQualifiedExpenseTypeGoodsOrServices, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(
                                IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeGoodsOrServices,
                            )
						}
						(Field::IncentiveQualifiedExpenseTypeServicesOnly, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(
                                IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeServicesOnly,
                            )
						}
						(Field::IncentiveQualifiedExpenseTypeUtilityBill, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(
                                IncentiveQualifiedExpenseType::IncentiveQualifiedExpenseTypeUtilityBill,
                            )
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"IncentiveQualifiedExpenseTypeGoodsOnly",
				"IncentiveQualifiedExpenseTypeGoodsOrServices",
				"IncentiveQualifiedExpenseTypeServicesOnly",
				"IncentiveQualifiedExpenseTypeUtilityBill",
			];
			deserializer.deserialize_enum(
				"IncentiveQualifiedExpenseType",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}
