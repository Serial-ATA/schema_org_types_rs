use super::*;
/// <https://schema.org/Quantity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Quantity(pub String);
impl std::ops::Deref for Quantity {
	type Target = String;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for Quantity {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("Quantity", &self.0)
		}
	}
	impl<'de> Deserialize<'de> for Quantity {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = Quantity;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Quantity")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: String = <String as Deserialize>::deserialize(e)?;
					Ok(Quantity(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(deserializer, "Quantity", DataTypeVisitor)
		}
	}
}
