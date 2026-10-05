use super::*;
/// <https://schema.org/UnitPriceSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct UnitPriceSpecification {
	/// <https://schema.org/billingDuration>
	#[cfg_attr(feature = "serde", serde(rename = "billingDuration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#billing_duration: Vec<BillingDurationProperty>,
	/// <https://schema.org/billingIncrement>
	#[cfg_attr(feature = "serde", serde(rename = "billingIncrement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#billing_increment: Vec<BillingIncrementProperty>,
	/// <https://schema.org/billingStart>
	#[cfg_attr(feature = "serde", serde(rename = "billingStart"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#billing_start: Vec<BillingStartProperty>,
	/// <https://schema.org/priceComponentType>
	#[cfg_attr(feature = "serde", serde(rename = "priceComponentType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price_component_type: Vec<PriceComponentTypeProperty>,
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
	/// <https://schema.org/referenceQuantity>
	#[cfg_attr(feature = "serde", serde(rename = "referenceQuantity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#reference_quantity: Vec<ReferenceQuantityProperty>,
	/// <https://schema.org/unitCode>
	#[cfg_attr(feature = "serde", serde(rename = "unitCode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#unit_code: Vec<UnitCodeProperty>,
	/// <https://schema.org/unitText>
	#[cfg_attr(feature = "serde", serde(rename = "unitText"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#unit_text: Vec<UnitTextProperty>,
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
/// This trait is for properties from <https://schema.org/UnitPriceSpecification>.
pub trait UnitPriceSpecificationTrait {
	/// Get <https://schema.org/billingDuration> from [`Self`] as borrowed slice.
	fn r#billing_duration(&self) -> &[BillingDurationProperty];
	/// Get <https://schema.org/billingIncrement> from [`Self`] as borrowed slice.
	fn r#billing_increment(&self) -> &[BillingIncrementProperty];
	/// Get <https://schema.org/billingStart> from [`Self`] as borrowed slice.
	fn r#billing_start(&self) -> &[BillingStartProperty];
	/// Get <https://schema.org/priceComponentType> from [`Self`] as borrowed slice.
	fn r#price_component_type(&self) -> &[PriceComponentTypeProperty];
	/// Get <https://schema.org/priceType> from [`Self`] as borrowed slice.
	fn r#price_type(&self) -> &[PriceTypeProperty];
	/// Get <https://schema.org/referenceQuantity> from [`Self`] as borrowed slice.
	fn r#reference_quantity(&self) -> &[ReferenceQuantityProperty];
	/// Get <https://schema.org/unitCode> from [`Self`] as borrowed slice.
	fn r#unit_code(&self) -> &[UnitCodeProperty];
	/// Get <https://schema.org/unitText> from [`Self`] as borrowed slice.
	fn r#unit_text(&self) -> &[UnitTextProperty];
}
impl UnitPriceSpecificationTrait for UnitPriceSpecification {
	fn r#billing_duration(&self) -> &[BillingDurationProperty] {
		self.r#billing_duration.as_slice()
	}
	fn r#billing_increment(&self) -> &[BillingIncrementProperty] {
		self.r#billing_increment.as_slice()
	}
	fn r#billing_start(&self) -> &[BillingStartProperty] {
		self.r#billing_start.as_slice()
	}
	fn r#price_component_type(&self) -> &[PriceComponentTypeProperty] {
		self.r#price_component_type.as_slice()
	}
	fn r#price_type(&self) -> &[PriceTypeProperty] {
		self.r#price_type.as_slice()
	}
	fn r#reference_quantity(&self) -> &[ReferenceQuantityProperty] {
		self.r#reference_quantity.as_slice()
	}
	fn r#unit_code(&self) -> &[UnitCodeProperty] {
		self.r#unit_code.as_slice()
	}
	fn r#unit_text(&self) -> &[UnitTextProperty] {
		self.r#unit_text.as_slice()
	}
}
impl PriceSpecificationTrait for UnitPriceSpecification {
	fn r#eligible_quantity(&self) -> &[EligibleQuantityProperty] {
		self.r#eligible_quantity.as_slice()
	}
	fn r#eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty] {
		self.r#eligible_transaction_volume.as_slice()
	}
	fn r#max_price(&self) -> &[MaxPriceProperty] {
		self.r#max_price.as_slice()
	}
	fn r#membership_points_earned(&self) -> &[MembershipPointsEarnedProperty] {
		self.r#membership_points_earned.as_slice()
	}
	fn r#min_price(&self) -> &[MinPriceProperty] {
		self.r#min_price.as_slice()
	}
	fn r#price(&self) -> &[PriceProperty] {
		self.r#price.as_slice()
	}
	fn r#price_currency(&self) -> &[PriceCurrencyProperty] {
		self.r#price_currency.as_slice()
	}
	fn r#valid_for_member_tier(&self) -> &[ValidForMemberTierProperty] {
		self.r#valid_for_member_tier.as_slice()
	}
	fn r#valid_from(&self) -> &[ValidFromProperty] {
		self.r#valid_from.as_slice()
	}
	fn r#valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn r#value_added_tax_included(&self) -> &[ValueAddedTaxIncludedProperty] {
		self.r#value_added_tax_included.as_slice()
	}
}
impl StructuredValueTrait for UnitPriceSpecification {}
impl ThingTrait for UnitPriceSpecification {
	fn r#additional_type(&self) -> &[AdditionalTypeProperty] {
		self.r#additional_type.as_slice()
	}
	fn r#alternate_name(&self) -> &[AlternateNameProperty] {
		self.r#alternate_name.as_slice()
	}
	fn r#description(&self) -> &[DescriptionProperty] {
		self.r#description.as_slice()
	}
	fn r#disambiguating_description(&self) -> &[DisambiguatingDescriptionProperty] {
		self.r#disambiguating_description.as_slice()
	}
	fn r#identifier(&self) -> &[IdentifierProperty] {
		self.r#identifier.as_slice()
	}
	fn r#image(&self) -> &[ImageProperty] {
		self.r#image.as_slice()
	}
	fn r#main_entity_of_page(&self) -> &[MainEntityOfPageProperty] {
		self.r#main_entity_of_page.as_slice()
	}
	fn r#name(&self) -> &[NameProperty] {
		self.r#name.as_slice()
	}
	fn r#owner(&self) -> &[OwnerProperty] {
		self.r#owner.as_slice()
	}
	fn r#potential_action(&self) -> &[PotentialActionProperty] {
		self.r#potential_action.as_slice()
	}
	fn r#same_as(&self) -> &[SameAsProperty] {
		self.r#same_as.as_slice()
	}
	fn r#subject_of(&self) -> &[SubjectOfProperty] {
		self.r#subject_of.as_slice()
	}
	fn r#url(&self) -> &[UrlProperty] {
		self.r#url.as_slice()
	}
}
