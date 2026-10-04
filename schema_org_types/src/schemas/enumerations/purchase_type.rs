/// <https://schema.org/PurchaseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PurchaseType {
	/// <https://schema.org/PurchaseTypeLease>
	PurchaseTypeLease,
	/// <https://schema.org/PurchaseTypeNewPurchase>
	PurchaseTypeNewPurchase,
	/// <https://schema.org/PurchaseTypeTradeIn>
	PurchaseTypeTradeIn,
	/// <https://schema.org/PurchaseTypeUsedPurchase>
	PurchaseTypeUsedPurchase,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for PurchaseType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PurchaseType::PurchaseTypeLease => {
					serializer.serialize_unit_variant("PurchaseType", 0u32, "PurchaseTypeLease")
				}
				PurchaseType::PurchaseTypeNewPurchase => serializer.serialize_unit_variant(
					"PurchaseType",
					1u32,
					"PurchaseTypeNewPurchase",
				),
				PurchaseType::PurchaseTypeTradeIn => {
					serializer.serialize_unit_variant("PurchaseType", 2u32, "PurchaseTypeTradeIn")
				}
				PurchaseType::PurchaseTypeUsedPurchase => serializer.serialize_unit_variant(
					"PurchaseType",
					3u32,
					"PurchaseTypeUsedPurchase",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for PurchaseType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				PurchaseTypeLease,
				PurchaseTypeNewPurchase,
				PurchaseTypeTradeIn,
				PurchaseTypeUsedPurchase,
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
						"PurchaseTypeLease" => Ok(Field::PurchaseTypeLease),
						"PurchaseTypeNewPurchase" => Ok(Field::PurchaseTypeNewPurchase),
						"PurchaseTypeTradeIn" => Ok(Field::PurchaseTypeTradeIn),
						"PurchaseTypeUsedPurchase" => Ok(Field::PurchaseTypeUsedPurchase),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"PurchaseTypeLease" => Ok(Field::PurchaseTypeLease),
						b"PurchaseTypeNewPurchase" => Ok(Field::PurchaseTypeNewPurchase),
						b"PurchaseTypeTradeIn" => Ok(Field::PurchaseTypeTradeIn),
						b"PurchaseTypeUsedPurchase" => Ok(Field::PurchaseTypeUsedPurchase),
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
				type Value = PurchaseType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PurchaseType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::PurchaseTypeLease, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PurchaseType::PurchaseTypeLease)
						}
						(Field::PurchaseTypeNewPurchase, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PurchaseType::PurchaseTypeNewPurchase)
						}
						(Field::PurchaseTypeTradeIn, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PurchaseType::PurchaseTypeTradeIn)
						}
						(Field::PurchaseTypeUsedPurchase, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PurchaseType::PurchaseTypeUsedPurchase)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"PurchaseTypeLease",
				"PurchaseTypeNewPurchase",
				"PurchaseTypeTradeIn",
				"PurchaseTypeUsedPurchase",
			];
			deserializer.deserialize_enum("PurchaseType", VARIANTS, EnumerationVisitor)
		}
	}
}
