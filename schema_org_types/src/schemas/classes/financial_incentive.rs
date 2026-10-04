use super::*;
/// <https://schema.org/FinancialIncentive>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FinancialIncentive {
	/// <https://schema.org/areaServed>
	#[cfg_attr(feature = "serde", serde(rename = "areaServed"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#area_served: Vec<AreaServedProperty>,
	/// <https://schema.org/eligibleWithSupplier>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleWithSupplier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_with_supplier: Vec<EligibleWithSupplierProperty>,
	/// <https://schema.org/incentiveAmount>
	#[cfg_attr(feature = "serde", serde(rename = "incentiveAmount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#incentive_amount: Vec<IncentiveAmountProperty>,
	/// <https://schema.org/incentiveStatus>
	#[cfg_attr(feature = "serde", serde(rename = "incentiveStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#incentive_status: Vec<IncentiveStatusProperty>,
	/// <https://schema.org/incentiveType>
	#[cfg_attr(feature = "serde", serde(rename = "incentiveType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#incentive_type: Vec<IncentiveTypeProperty>,
	/// <https://schema.org/incentivizedItem>
	#[cfg_attr(feature = "serde", serde(rename = "incentivizedItem"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#incentivized_item: Vec<IncentivizedItemProperty>,
	/// <https://schema.org/incomeLimit>
	#[cfg_attr(feature = "serde", serde(rename = "incomeLimit"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#income_limit: Vec<IncomeLimitProperty>,
	/// <https://schema.org/provider>
	#[cfg_attr(feature = "serde", serde(rename = "provider"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#provider: Vec<ProviderProperty>,
	/// <https://schema.org/publisher>
	#[cfg_attr(feature = "serde", serde(rename = "publisher"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#publisher: Vec<PublisherProperty>,
	/// <https://schema.org/purchasePriceLimit>
	#[cfg_attr(feature = "serde", serde(rename = "purchasePriceLimit"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#purchase_price_limit: Vec<PurchasePriceLimitProperty>,
	/// <https://schema.org/purchaseType>
	#[cfg_attr(feature = "serde", serde(rename = "purchaseType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#purchase_type: Vec<PurchaseTypeProperty>,
	/// <https://schema.org/qualifiedExpense>
	#[cfg_attr(feature = "serde", serde(rename = "qualifiedExpense"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#qualified_expense: Vec<QualifiedExpenseProperty>,
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
/// This trait is for properties from <https://schema.org/FinancialIncentive>.
pub trait FinancialIncentiveTrait {
	/// Get <https://schema.org/areaServed> from [`Self`] as borrowed slice.
	fn get_area_served(&self) -> &[AreaServedProperty];
	/// Take <https://schema.org/areaServed> from [`Self`] as owned vector.
	fn take_area_served(&mut self) -> Vec<AreaServedProperty>;
	/// Get <https://schema.org/eligibleWithSupplier> from [`Self`] as borrowed slice.
	fn get_eligible_with_supplier(&self) -> &[EligibleWithSupplierProperty];
	/// Take <https://schema.org/eligibleWithSupplier> from [`Self`] as owned vector.
	fn take_eligible_with_supplier(&mut self) -> Vec<EligibleWithSupplierProperty>;
	/// Get <https://schema.org/incentiveAmount> from [`Self`] as borrowed slice.
	fn get_incentive_amount(&self) -> &[IncentiveAmountProperty];
	/// Take <https://schema.org/incentiveAmount> from [`Self`] as owned vector.
	fn take_incentive_amount(&mut self) -> Vec<IncentiveAmountProperty>;
	/// Get <https://schema.org/incentiveStatus> from [`Self`] as borrowed slice.
	fn get_incentive_status(&self) -> &[IncentiveStatusProperty];
	/// Take <https://schema.org/incentiveStatus> from [`Self`] as owned vector.
	fn take_incentive_status(&mut self) -> Vec<IncentiveStatusProperty>;
	/// Get <https://schema.org/incentiveType> from [`Self`] as borrowed slice.
	fn get_incentive_type(&self) -> &[IncentiveTypeProperty];
	/// Take <https://schema.org/incentiveType> from [`Self`] as owned vector.
	fn take_incentive_type(&mut self) -> Vec<IncentiveTypeProperty>;
	/// Get <https://schema.org/incentivizedItem> from [`Self`] as borrowed slice.
	fn get_incentivized_item(&self) -> &[IncentivizedItemProperty];
	/// Take <https://schema.org/incentivizedItem> from [`Self`] as owned vector.
	fn take_incentivized_item(&mut self) -> Vec<IncentivizedItemProperty>;
	/// Get <https://schema.org/incomeLimit> from [`Self`] as borrowed slice.
	fn get_income_limit(&self) -> &[IncomeLimitProperty];
	/// Take <https://schema.org/incomeLimit> from [`Self`] as owned vector.
	fn take_income_limit(&mut self) -> Vec<IncomeLimitProperty>;
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn get_provider(&self) -> &[ProviderProperty];
	/// Take <https://schema.org/provider> from [`Self`] as owned vector.
	fn take_provider(&mut self) -> Vec<ProviderProperty>;
	/// Get <https://schema.org/publisher> from [`Self`] as borrowed slice.
	fn get_publisher(&self) -> &[PublisherProperty];
	/// Take <https://schema.org/publisher> from [`Self`] as owned vector.
	fn take_publisher(&mut self) -> Vec<PublisherProperty>;
	/// Get <https://schema.org/purchasePriceLimit> from [`Self`] as borrowed slice.
	fn get_purchase_price_limit(&self) -> &[PurchasePriceLimitProperty];
	/// Take <https://schema.org/purchasePriceLimit> from [`Self`] as owned vector.
	fn take_purchase_price_limit(&mut self) -> Vec<PurchasePriceLimitProperty>;
	/// Get <https://schema.org/purchaseType> from [`Self`] as borrowed slice.
	fn get_purchase_type(&self) -> &[PurchaseTypeProperty];
	/// Take <https://schema.org/purchaseType> from [`Self`] as owned vector.
	fn take_purchase_type(&mut self) -> Vec<PurchaseTypeProperty>;
	/// Get <https://schema.org/qualifiedExpense> from [`Self`] as borrowed slice.
	fn get_qualified_expense(&self) -> &[QualifiedExpenseProperty];
	/// Take <https://schema.org/qualifiedExpense> from [`Self`] as owned vector.
	fn take_qualified_expense(&mut self) -> Vec<QualifiedExpenseProperty>;
	/// Get <https://schema.org/validFrom> from [`Self`] as borrowed slice.
	fn get_valid_from(&self) -> &[ValidFromProperty];
	/// Take <https://schema.org/validFrom> from [`Self`] as owned vector.
	fn take_valid_from(&mut self) -> Vec<ValidFromProperty>;
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn get_valid_through(&self) -> &[ValidThroughProperty];
	/// Take <https://schema.org/validThrough> from [`Self`] as owned vector.
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty>;
}
impl FinancialIncentiveTrait for FinancialIncentive {
	fn get_area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn take_area_served(&mut self) -> Vec<AreaServedProperty> {
		std::mem::take(&mut self.r#area_served)
	}
	fn get_eligible_with_supplier(&self) -> &[EligibleWithSupplierProperty] {
		self.r#eligible_with_supplier.as_slice()
	}
	fn take_eligible_with_supplier(&mut self) -> Vec<EligibleWithSupplierProperty> {
		std::mem::take(&mut self.r#eligible_with_supplier)
	}
	fn get_incentive_amount(&self) -> &[IncentiveAmountProperty] {
		self.r#incentive_amount.as_slice()
	}
	fn take_incentive_amount(&mut self) -> Vec<IncentiveAmountProperty> {
		std::mem::take(&mut self.r#incentive_amount)
	}
	fn get_incentive_status(&self) -> &[IncentiveStatusProperty] {
		self.r#incentive_status.as_slice()
	}
	fn take_incentive_status(&mut self) -> Vec<IncentiveStatusProperty> {
		std::mem::take(&mut self.r#incentive_status)
	}
	fn get_incentive_type(&self) -> &[IncentiveTypeProperty] {
		self.r#incentive_type.as_slice()
	}
	fn take_incentive_type(&mut self) -> Vec<IncentiveTypeProperty> {
		std::mem::take(&mut self.r#incentive_type)
	}
	fn get_incentivized_item(&self) -> &[IncentivizedItemProperty] {
		self.r#incentivized_item.as_slice()
	}
	fn take_incentivized_item(&mut self) -> Vec<IncentivizedItemProperty> {
		std::mem::take(&mut self.r#incentivized_item)
	}
	fn get_income_limit(&self) -> &[IncomeLimitProperty] {
		self.r#income_limit.as_slice()
	}
	fn take_income_limit(&mut self) -> Vec<IncomeLimitProperty> {
		std::mem::take(&mut self.r#income_limit)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_publisher(&self) -> &[PublisherProperty] {
		self.r#publisher.as_slice()
	}
	fn take_publisher(&mut self) -> Vec<PublisherProperty> {
		std::mem::take(&mut self.r#publisher)
	}
	fn get_purchase_price_limit(&self) -> &[PurchasePriceLimitProperty] {
		self.r#purchase_price_limit.as_slice()
	}
	fn take_purchase_price_limit(&mut self) -> Vec<PurchasePriceLimitProperty> {
		std::mem::take(&mut self.r#purchase_price_limit)
	}
	fn get_purchase_type(&self) -> &[PurchaseTypeProperty] {
		self.r#purchase_type.as_slice()
	}
	fn take_purchase_type(&mut self) -> Vec<PurchaseTypeProperty> {
		std::mem::take(&mut self.r#purchase_type)
	}
	fn get_qualified_expense(&self) -> &[QualifiedExpenseProperty] {
		self.r#qualified_expense.as_slice()
	}
	fn take_qualified_expense(&mut self) -> Vec<QualifiedExpenseProperty> {
		std::mem::take(&mut self.r#qualified_expense)
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
}
impl ThingTrait for FinancialIncentive {
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
