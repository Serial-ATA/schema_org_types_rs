/// <https://schema.org/FulfillmentTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum FulfillmentTypeEnumeration {
	/// <https://schema.org/FulfillmentTypeCollectionPoint>
	FulfillmentTypeCollectionPoint,
	/// <https://schema.org/FulfillmentTypeDelivery>
	FulfillmentTypeDelivery,
	/// <https://schema.org/FulfillmentTypePickupDropoff>
	FulfillmentTypePickupDropoff,
	/// <https://schema.org/FulfillmentTypePickupInStore>
	FulfillmentTypePickupInStore,
	/// <https://schema.org/FulfillmentTypeScheduledDelivery>
	FulfillmentTypeScheduledDelivery,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for FulfillmentTypeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				FulfillmentTypeEnumeration::FulfillmentTypeCollectionPoint => serializer
					.serialize_unit_variant(
						"FulfillmentTypeEnumeration",
						0u32,
						"FulfillmentTypeCollectionPoint",
					),
				FulfillmentTypeEnumeration::FulfillmentTypeDelivery => serializer
					.serialize_unit_variant(
						"FulfillmentTypeEnumeration",
						1u32,
						"FulfillmentTypeDelivery",
					),
				FulfillmentTypeEnumeration::FulfillmentTypePickupDropoff => serializer
					.serialize_unit_variant(
						"FulfillmentTypeEnumeration",
						2u32,
						"FulfillmentTypePickupDropoff",
					),
				FulfillmentTypeEnumeration::FulfillmentTypePickupInStore => serializer
					.serialize_unit_variant(
						"FulfillmentTypeEnumeration",
						3u32,
						"FulfillmentTypePickupInStore",
					),
				FulfillmentTypeEnumeration::FulfillmentTypeScheduledDelivery => serializer
					.serialize_unit_variant(
						"FulfillmentTypeEnumeration",
						4u32,
						"FulfillmentTypeScheduledDelivery",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for FulfillmentTypeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				FulfillmentTypeCollectionPoint,
				FulfillmentTypeDelivery,
				FulfillmentTypePickupDropoff,
				FulfillmentTypePickupInStore,
				FulfillmentTypeScheduledDelivery,
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
						"FulfillmentTypeCollectionPoint" => {
							Ok(Field::FulfillmentTypeCollectionPoint)
						}
						"FulfillmentTypeDelivery" => Ok(Field::FulfillmentTypeDelivery),
						"FulfillmentTypePickupDropoff" => Ok(Field::FulfillmentTypePickupDropoff),
						"FulfillmentTypePickupInStore" => Ok(Field::FulfillmentTypePickupInStore),
						"FulfillmentTypeScheduledDelivery" => {
							Ok(Field::FulfillmentTypeScheduledDelivery)
						}
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"FulfillmentTypeCollectionPoint" => {
							Ok(Field::FulfillmentTypeCollectionPoint)
						}
						b"FulfillmentTypeDelivery" => Ok(Field::FulfillmentTypeDelivery),
						b"FulfillmentTypePickupDropoff" => Ok(Field::FulfillmentTypePickupDropoff),
						b"FulfillmentTypePickupInStore" => Ok(Field::FulfillmentTypePickupInStore),
						b"FulfillmentTypeScheduledDelivery" => {
							Ok(Field::FulfillmentTypeScheduledDelivery)
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
				type Value = FulfillmentTypeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema FulfillmentTypeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::FulfillmentTypeCollectionPoint, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(FulfillmentTypeEnumeration::FulfillmentTypeCollectionPoint)
						}
						(Field::FulfillmentTypeDelivery, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(FulfillmentTypeEnumeration::FulfillmentTypeDelivery)
						}
						(Field::FulfillmentTypePickupDropoff, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(FulfillmentTypeEnumeration::FulfillmentTypePickupDropoff)
						}
						(Field::FulfillmentTypePickupInStore, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(FulfillmentTypeEnumeration::FulfillmentTypePickupInStore)
						}
						(Field::FulfillmentTypeScheduledDelivery, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(FulfillmentTypeEnumeration::FulfillmentTypeScheduledDelivery)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"FulfillmentTypeCollectionPoint",
				"FulfillmentTypeDelivery",
				"FulfillmentTypePickupDropoff",
				"FulfillmentTypePickupInStore",
				"FulfillmentTypeScheduledDelivery",
			];
			deserializer.deserialize_enum(
				"FulfillmentTypeEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}
