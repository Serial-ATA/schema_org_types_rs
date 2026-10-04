/// <https://schema.org/IPTCDigitalSourceEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IptcDigitalSourceEnumeration {
	/// <https://schema.org/AlgorithmicMediaDigitalSource>
	AlgorithmicMediaDigitalSource,
	/// <https://schema.org/AlgorithmicallyEnhancedDigitalSource>
	AlgorithmicallyEnhancedDigitalSource,
	/// <https://schema.org/CompositeCaptureDigitalSource>
	CompositeCaptureDigitalSource,
	/// <https://schema.org/CompositeDigitalSource>
	CompositeDigitalSource,
	/// <https://schema.org/CompositeSyntheticDigitalSource>
	CompositeSyntheticDigitalSource,
	/// <https://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource>
	CompositeWithTrainedAlgorithmicMediaDigitalSource,
	/// <https://schema.org/DataDrivenMediaDigitalSource>
	DataDrivenMediaDigitalSource,
	/// <https://schema.org/DigitalArtDigitalSource>
	DigitalArtDigitalSource,
	/// <https://schema.org/DigitalCaptureDigitalSource>
	DigitalCaptureDigitalSource,
	/// <https://schema.org/MinorHumanEditsDigitalSource>
	MinorHumanEditsDigitalSource,
	/// <https://schema.org/MultiFrameComputationalCaptureDigitalSource>
	MultiFrameComputationalCaptureDigitalSource,
	/// <https://schema.org/NegativeFilmDigitalSource>
	NegativeFilmDigitalSource,
	/// <https://schema.org/PositiveFilmDigitalSource>
	PositiveFilmDigitalSource,
	/// <https://schema.org/PrintDigitalSource>
	PrintDigitalSource,
	/// <https://schema.org/ScreenCaptureDigitalSource>
	ScreenCaptureDigitalSource,
	/// <https://schema.org/TrainedAlgorithmicMediaDigitalSource>
	TrainedAlgorithmicMediaDigitalSource,
	/// <https://schema.org/VirtualRecordingDigitalSource>
	VirtualRecordingDigitalSource,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for IptcDigitalSourceEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				IptcDigitalSourceEnumeration::AlgorithmicMediaDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						0u32,
						"AlgorithmicMediaDigitalSource",
					),
				IptcDigitalSourceEnumeration::AlgorithmicallyEnhancedDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						1u32,
						"AlgorithmicallyEnhancedDigitalSource",
					),
				IptcDigitalSourceEnumeration::CompositeCaptureDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						2u32,
						"CompositeCaptureDigitalSource",
					),
				IptcDigitalSourceEnumeration::CompositeDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						3u32,
						"CompositeDigitalSource",
					),
				IptcDigitalSourceEnumeration::CompositeSyntheticDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						4u32,
						"CompositeSyntheticDigitalSource",
					),
				IptcDigitalSourceEnumeration::CompositeWithTrainedAlgorithmicMediaDigitalSource => {
					serializer.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						5u32,
						"CompositeWithTrainedAlgorithmicMediaDigitalSource",
					)
				}
				IptcDigitalSourceEnumeration::DataDrivenMediaDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						6u32,
						"DataDrivenMediaDigitalSource",
					),
				IptcDigitalSourceEnumeration::DigitalArtDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						7u32,
						"DigitalArtDigitalSource",
					),
				IptcDigitalSourceEnumeration::DigitalCaptureDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						8u32,
						"DigitalCaptureDigitalSource",
					),
				IptcDigitalSourceEnumeration::MinorHumanEditsDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						9u32,
						"MinorHumanEditsDigitalSource",
					),
				IptcDigitalSourceEnumeration::MultiFrameComputationalCaptureDigitalSource => {
					serializer.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						10u32,
						"MultiFrameComputationalCaptureDigitalSource",
					)
				}
				IptcDigitalSourceEnumeration::NegativeFilmDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						11u32,
						"NegativeFilmDigitalSource",
					),
				IptcDigitalSourceEnumeration::PositiveFilmDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						12u32,
						"PositiveFilmDigitalSource",
					),
				IptcDigitalSourceEnumeration::PrintDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						13u32,
						"PrintDigitalSource",
					),
				IptcDigitalSourceEnumeration::ScreenCaptureDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						14u32,
						"ScreenCaptureDigitalSource",
					),
				IptcDigitalSourceEnumeration::TrainedAlgorithmicMediaDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						15u32,
						"TrainedAlgorithmicMediaDigitalSource",
					),
				IptcDigitalSourceEnumeration::VirtualRecordingDigitalSource => serializer
					.serialize_unit_variant(
						"IptcDigitalSourceEnumeration",
						16u32,
						"VirtualRecordingDigitalSource",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for IptcDigitalSourceEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AlgorithmicMediaDigitalSource,
				AlgorithmicallyEnhancedDigitalSource,
				CompositeCaptureDigitalSource,
				CompositeDigitalSource,
				CompositeSyntheticDigitalSource,
				CompositeWithTrainedAlgorithmicMediaDigitalSource,
				DataDrivenMediaDigitalSource,
				DigitalArtDigitalSource,
				DigitalCaptureDigitalSource,
				MinorHumanEditsDigitalSource,
				MultiFrameComputationalCaptureDigitalSource,
				NegativeFilmDigitalSource,
				PositiveFilmDigitalSource,
				PrintDigitalSource,
				ScreenCaptureDigitalSource,
				TrainedAlgorithmicMediaDigitalSource,
				VirtualRecordingDigitalSource,
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
						"AlgorithmicMediaDigitalSource" => Ok(Field::AlgorithmicMediaDigitalSource),
						"AlgorithmicallyEnhancedDigitalSource" => {
							Ok(Field::AlgorithmicallyEnhancedDigitalSource)
						}
						"CompositeCaptureDigitalSource" => Ok(Field::CompositeCaptureDigitalSource),
						"CompositeDigitalSource" => Ok(Field::CompositeDigitalSource),
						"CompositeSyntheticDigitalSource" => {
							Ok(Field::CompositeSyntheticDigitalSource)
						}
						"CompositeWithTrainedAlgorithmicMediaDigitalSource" => {
							Ok(Field::CompositeWithTrainedAlgorithmicMediaDigitalSource)
						}
						"DataDrivenMediaDigitalSource" => Ok(Field::DataDrivenMediaDigitalSource),
						"DigitalArtDigitalSource" => Ok(Field::DigitalArtDigitalSource),
						"DigitalCaptureDigitalSource" => Ok(Field::DigitalCaptureDigitalSource),
						"MinorHumanEditsDigitalSource" => Ok(Field::MinorHumanEditsDigitalSource),
						"MultiFrameComputationalCaptureDigitalSource" => {
							Ok(Field::MultiFrameComputationalCaptureDigitalSource)
						}
						"NegativeFilmDigitalSource" => Ok(Field::NegativeFilmDigitalSource),
						"PositiveFilmDigitalSource" => Ok(Field::PositiveFilmDigitalSource),
						"PrintDigitalSource" => Ok(Field::PrintDigitalSource),
						"ScreenCaptureDigitalSource" => Ok(Field::ScreenCaptureDigitalSource),
						"TrainedAlgorithmicMediaDigitalSource" => {
							Ok(Field::TrainedAlgorithmicMediaDigitalSource)
						}
						"VirtualRecordingDigitalSource" => Ok(Field::VirtualRecordingDigitalSource),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AlgorithmicMediaDigitalSource" => {
							Ok(Field::AlgorithmicMediaDigitalSource)
						}
						b"AlgorithmicallyEnhancedDigitalSource" => {
							Ok(Field::AlgorithmicallyEnhancedDigitalSource)
						}
						b"CompositeCaptureDigitalSource" => {
							Ok(Field::CompositeCaptureDigitalSource)
						}
						b"CompositeDigitalSource" => Ok(Field::CompositeDigitalSource),
						b"CompositeSyntheticDigitalSource" => {
							Ok(Field::CompositeSyntheticDigitalSource)
						}
						b"CompositeWithTrainedAlgorithmicMediaDigitalSource" => {
							Ok(Field::CompositeWithTrainedAlgorithmicMediaDigitalSource)
						}
						b"DataDrivenMediaDigitalSource" => Ok(Field::DataDrivenMediaDigitalSource),
						b"DigitalArtDigitalSource" => Ok(Field::DigitalArtDigitalSource),
						b"DigitalCaptureDigitalSource" => Ok(Field::DigitalCaptureDigitalSource),
						b"MinorHumanEditsDigitalSource" => Ok(Field::MinorHumanEditsDigitalSource),
						b"MultiFrameComputationalCaptureDigitalSource" => {
							Ok(Field::MultiFrameComputationalCaptureDigitalSource)
						}
						b"NegativeFilmDigitalSource" => Ok(Field::NegativeFilmDigitalSource),
						b"PositiveFilmDigitalSource" => Ok(Field::PositiveFilmDigitalSource),
						b"PrintDigitalSource" => Ok(Field::PrintDigitalSource),
						b"ScreenCaptureDigitalSource" => Ok(Field::ScreenCaptureDigitalSource),
						b"TrainedAlgorithmicMediaDigitalSource" => {
							Ok(Field::TrainedAlgorithmicMediaDigitalSource)
						}
						b"VirtualRecordingDigitalSource" => {
							Ok(Field::VirtualRecordingDigitalSource)
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
				type Value = IptcDigitalSourceEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema IPTCDigitalSourceEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AlgorithmicMediaDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::AlgorithmicMediaDigitalSource)
						}
						(Field::AlgorithmicallyEnhancedDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::AlgorithmicallyEnhancedDigitalSource)
						}
						(Field::CompositeCaptureDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::CompositeCaptureDigitalSource)
						}
						(Field::CompositeDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::CompositeDigitalSource)
						}
						(Field::CompositeSyntheticDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::CompositeSyntheticDigitalSource)
						}
						(Field::CompositeWithTrainedAlgorithmicMediaDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(
                                IptcDigitalSourceEnumeration::CompositeWithTrainedAlgorithmicMediaDigitalSource,
                            )
						}
						(Field::DataDrivenMediaDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::DataDrivenMediaDigitalSource)
						}
						(Field::DigitalArtDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::DigitalArtDigitalSource)
						}
						(Field::DigitalCaptureDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::DigitalCaptureDigitalSource)
						}
						(Field::MinorHumanEditsDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::MinorHumanEditsDigitalSource)
						}
						(Field::MultiFrameComputationalCaptureDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(
                                IptcDigitalSourceEnumeration::MultiFrameComputationalCaptureDigitalSource,
                            )
						}
						(Field::NegativeFilmDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::NegativeFilmDigitalSource)
						}
						(Field::PositiveFilmDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::PositiveFilmDigitalSource)
						}
						(Field::PrintDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::PrintDigitalSource)
						}
						(Field::ScreenCaptureDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::ScreenCaptureDigitalSource)
						}
						(Field::TrainedAlgorithmicMediaDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::TrainedAlgorithmicMediaDigitalSource)
						}
						(Field::VirtualRecordingDigitalSource, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(IptcDigitalSourceEnumeration::VirtualRecordingDigitalSource)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AlgorithmicMediaDigitalSource",
				"AlgorithmicallyEnhancedDigitalSource",
				"CompositeCaptureDigitalSource",
				"CompositeDigitalSource",
				"CompositeSyntheticDigitalSource",
				"CompositeWithTrainedAlgorithmicMediaDigitalSource",
				"DataDrivenMediaDigitalSource",
				"DigitalArtDigitalSource",
				"DigitalCaptureDigitalSource",
				"MinorHumanEditsDigitalSource",
				"MultiFrameComputationalCaptureDigitalSource",
				"NegativeFilmDigitalSource",
				"PositiveFilmDigitalSource",
				"PrintDigitalSource",
				"ScreenCaptureDigitalSource",
				"TrainedAlgorithmicMediaDigitalSource",
				"VirtualRecordingDigitalSource",
			];
			deserializer.deserialize_enum(
				"IptcDigitalSourceEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}
