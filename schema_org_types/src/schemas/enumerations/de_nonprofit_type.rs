/// <https://schema.org/DENonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DeNonprofitType {
	/// <https://schema.org/DECooperativeCharity>
	DeCooperativeCharity,
	/// <https://schema.org/DEFoundationCharity>
	DeFoundationCharity,
	/// <https://schema.org/DEJointStockCompanyCharity>
	DeJointStockCompanyCharity,
	/// <https://schema.org/DELimitedLiabilityCharity>
	DeLimitedLiabilityCharity,
	/// <https://schema.org/DENotRegisteredAssociationCharity>
	DeNotRegisteredAssociationCharity,
	/// <https://schema.org/DEPublicCharity>
	DePublicCharity,
	/// <https://schema.org/DERegisteredAssociationCharity>
	DeRegisteredAssociationCharity,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for DeNonprofitType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DeNonprofitType::DeCooperativeCharity => serializer.serialize_unit_variant(
					"DeNonprofitType",
					0u32,
					"DeCooperativeCharity",
				),
				DeNonprofitType::DeFoundationCharity => serializer.serialize_unit_variant(
					"DeNonprofitType",
					1u32,
					"DeFoundationCharity",
				),
				DeNonprofitType::DeJointStockCompanyCharity => serializer.serialize_unit_variant(
					"DeNonprofitType",
					2u32,
					"DeJointStockCompanyCharity",
				),
				DeNonprofitType::DeLimitedLiabilityCharity => serializer.serialize_unit_variant(
					"DeNonprofitType",
					3u32,
					"DeLimitedLiabilityCharity",
				),
				DeNonprofitType::DeNotRegisteredAssociationCharity => serializer
					.serialize_unit_variant(
						"DeNonprofitType",
						4u32,
						"DeNotRegisteredAssociationCharity",
					),
				DeNonprofitType::DePublicCharity => {
					serializer.serialize_unit_variant("DeNonprofitType", 5u32, "DePublicCharity")
				}
				DeNonprofitType::DeRegisteredAssociationCharity => serializer
					.serialize_unit_variant(
						"DeNonprofitType",
						6u32,
						"DeRegisteredAssociationCharity",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for DeNonprofitType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DeCooperativeCharity,
				DeFoundationCharity,
				DeJointStockCompanyCharity,
				DeLimitedLiabilityCharity,
				DeNotRegisteredAssociationCharity,
				DePublicCharity,
				DeRegisteredAssociationCharity,
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
						"DeCooperativeCharity" => Ok(Field::DeCooperativeCharity),
						"DeFoundationCharity" => Ok(Field::DeFoundationCharity),
						"DeJointStockCompanyCharity" => Ok(Field::DeJointStockCompanyCharity),
						"DeLimitedLiabilityCharity" => Ok(Field::DeLimitedLiabilityCharity),
						"DeNotRegisteredAssociationCharity" => {
							Ok(Field::DeNotRegisteredAssociationCharity)
						}
						"DePublicCharity" => Ok(Field::DePublicCharity),
						"DeRegisteredAssociationCharity" => {
							Ok(Field::DeRegisteredAssociationCharity)
						}
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DeCooperativeCharity" => Ok(Field::DeCooperativeCharity),
						b"DeFoundationCharity" => Ok(Field::DeFoundationCharity),
						b"DeJointStockCompanyCharity" => Ok(Field::DeJointStockCompanyCharity),
						b"DeLimitedLiabilityCharity" => Ok(Field::DeLimitedLiabilityCharity),
						b"DeNotRegisteredAssociationCharity" => {
							Ok(Field::DeNotRegisteredAssociationCharity)
						}
						b"DePublicCharity" => Ok(Field::DePublicCharity),
						b"DeRegisteredAssociationCharity" => {
							Ok(Field::DeRegisteredAssociationCharity)
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
				type Value = DeNonprofitType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DENonprofitType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DeCooperativeCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DeCooperativeCharity)
						}
						(Field::DeFoundationCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DeFoundationCharity)
						}
						(Field::DeJointStockCompanyCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DeJointStockCompanyCharity)
						}
						(Field::DeLimitedLiabilityCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DeLimitedLiabilityCharity)
						}
						(Field::DeNotRegisteredAssociationCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DeNotRegisteredAssociationCharity)
						}
						(Field::DePublicCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DePublicCharity)
						}
						(Field::DeRegisteredAssociationCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeNonprofitType::DeRegisteredAssociationCharity)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"DeCooperativeCharity",
				"DeFoundationCharity",
				"DeJointStockCompanyCharity",
				"DeLimitedLiabilityCharity",
				"DeNotRegisteredAssociationCharity",
				"DePublicCharity",
				"DeRegisteredAssociationCharity",
			];
			deserializer.deserialize_enum("DeNonprofitType", VARIANTS, EnumerationVisitor)
		}
	}
}
