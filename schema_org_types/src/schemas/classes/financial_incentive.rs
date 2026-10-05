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
	fn r#area_served(&self) -> &[AreaServedProperty];
	/// Get <https://schema.org/eligibleWithSupplier> from [`Self`] as borrowed slice.
	fn r#eligible_with_supplier(&self) -> &[EligibleWithSupplierProperty];
	/// Get <https://schema.org/incentiveAmount> from [`Self`] as borrowed slice.
	fn r#incentive_amount(&self) -> &[IncentiveAmountProperty];
	/// Get <https://schema.org/incentiveStatus> from [`Self`] as borrowed slice.
	fn r#incentive_status(&self) -> &[IncentiveStatusProperty];
	/// Get <https://schema.org/incentiveType> from [`Self`] as borrowed slice.
	fn r#incentive_type(&self) -> &[IncentiveTypeProperty];
	/// Get <https://schema.org/incentivizedItem> from [`Self`] as borrowed slice.
	fn r#incentivized_item(&self) -> &[IncentivizedItemProperty];
	/// Get <https://schema.org/incomeLimit> from [`Self`] as borrowed slice.
	fn r#income_limit(&self) -> &[IncomeLimitProperty];
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn r#provider(&self) -> &[ProviderProperty];
	/// Get <https://schema.org/publisher> from [`Self`] as borrowed slice.
	fn r#publisher(&self) -> &[PublisherProperty];
	/// Get <https://schema.org/purchasePriceLimit> from [`Self`] as borrowed slice.
	fn r#purchase_price_limit(&self) -> &[PurchasePriceLimitProperty];
	/// Get <https://schema.org/purchaseType> from [`Self`] as borrowed slice.
	fn r#purchase_type(&self) -> &[PurchaseTypeProperty];
	/// Get <https://schema.org/qualifiedExpense> from [`Self`] as borrowed slice.
	fn r#qualified_expense(&self) -> &[QualifiedExpenseProperty];
	/// Get <https://schema.org/validFrom> from [`Self`] as borrowed slice.
	fn r#valid_from(&self) -> &[ValidFromProperty];
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn r#valid_through(&self) -> &[ValidThroughProperty];
}
impl FinancialIncentiveTrait for FinancialIncentive {
	fn r#area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn r#eligible_with_supplier(&self) -> &[EligibleWithSupplierProperty] {
		self.r#eligible_with_supplier.as_slice()
	}
	fn r#incentive_amount(&self) -> &[IncentiveAmountProperty] {
		self.r#incentive_amount.as_slice()
	}
	fn r#incentive_status(&self) -> &[IncentiveStatusProperty] {
		self.r#incentive_status.as_slice()
	}
	fn r#incentive_type(&self) -> &[IncentiveTypeProperty] {
		self.r#incentive_type.as_slice()
	}
	fn r#incentivized_item(&self) -> &[IncentivizedItemProperty] {
		self.r#incentivized_item.as_slice()
	}
	fn r#income_limit(&self) -> &[IncomeLimitProperty] {
		self.r#income_limit.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#publisher(&self) -> &[PublisherProperty] {
		self.r#publisher.as_slice()
	}
	fn r#purchase_price_limit(&self) -> &[PurchasePriceLimitProperty] {
		self.r#purchase_price_limit.as_slice()
	}
	fn r#purchase_type(&self) -> &[PurchaseTypeProperty] {
		self.r#purchase_type.as_slice()
	}
	fn r#qualified_expense(&self) -> &[QualifiedExpenseProperty] {
		self.r#qualified_expense.as_slice()
	}
	fn r#valid_from(&self) -> &[ValidFromProperty] {
		self.r#valid_from.as_slice()
	}
	fn r#valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
}
impl ThingTrait for FinancialIncentive {
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
