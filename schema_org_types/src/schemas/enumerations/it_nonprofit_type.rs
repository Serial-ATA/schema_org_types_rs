/// <https://schema.org/ITNonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ItNonprofitType {
	/// <https://schema.org/ITAmateurSportsClubCharity>
	ItAmateurSportsClubCharity,
	/// <https://schema.org/ITAssociativeNetworkCharity>
	ItAssociativeNetworkCharity,
	/// <https://schema.org/ITCooperativeCharity>
	#[deprecated = "This schema is superseded by <https://schema.org/ITSocialCooperativeCharity>."]
	ItCooperativeCharity,
	/// <https://schema.org/ITMutualAidCharity>
	ItMutualAidCharity,
	/// <https://schema.org/ITOtherThirdSectorEntityCharity>
	ItOtherThirdSectorEntityCharity,
	/// <https://schema.org/ITPhilanthropicEntityCharity>
	ItPhilanthropicEntityCharity,
	/// <https://schema.org/ITSocialCompanyCharity>
	#[deprecated = "This schema is superseded by <https://schema.org/ITSocialEnterpriseCharity>."]
	ItSocialCompanyCharity,
	/// <https://schema.org/ITSocialCooperativeCharity>
	ItSocialCooperativeCharity,
	/// <https://schema.org/ITSocialEnterpriseCharity>
	ItSocialEnterpriseCharity,
	/// <https://schema.org/ITSocialPromotionCharity>
	ItSocialPromotionCharity,
	/// <https://schema.org/ITSportCompanyCharity>
	#[deprecated = "This schema is superseded by <https://schema.org/ITAmateurSportsClubCharity>."]
	ItSportCompanyCharity,
	/// <https://schema.org/ITVolunteerAssociationCharity>
	ItVolunteerAssociationCharity,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for ItNonprofitType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ItNonprofitType::ItAmateurSportsClubCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					0u32,
					"ItAmateurSportsClubCharity",
				),
				ItNonprofitType::ItAssociativeNetworkCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					1u32,
					"ItAssociativeNetworkCharity",
				),
				ItNonprofitType::ItCooperativeCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					2u32,
					"ItCooperativeCharity",
				),
				ItNonprofitType::ItMutualAidCharity => {
					serializer.serialize_unit_variant("ItNonprofitType", 3u32, "ItMutualAidCharity")
				}
				ItNonprofitType::ItOtherThirdSectorEntityCharity => serializer
					.serialize_unit_variant(
						"ItNonprofitType",
						4u32,
						"ItOtherThirdSectorEntityCharity",
					),
				ItNonprofitType::ItPhilanthropicEntityCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					5u32,
					"ItPhilanthropicEntityCharity",
				),
				ItNonprofitType::ItSocialCompanyCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					6u32,
					"ItSocialCompanyCharity",
				),
				ItNonprofitType::ItSocialCooperativeCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					7u32,
					"ItSocialCooperativeCharity",
				),
				ItNonprofitType::ItSocialEnterpriseCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					8u32,
					"ItSocialEnterpriseCharity",
				),
				ItNonprofitType::ItSocialPromotionCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					9u32,
					"ItSocialPromotionCharity",
				),
				ItNonprofitType::ItSportCompanyCharity => serializer.serialize_unit_variant(
					"ItNonprofitType",
					10u32,
					"ItSportCompanyCharity",
				),
				ItNonprofitType::ItVolunteerAssociationCharity => serializer
					.serialize_unit_variant(
						"ItNonprofitType",
						11u32,
						"ItVolunteerAssociationCharity",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for ItNonprofitType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ItAmateurSportsClubCharity,
				ItAssociativeNetworkCharity,
				ItCooperativeCharity,
				ItMutualAidCharity,
				ItOtherThirdSectorEntityCharity,
				ItPhilanthropicEntityCharity,
				ItSocialCompanyCharity,
				ItSocialCooperativeCharity,
				ItSocialEnterpriseCharity,
				ItSocialPromotionCharity,
				ItSportCompanyCharity,
				ItVolunteerAssociationCharity,
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
						"ItAmateurSportsClubCharity" => Ok(Field::ItAmateurSportsClubCharity),
						"ItAssociativeNetworkCharity" => Ok(Field::ItAssociativeNetworkCharity),
						"ItCooperativeCharity" => Ok(Field::ItCooperativeCharity),
						"ItMutualAidCharity" => Ok(Field::ItMutualAidCharity),
						"ItOtherThirdSectorEntityCharity" => {
							Ok(Field::ItOtherThirdSectorEntityCharity)
						}
						"ItPhilanthropicEntityCharity" => Ok(Field::ItPhilanthropicEntityCharity),
						"ItSocialCompanyCharity" => Ok(Field::ItSocialCompanyCharity),
						"ItSocialCooperativeCharity" => Ok(Field::ItSocialCooperativeCharity),
						"ItSocialEnterpriseCharity" => Ok(Field::ItSocialEnterpriseCharity),
						"ItSocialPromotionCharity" => Ok(Field::ItSocialPromotionCharity),
						"ItSportCompanyCharity" => Ok(Field::ItSportCompanyCharity),
						"ItVolunteerAssociationCharity" => Ok(Field::ItVolunteerAssociationCharity),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ItAmateurSportsClubCharity" => Ok(Field::ItAmateurSportsClubCharity),
						b"ItAssociativeNetworkCharity" => Ok(Field::ItAssociativeNetworkCharity),
						b"ItCooperativeCharity" => Ok(Field::ItCooperativeCharity),
						b"ItMutualAidCharity" => Ok(Field::ItMutualAidCharity),
						b"ItOtherThirdSectorEntityCharity" => {
							Ok(Field::ItOtherThirdSectorEntityCharity)
						}
						b"ItPhilanthropicEntityCharity" => Ok(Field::ItPhilanthropicEntityCharity),
						b"ItSocialCompanyCharity" => Ok(Field::ItSocialCompanyCharity),
						b"ItSocialCooperativeCharity" => Ok(Field::ItSocialCooperativeCharity),
						b"ItSocialEnterpriseCharity" => Ok(Field::ItSocialEnterpriseCharity),
						b"ItSocialPromotionCharity" => Ok(Field::ItSocialPromotionCharity),
						b"ItSportCompanyCharity" => Ok(Field::ItSportCompanyCharity),
						b"ItVolunteerAssociationCharity" => {
							Ok(Field::ItVolunteerAssociationCharity)
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
				type Value = ItNonprofitType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ITNonprofitType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ItAmateurSportsClubCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItAmateurSportsClubCharity)
						}
						(Field::ItAssociativeNetworkCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItAssociativeNetworkCharity)
						}
						(Field::ItCooperativeCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItCooperativeCharity)
						}
						(Field::ItMutualAidCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItMutualAidCharity)
						}
						(Field::ItOtherThirdSectorEntityCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItOtherThirdSectorEntityCharity)
						}
						(Field::ItPhilanthropicEntityCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItPhilanthropicEntityCharity)
						}
						(Field::ItSocialCompanyCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItSocialCompanyCharity)
						}
						(Field::ItSocialCooperativeCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItSocialCooperativeCharity)
						}
						(Field::ItSocialEnterpriseCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItSocialEnterpriseCharity)
						}
						(Field::ItSocialPromotionCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItSocialPromotionCharity)
						}
						(Field::ItSportCompanyCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItSportCompanyCharity)
						}
						(Field::ItVolunteerAssociationCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItNonprofitType::ItVolunteerAssociationCharity)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ItAmateurSportsClubCharity",
				"ItAssociativeNetworkCharity",
				"ItCooperativeCharity",
				"ItMutualAidCharity",
				"ItOtherThirdSectorEntityCharity",
				"ItPhilanthropicEntityCharity",
				"ItSocialCompanyCharity",
				"ItSocialCooperativeCharity",
				"ItSocialEnterpriseCharity",
				"ItSocialPromotionCharity",
				"ItSportCompanyCharity",
				"ItVolunteerAssociationCharity",
			];
			deserializer.deserialize_enum("ItNonprofitType", VARIANTS, EnumerationVisitor)
		}
	}
}
