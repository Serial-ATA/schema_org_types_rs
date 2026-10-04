use super::*;
/// <https://schema.org/CompoundPriceSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct CompoundPriceSpecification {
	/// <https://schema.org/priceComponent>
	#[cfg_attr(feature = "serde", serde(rename = "priceComponent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price_component: Vec<PriceComponentProperty>,
	/// <https://schema.org/priceType>
	#[cfg_attr(feature = "serde", serde(rename = "priceType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price_type: Vec<PriceTypeProperty>,
	/// <https://schema.org/eligibleQuantity>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleQuantity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_quantity: Vec<EligibleQuantityProperty>,
	/// <https://schema.org/eligibleTransactionVolume>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleTransactionVolume"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
	/// <https://schema.org/maxPrice>
	#[cfg_attr(feature = "serde", serde(rename = "maxPrice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#max_price: Vec<MaxPriceProperty>,
	/// <https://schema.org/membershipPointsEarned>
	#[cfg_attr(feature = "serde", serde(rename = "membershipPointsEarned"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#membership_points_earned: Vec<MembershipPointsEarnedProperty>,
	/// <https://schema.org/minPrice>
	#[cfg_attr(feature = "serde", serde(rename = "minPrice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#min_price: Vec<MinPriceProperty>,
	/// <https://schema.org/price>
	#[cfg_attr(feature = "serde", serde(rename = "price"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price: Vec<PriceProperty>,
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
	/// <https://schema.org/validForMemberTier>
	#[cfg_attr(feature = "serde", serde(rename = "validForMemberTier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#valid_for_member_tier: Vec<ValidForMemberTierProperty>,
	/// <https://schema.org/validFrom>
	#[cfg_attr(feature = "serde", serde(rename = "validFrom"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#valid_from: Vec<ValidFromProperty>,
	/// <https://schema.org/validThrough>
	#[cfg_attr(feature = "serde", serde(rename = "validThrough"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#valid_through: Vec<ValidThroughProperty>,
	/// <https://schema.org/valueAddedTaxIncluded>
	#[cfg_attr(feature = "serde", serde(rename = "valueAddedTaxIncluded"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_added_tax_included: Vec<ValueAddedTaxIncludedProperty>,
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
/// This trait is for properties from <https://schema.org/CompoundPriceSpecification>.
pub trait CompoundPriceSpecificationTrait {
	/// Get <https://schema.org/priceComponent> from [`Self`] as borrowed slice.
	fn get_price_component(&self) -> &[PriceComponentProperty];
	/// Take <https://schema.org/priceComponent> from [`Self`] as owned vector.
	fn take_price_component(&mut self) -> Vec<PriceComponentProperty>;
	/// Get <https://schema.org/priceType> from [`Self`] as borrowed slice.
	fn get_price_type(&self) -> &[PriceTypeProperty];
	/// Take <https://schema.org/priceType> from [`Self`] as owned vector.
	fn take_price_type(&mut self) -> Vec<PriceTypeProperty>;
}
impl CompoundPriceSpecificationTrait for CompoundPriceSpecification {
	fn get_price_component(&self) -> &[PriceComponentProperty] {
		self.r#price_component.as_slice()
	}
	fn take_price_component(&mut self) -> Vec<PriceComponentProperty> {
		std::mem::take(&mut self.r#price_component)
	}
	fn get_price_type(&self) -> &[PriceTypeProperty] {
		self.r#price_type.as_slice()
	}
	fn take_price_type(&mut self) -> Vec<PriceTypeProperty> {
		std::mem::take(&mut self.r#price_type)
	}
}
impl PriceSpecificationTrait for CompoundPriceSpecification {
	fn get_eligible_quantity(&self) -> &[EligibleQuantityProperty] {
		self.r#eligible_quantity.as_slice()
	}
	fn take_eligible_quantity(&mut self) -> Vec<EligibleQuantityProperty> {
		std::mem::take(&mut self.r#eligible_quantity)
	}
	fn get_eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty] {
		self.r#eligible_transaction_volume.as_slice()
	}
	fn take_eligible_transaction_volume(&mut self) -> Vec<EligibleTransactionVolumeProperty> {
		std::mem::take(&mut self.r#eligible_transaction_volume)
	}
	fn get_max_price(&self) -> &[MaxPriceProperty] {
		self.r#max_price.as_slice()
	}
	fn take_max_price(&mut self) -> Vec<MaxPriceProperty> {
		std::mem::take(&mut self.r#max_price)
	}
	fn get_membership_points_earned(&self) -> &[MembershipPointsEarnedProperty] {
		self.r#membership_points_earned.as_slice()
	}
	fn take_membership_points_earned(&mut self) -> Vec<MembershipPointsEarnedProperty> {
		std::mem::take(&mut self.r#membership_points_earned)
	}
	fn get_min_price(&self) -> &[MinPriceProperty] {
		self.r#min_price.as_slice()
	}
	fn take_min_price(&mut self) -> Vec<MinPriceProperty> {
		std::mem::take(&mut self.r#min_price)
	}
	fn get_price(&self) -> &[PriceProperty] {
		self.r#price.as_slice()
	}
	fn take_price(&mut self) -> Vec<PriceProperty> {
		std::mem::take(&mut self.r#price)
	}
	fn get_price_currency(&self) -> &[PriceCurrencyProperty] {
		self.r#price_currency.as_slice()
	}
	fn take_price_currency(&mut self) -> Vec<PriceCurrencyProperty> {
		std::mem::take(&mut self.r#price_currency)
	}
	fn get_valid_for_member_tier(&self) -> &[ValidForMemberTierProperty] {
		self.r#valid_for_member_tier.as_slice()
	}
	fn take_valid_for_member_tier(&mut self) -> Vec<ValidForMemberTierProperty> {
		std::mem::take(&mut self.r#valid_for_member_tier)
	}
	fn get_valid_from(&self) -> &[ValidFromProperty] {
		self.r#valid_from.as_slice()
	}
	fn take_valid_from(&mut self) -> Vec<ValidFromProperty> {
		std::mem::take(&mut self.r#valid_from)
	}
	fn get_valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty> {
		std::mem::take(&mut self.r#valid_through)
	}
	fn get_value_added_tax_included(&self) -> &[ValueAddedTaxIncludedProperty] {
		self.r#value_added_tax_included.as_slice()
	}
	fn take_value_added_tax_included(&mut self) -> Vec<ValueAddedTaxIncludedProperty> {
		std::mem::take(&mut self.r#value_added_tax_included)
	}
}
impl StructuredValueTrait for CompoundPriceSpecification {}
impl ThingTrait for CompoundPriceSpecification {
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
