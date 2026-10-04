use super::*;
/// <https://schema.org/Ticket>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Ticket {
	/// <https://schema.org/dateIssued>
	#[cfg_attr(feature = "serde", serde(rename = "dateIssued"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#date_issued: Vec<DateIssuedProperty>,
	/// <https://schema.org/issuedBy>
	#[cfg_attr(feature = "serde", serde(rename = "issuedBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#issued_by: Vec<IssuedByProperty>,
	/// <https://schema.org/priceCurrency>
	#[cfg_attr(feature = "serde", serde(rename = "priceCurrency"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price_currency: Vec<PriceCurrencyProperty>,
	/// <https://schema.org/ticketNumber>
	#[cfg_attr(feature = "serde", serde(rename = "ticketNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ticket_number: Vec<TicketNumberProperty>,
	/// <https://schema.org/ticketToken>
	#[cfg_attr(feature = "serde", serde(rename = "ticketToken"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ticket_token: Vec<TicketTokenProperty>,
	/// <https://schema.org/ticketedSeat>
	#[cfg_attr(feature = "serde", serde(rename = "ticketedSeat"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ticketed_seat: Vec<TicketedSeatProperty>,
	/// <https://schema.org/totalPrice>
	#[cfg_attr(feature = "serde", serde(rename = "totalPrice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#total_price: Vec<TotalPriceProperty>,
	/// <https://schema.org/underName>
	#[cfg_attr(feature = "serde", serde(rename = "underName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#under_name: Vec<UnderNameProperty>,
	/// <https://schema.org/additionalType>
	#[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	/// <https://schema.org/alternateName>
	#[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	/// <https://schema.org/description>
	#[cfg_attr(feature = "serde", serde(rename = "description"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#description: Vec<DescriptionProperty>,
	/// <https://schema.org/disambiguatingDescription>
	#[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	/// <https://schema.org/identifier>
	#[cfg_attr(feature = "serde", serde(rename = "identifier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#identifier: Vec<IdentifierProperty>,
	/// <https://schema.org/image>
	#[cfg_attr(feature = "serde", serde(rename = "image"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#image: Vec<ImageProperty>,
	/// <https://schema.org/mainEntityOfPage>
	#[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	/// <https://schema.org/name>
	#[cfg_attr(feature = "serde", serde(rename = "name"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#name: Vec<NameProperty>,
	/// <https://schema.org/owner>
	#[cfg_attr(feature = "serde", serde(rename = "owner"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#owner: Vec<OwnerProperty>,
	/// <https://schema.org/potentialAction>
	#[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#potential_action: Vec<PotentialActionProperty>,
	/// <https://schema.org/sameAs>
	#[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#same_as: Vec<SameAsProperty>,
	/// <https://schema.org/subjectOf>
	#[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#subject_of: Vec<SubjectOfProperty>,
	/// <https://schema.org/url>
	#[cfg_attr(feature = "serde", serde(rename = "url"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#url: Vec<UrlProperty>,
}
/// This trait is for properties from <https://schema.org/Ticket>.
pub trait TicketTrait {
	/// Get <https://schema.org/dateIssued> from [`Self`] as borrowed slice.
	fn get_date_issued(&self) -> &[DateIssuedProperty];
	/// Take <https://schema.org/dateIssued> from [`Self`] as owned vector.
	fn take_date_issued(&mut self) -> Vec<DateIssuedProperty>;
	/// Get <https://schema.org/issuedBy> from [`Self`] as borrowed slice.
	fn get_issued_by(&self) -> &[IssuedByProperty];
	/// Take <https://schema.org/issuedBy> from [`Self`] as owned vector.
	fn take_issued_by(&mut self) -> Vec<IssuedByProperty>;
	/// Get <https://schema.org/priceCurrency> from [`Self`] as borrowed slice.
	fn get_price_currency(&self) -> &[PriceCurrencyProperty];
	/// Take <https://schema.org/priceCurrency> from [`Self`] as owned vector.
	fn take_price_currency(&mut self) -> Vec<PriceCurrencyProperty>;
	/// Get <https://schema.org/ticketNumber> from [`Self`] as borrowed slice.
	fn get_ticket_number(&self) -> &[TicketNumberProperty];
	/// Take <https://schema.org/ticketNumber> from [`Self`] as owned vector.
	fn take_ticket_number(&mut self) -> Vec<TicketNumberProperty>;
	/// Get <https://schema.org/ticketToken> from [`Self`] as borrowed slice.
	fn get_ticket_token(&self) -> &[TicketTokenProperty];
	/// Take <https://schema.org/ticketToken> from [`Self`] as owned vector.
	fn take_ticket_token(&mut self) -> Vec<TicketTokenProperty>;
	/// Get <https://schema.org/ticketedSeat> from [`Self`] as borrowed slice.
	fn get_ticketed_seat(&self) -> &[TicketedSeatProperty];
	/// Take <https://schema.org/ticketedSeat> from [`Self`] as owned vector.
	fn take_ticketed_seat(&mut self) -> Vec<TicketedSeatProperty>;
	/// Get <https://schema.org/totalPrice> from [`Self`] as borrowed slice.
	fn get_total_price(&self) -> &[TotalPriceProperty];
	/// Take <https://schema.org/totalPrice> from [`Self`] as owned vector.
	fn take_total_price(&mut self) -> Vec<TotalPriceProperty>;
	/// Get <https://schema.org/underName> from [`Self`] as borrowed slice.
	fn get_under_name(&self) -> &[UnderNameProperty];
	/// Take <https://schema.org/underName> from [`Self`] as owned vector.
	fn take_under_name(&mut self) -> Vec<UnderNameProperty>;
}
impl TicketTrait for Ticket {
	fn get_date_issued(&self) -> &[DateIssuedProperty] {
		self.r#date_issued.as_slice()
	}
	fn take_date_issued(&mut self) -> Vec<DateIssuedProperty> {
		std::mem::take(&mut self.r#date_issued)
	}
	fn get_issued_by(&self) -> &[IssuedByProperty] {
		self.r#issued_by.as_slice()
	}
	fn take_issued_by(&mut self) -> Vec<IssuedByProperty> {
		std::mem::take(&mut self.r#issued_by)
	}
	fn get_price_currency(&self) -> &[PriceCurrencyProperty] {
		self.r#price_currency.as_slice()
	}
	fn take_price_currency(&mut self) -> Vec<PriceCurrencyProperty> {
		std::mem::take(&mut self.r#price_currency)
	}
	fn get_ticket_number(&self) -> &[TicketNumberProperty] {
		self.r#ticket_number.as_slice()
	}
	fn take_ticket_number(&mut self) -> Vec<TicketNumberProperty> {
		std::mem::take(&mut self.r#ticket_number)
	}
	fn get_ticket_token(&self) -> &[TicketTokenProperty] {
		self.r#ticket_token.as_slice()
	}
	fn take_ticket_token(&mut self) -> Vec<TicketTokenProperty> {
		std::mem::take(&mut self.r#ticket_token)
	}
	fn get_ticketed_seat(&self) -> &[TicketedSeatProperty] {
		self.r#ticketed_seat.as_slice()
	}
	fn take_ticketed_seat(&mut self) -> Vec<TicketedSeatProperty> {
		std::mem::take(&mut self.r#ticketed_seat)
	}
	fn get_total_price(&self) -> &[TotalPriceProperty] {
		self.r#total_price.as_slice()
	}
	fn take_total_price(&mut self) -> Vec<TotalPriceProperty> {
		std::mem::take(&mut self.r#total_price)
	}
	fn get_under_name(&self) -> &[UnderNameProperty] {
		self.r#under_name.as_slice()
	}
	fn take_under_name(&mut self) -> Vec<UnderNameProperty> {
		std::mem::take(&mut self.r#under_name)
	}
}
impl ThingTrait for Ticket {
	fn get_additional_type(&self) -> &[AdditionalTypeProperty] {
		self.r#additional_type.as_slice()
	}
	fn take_additional_type(&mut self) -> Vec<AdditionalTypeProperty> {
		std::mem::take(&mut self.r#additional_type)
	}
	fn get_alternate_name(&self) -> &[AlternateNameProperty] {
		self.r#alternate_name.as_slice()
	}
	fn take_alternate_name(&mut self) -> Vec<AlternateNameProperty> {
		std::mem::take(&mut self.r#alternate_name)
	}
	fn get_description(&self) -> &[DescriptionProperty] {
		self.r#description.as_slice()
	}
	fn take_description(&mut self) -> Vec<DescriptionProperty> {
		std::mem::take(&mut self.r#description)
	}
	fn get_disambiguating_description(&self) -> &[DisambiguatingDescriptionProperty] {
		self.r#disambiguating_description.as_slice()
	}
	fn take_disambiguating_description(&mut self) -> Vec<DisambiguatingDescriptionProperty> {
		std::mem::take(&mut self.r#disambiguating_description)
	}
	fn get_identifier(&self) -> &[IdentifierProperty] {
		self.r#identifier.as_slice()
	}
	fn take_identifier(&mut self) -> Vec<IdentifierProperty> {
		std::mem::take(&mut self.r#identifier)
	}
	fn get_image(&self) -> &[ImageProperty] {
		self.r#image.as_slice()
	}
	fn take_image(&mut self) -> Vec<ImageProperty> {
		std::mem::take(&mut self.r#image)
	}
	fn get_main_entity_of_page(&self) -> &[MainEntityOfPageProperty] {
		self.r#main_entity_of_page.as_slice()
	}
	fn take_main_entity_of_page(&mut self) -> Vec<MainEntityOfPageProperty> {
		std::mem::take(&mut self.r#main_entity_of_page)
	}
	fn get_name(&self) -> &[NameProperty] {
		self.r#name.as_slice()
	}
	fn take_name(&mut self) -> Vec<NameProperty> {
		std::mem::take(&mut self.r#name)
	}
	fn get_owner(&self) -> &[OwnerProperty] {
		self.r#owner.as_slice()
	}
	fn take_owner(&mut self) -> Vec<OwnerProperty> {
		std::mem::take(&mut self.r#owner)
	}
	fn get_potential_action(&self) -> &[PotentialActionProperty] {
		self.r#potential_action.as_slice()
	}
	fn take_potential_action(&mut self) -> Vec<PotentialActionProperty> {
		std::mem::take(&mut self.r#potential_action)
	}
	fn get_same_as(&self) -> &[SameAsProperty] {
		self.r#same_as.as_slice()
	}
	fn take_same_as(&mut self) -> Vec<SameAsProperty> {
		std::mem::take(&mut self.r#same_as)
	}
	fn get_subject_of(&self) -> &[SubjectOfProperty] {
		self.r#subject_of.as_slice()
	}
	fn take_subject_of(&mut self) -> Vec<SubjectOfProperty> {
		std::mem::take(&mut self.r#subject_of)
	}
	fn get_url(&self) -> &[UrlProperty] {
		self.r#url.as_slice()
	}
	fn take_url(&mut self) -> Vec<UrlProperty> {
		std::mem::take(&mut self.r#url)
	}
}
