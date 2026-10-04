/// <https://schema.org/CertificationStatusEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CertificationStatusEnumeration {
	/// <https://schema.org/CertificationActive>
	CertificationActive,
	/// <https://schema.org/CertificationInactive>
	CertificationInactive,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for CertificationStatusEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				CertificationStatusEnumeration::CertificationActive => serializer
					.serialize_unit_variant(
						"CertificationStatusEnumeration",
						0u32,
						"CertificationActive",
					),
				CertificationStatusEnumeration::CertificationInactive => serializer
					.serialize_unit_variant(
						"CertificationStatusEnumeration",
						1u32,
						"CertificationInactive",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for CertificationStatusEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CertificationActive,
				CertificationInactive,
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
						"CertificationActive" => Ok(Field::CertificationActive),
						"CertificationInactive" => Ok(Field::CertificationInactive),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CertificationActive" => Ok(Field::CertificationActive),
						b"CertificationInactive" => Ok(Field::CertificationInactive),
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
				type Value = CertificationStatusEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema CertificationStatusEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CertificationActive, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(CertificationStatusEnumeration::CertificationActive)
						}
						(Field::CertificationInactive, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(CertificationStatusEnumeration::CertificationInactive)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["CertificationActive", "CertificationInactive"];
			deserializer.deserialize_enum(
				"CertificationStatusEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}
