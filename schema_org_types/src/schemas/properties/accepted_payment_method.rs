use super::*;
/// <https://schema.org/acceptedPaymentMethod>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AcceptedPaymentMethodProperty {
	/// <https://schema.org/LoanOrCredit>
	LoanOrCredit(LoanOrCredit),
	/// <https://schema.org/PaymentMethod>
	PaymentMethod(PaymentMethod),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
