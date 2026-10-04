/// <https://schema.org/PriceTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PriceTypeEnumeration {
	/// <https://schema.org/InvoicePrice>
	InvoicePrice,
	/// <https://schema.org/ListPrice>
	ListPrice,
	/// <https://schema.org/MSRP>
	Msrp,
	/// <https://schema.org/MaximumRetailPrice>
	MaximumRetailPrice,
	/// <https://schema.org/MinimumAdvertisedPrice>
	MinimumAdvertisedPrice,
	/// <https://schema.org/RegularPrice>
	RegularPrice,
	/// <https://schema.org/SRP>
	Srp,
	/// <https://schema.org/SalePrice>
	SalePrice,
	/// <https://schema.org/StrikethroughPrice>
	StrikethroughPrice,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for PriceTypeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PriceTypeEnumeration::InvoicePrice => {
					serializer.serialize_unit_variant("PriceTypeEnumeration", 0u32, "InvoicePrice")
				}
				PriceTypeEnumeration::ListPrice => {
					serializer.serialize_unit_variant("PriceTypeEnumeration", 1u32, "ListPrice")
				}
				PriceTypeEnumeration::Msrp => {
					serializer.serialize_unit_variant("PriceTypeEnumeration", 2u32, "Msrp")
				}
				PriceTypeEnumeration::MaximumRetailPrice => serializer.serialize_unit_variant(
					"PriceTypeEnumeration",
					3u32,
					"MaximumRetailPrice",
				),
				PriceTypeEnumeration::MinimumAdvertisedPrice => serializer.serialize_unit_variant(
					"PriceTypeEnumeration",
					4u32,
					"MinimumAdvertisedPrice",
				),
				PriceTypeEnumeration::RegularPrice => {
					serializer.serialize_unit_variant("PriceTypeEnumeration", 5u32, "RegularPrice")
				}
				PriceTypeEnumeration::Srp => {
					serializer.serialize_unit_variant("PriceTypeEnumeration", 6u32, "Srp")
				}
				PriceTypeEnumeration::SalePrice => {
					serializer.serialize_unit_variant("PriceTypeEnumeration", 7u32, "SalePrice")
				}
				PriceTypeEnumeration::StrikethroughPrice => serializer.serialize_unit_variant(
					"PriceTypeEnumeration",
					8u32,
					"StrikethroughPrice",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for PriceTypeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				InvoicePrice,
				ListPrice,
				Msrp,
				MaximumRetailPrice,
				MinimumAdvertisedPrice,
				RegularPrice,
				Srp,
				SalePrice,
				StrikethroughPrice,
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
						"InvoicePrice" => Ok(Field::InvoicePrice),
						"ListPrice" => Ok(Field::ListPrice),
						"Msrp" => Ok(Field::Msrp),
						"MaximumRetailPrice" => Ok(Field::MaximumRetailPrice),
						"MinimumAdvertisedPrice" => Ok(Field::MinimumAdvertisedPrice),
						"RegularPrice" => Ok(Field::RegularPrice),
						"Srp" => Ok(Field::Srp),
						"SalePrice" => Ok(Field::SalePrice),
						"StrikethroughPrice" => Ok(Field::StrikethroughPrice),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"InvoicePrice" => Ok(Field::InvoicePrice),
						b"ListPrice" => Ok(Field::ListPrice),
						b"Msrp" => Ok(Field::Msrp),
						b"MaximumRetailPrice" => Ok(Field::MaximumRetailPrice),
						b"MinimumAdvertisedPrice" => Ok(Field::MinimumAdvertisedPrice),
						b"RegularPrice" => Ok(Field::RegularPrice),
						b"Srp" => Ok(Field::Srp),
						b"SalePrice" => Ok(Field::SalePrice),
						b"StrikethroughPrice" => Ok(Field::StrikethroughPrice),
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
				type Value = PriceTypeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PriceTypeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::InvoicePrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::InvoicePrice)
						}
						(Field::ListPrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::ListPrice)
						}
						(Field::Msrp, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::Msrp)
						}
						(Field::MaximumRetailPrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::MaximumRetailPrice)
						}
						(Field::MinimumAdvertisedPrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::MinimumAdvertisedPrice)
						}
						(Field::RegularPrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::RegularPrice)
						}
						(Field::Srp, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::Srp)
						}
						(Field::SalePrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::SalePrice)
						}
						(Field::StrikethroughPrice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceTypeEnumeration::StrikethroughPrice)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"InvoicePrice",
				"ListPrice",
				"Msrp",
				"MaximumRetailPrice",
				"MinimumAdvertisedPrice",
				"RegularPrice",
				"Srp",
				"SalePrice",
				"StrikethroughPrice",
			];
			deserializer.deserialize_enum("PriceTypeEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}
