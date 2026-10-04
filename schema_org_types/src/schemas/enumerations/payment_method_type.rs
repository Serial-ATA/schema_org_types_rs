/// <https://schema.org/PaymentMethodType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PaymentMethodType {
	/// <https://schema.org/ByBankTransferInAdvance>
	ByBankTransferInAdvance,
	/// <https://schema.org/ByInvoice>
	ByInvoice,
	/// <https://schema.org/COD>
	Cod,
	/// <https://schema.org/Cash>
	Cash,
	/// <https://schema.org/CheckInAdvance>
	CheckInAdvance,
	/// <https://schema.org/DirectDebit>
	DirectDebit,
	/// <https://schema.org/InStorePrepay>
	InStorePrepay,
	/// <https://schema.org/PhoneCarrierPayment>
	PhoneCarrierPayment,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for PaymentMethodType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PaymentMethodType::ByBankTransferInAdvance => serializer.serialize_unit_variant(
					"PaymentMethodType",
					0u32,
					"ByBankTransferInAdvance",
				),
				PaymentMethodType::ByInvoice => {
					serializer.serialize_unit_variant("PaymentMethodType", 1u32, "ByInvoice")
				}
				PaymentMethodType::Cod => {
					serializer.serialize_unit_variant("PaymentMethodType", 2u32, "Cod")
				}
				PaymentMethodType::Cash => {
					serializer.serialize_unit_variant("PaymentMethodType", 3u32, "Cash")
				}
				PaymentMethodType::CheckInAdvance => {
					serializer.serialize_unit_variant("PaymentMethodType", 4u32, "CheckInAdvance")
				}
				PaymentMethodType::DirectDebit => {
					serializer.serialize_unit_variant("PaymentMethodType", 5u32, "DirectDebit")
				}
				PaymentMethodType::InStorePrepay => {
					serializer.serialize_unit_variant("PaymentMethodType", 6u32, "InStorePrepay")
				}
				PaymentMethodType::PhoneCarrierPayment => serializer.serialize_unit_variant(
					"PaymentMethodType",
					7u32,
					"PhoneCarrierPayment",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for PaymentMethodType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ByBankTransferInAdvance,
				ByInvoice,
				Cod,
				Cash,
				CheckInAdvance,
				DirectDebit,
				InStorePrepay,
				PhoneCarrierPayment,
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
						"ByBankTransferInAdvance" => Ok(Field::ByBankTransferInAdvance),
						"ByInvoice" => Ok(Field::ByInvoice),
						"Cod" => Ok(Field::Cod),
						"Cash" => Ok(Field::Cash),
						"CheckInAdvance" => Ok(Field::CheckInAdvance),
						"DirectDebit" => Ok(Field::DirectDebit),
						"InStorePrepay" => Ok(Field::InStorePrepay),
						"PhoneCarrierPayment" => Ok(Field::PhoneCarrierPayment),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ByBankTransferInAdvance" => Ok(Field::ByBankTransferInAdvance),
						b"ByInvoice" => Ok(Field::ByInvoice),
						b"Cod" => Ok(Field::Cod),
						b"Cash" => Ok(Field::Cash),
						b"CheckInAdvance" => Ok(Field::CheckInAdvance),
						b"DirectDebit" => Ok(Field::DirectDebit),
						b"InStorePrepay" => Ok(Field::InStorePrepay),
						b"PhoneCarrierPayment" => Ok(Field::PhoneCarrierPayment),
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
				type Value = PaymentMethodType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PaymentMethodType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ByBankTransferInAdvance, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::ByBankTransferInAdvance)
						}
						(Field::ByInvoice, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::ByInvoice)
						}
						(Field::Cod, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::Cod)
						}
						(Field::Cash, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::Cash)
						}
						(Field::CheckInAdvance, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::CheckInAdvance)
						}
						(Field::DirectDebit, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::DirectDebit)
						}
						(Field::InStorePrepay, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::InStorePrepay)
						}
						(Field::PhoneCarrierPayment, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentMethodType::PhoneCarrierPayment)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ByBankTransferInAdvance",
				"ByInvoice",
				"Cod",
				"Cash",
				"CheckInAdvance",
				"DirectDebit",
				"InStorePrepay",
				"PhoneCarrierPayment",
			];
			deserializer.deserialize_enum("PaymentMethodType", VARIANTS, EnumerationVisitor)
		}
	}
}
