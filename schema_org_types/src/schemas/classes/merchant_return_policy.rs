use super::*;
/// <https://schema.org/MerchantReturnPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MerchantReturnPolicy {
	/// <https://schema.org/additionalProperty>
	#[cfg_attr(feature = "serde", serde(rename = "additionalProperty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	/// <https://schema.org/applicableCountry>
	#[cfg_attr(feature = "serde", serde(rename = "applicableCountry"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#applicable_country: Vec<ApplicableCountryProperty>,
	/// <https://schema.org/customerRemorseReturnFees>
	#[cfg_attr(feature = "serde", serde(rename = "customerRemorseReturnFees"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#customer_remorse_return_fees: Vec<CustomerRemorseReturnFeesProperty>,
	/// <https://schema.org/customerRemorseReturnLabelSource>
	#[cfg_attr(feature = "serde", serde(rename = "customerRemorseReturnLabelSource"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#customer_remorse_return_label_source: Vec<CustomerRemorseReturnLabelSourceProperty>,
	/// <https://schema.org/customerRemorseReturnShippingFeesAmount>
	#[cfg_attr(
		feature = "serde",
		serde(rename = "customerRemorseReturnShippingFeesAmount")
	)]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#customer_remorse_return_shipping_fees_amount:
		Vec<CustomerRemorseReturnShippingFeesAmountProperty>,
	/// <https://schema.org/inStoreReturnsOffered>
	#[cfg_attr(feature = "serde", serde(rename = "inStoreReturnsOffered"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#in_store_returns_offered: Vec<InStoreReturnsOfferedProperty>,
	/// <https://schema.org/itemCondition>
	#[cfg_attr(feature = "serde", serde(rename = "itemCondition"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_condition: Vec<ItemConditionProperty>,
	/// <https://schema.org/itemDefectReturnFees>
	#[cfg_attr(feature = "serde", serde(rename = "itemDefectReturnFees"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_defect_return_fees: Vec<ItemDefectReturnFeesProperty>,
	/// <https://schema.org/itemDefectReturnLabelSource>
	#[cfg_attr(feature = "serde", serde(rename = "itemDefectReturnLabelSource"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_defect_return_label_source: Vec<ItemDefectReturnLabelSourceProperty>,
	/// <https://schema.org/itemDefectReturnShippingFeesAmount>
	#[cfg_attr(
		feature = "serde",
		serde(rename = "itemDefectReturnShippingFeesAmount")
	)]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_defect_return_shipping_fees_amount: Vec<ItemDefectReturnShippingFeesAmountProperty>,
	/// <https://schema.org/merchantReturnDays>
	#[cfg_attr(feature = "serde", serde(rename = "merchantReturnDays"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#merchant_return_days: Vec<MerchantReturnDaysProperty>,
	/// <https://schema.org/merchantReturnLink>
	#[cfg_attr(feature = "serde", serde(rename = "merchantReturnLink"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#merchant_return_link: Vec<MerchantReturnLinkProperty>,
	/// <https://schema.org/refundType>
	#[cfg_attr(feature = "serde", serde(rename = "refundType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#refund_type: Vec<RefundTypeProperty>,
	/// <https://schema.org/restockingFee>
	#[cfg_attr(feature = "serde", serde(rename = "restockingFee"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#restocking_fee: Vec<RestockingFeeProperty>,
	/// <https://schema.org/returnFees>
	#[cfg_attr(feature = "serde", serde(rename = "returnFees"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_fees: Vec<ReturnFeesProperty>,
	/// <https://schema.org/returnLabelSource>
	#[cfg_attr(feature = "serde", serde(rename = "returnLabelSource"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_label_source: Vec<ReturnLabelSourceProperty>,
	/// <https://schema.org/returnMethod>
	#[cfg_attr(feature = "serde", serde(rename = "returnMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_method: Vec<ReturnMethodProperty>,
	/// <https://schema.org/returnPolicyCategory>
	#[cfg_attr(feature = "serde", serde(rename = "returnPolicyCategory"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_policy_category: Vec<ReturnPolicyCategoryProperty>,
	/// <https://schema.org/returnPolicyCountry>
	#[cfg_attr(feature = "serde", serde(rename = "returnPolicyCountry"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_policy_country: Vec<ReturnPolicyCountryProperty>,
	/// <https://schema.org/returnPolicySeasonalOverride>
	#[cfg_attr(feature = "serde", serde(rename = "returnPolicySeasonalOverride"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_policy_seasonal_override: Vec<ReturnPolicySeasonalOverrideProperty>,
	/// <https://schema.org/returnShippingFeesAmount>
	#[cfg_attr(feature = "serde", serde(rename = "returnShippingFeesAmount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#return_shipping_fees_amount: Vec<ReturnShippingFeesAmountProperty>,
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
/// This trait is for properties from <https://schema.org/MerchantReturnPolicy>.
pub trait MerchantReturnPolicyTrait {
	/// Get <https://schema.org/additionalProperty> from [`Self`] as borrowed slice.
	fn r#additional_property(&self) -> &[AdditionalPropertyProperty];
	/// Get <https://schema.org/applicableCountry> from [`Self`] as borrowed slice.
	fn r#applicable_country(&self) -> &[ApplicableCountryProperty];
	/// Get <https://schema.org/customerRemorseReturnFees> from [`Self`] as borrowed slice.
	fn r#customer_remorse_return_fees(&self) -> &[CustomerRemorseReturnFeesProperty];
	/// Get <https://schema.org/customerRemorseReturnLabelSource> from [`Self`] as borrowed slice.
	fn r#customer_remorse_return_label_source(&self)
	-> &[CustomerRemorseReturnLabelSourceProperty];
	/// Get <https://schema.org/customerRemorseReturnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn r#customer_remorse_return_shipping_fees_amount(
		&self,
	) -> &[CustomerRemorseReturnShippingFeesAmountProperty];
	/// Get <https://schema.org/inStoreReturnsOffered> from [`Self`] as borrowed slice.
	fn r#in_store_returns_offered(&self) -> &[InStoreReturnsOfferedProperty];
	/// Get <https://schema.org/itemCondition> from [`Self`] as borrowed slice.
	fn r#item_condition(&self) -> &[ItemConditionProperty];
	/// Get <https://schema.org/itemDefectReturnFees> from [`Self`] as borrowed slice.
	fn r#item_defect_return_fees(&self) -> &[ItemDefectReturnFeesProperty];
	/// Get <https://schema.org/itemDefectReturnLabelSource> from [`Self`] as borrowed slice.
	fn r#item_defect_return_label_source(&self) -> &[ItemDefectReturnLabelSourceProperty];
	/// Get <https://schema.org/itemDefectReturnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn r#item_defect_return_shipping_fees_amount(
		&self,
	) -> &[ItemDefectReturnShippingFeesAmountProperty];
	/// Get <https://schema.org/merchantReturnDays> from [`Self`] as borrowed slice.
	fn r#merchant_return_days(&self) -> &[MerchantReturnDaysProperty];
	/// Get <https://schema.org/merchantReturnLink> from [`Self`] as borrowed slice.
	fn r#merchant_return_link(&self) -> &[MerchantReturnLinkProperty];
	/// Get <https://schema.org/refundType> from [`Self`] as borrowed slice.
	fn r#refund_type(&self) -> &[RefundTypeProperty];
	/// Get <https://schema.org/restockingFee> from [`Self`] as borrowed slice.
	fn r#restocking_fee(&self) -> &[RestockingFeeProperty];
	/// Get <https://schema.org/returnFees> from [`Self`] as borrowed slice.
	fn r#return_fees(&self) -> &[ReturnFeesProperty];
	/// Get <https://schema.org/returnLabelSource> from [`Self`] as borrowed slice.
	fn r#return_label_source(&self) -> &[ReturnLabelSourceProperty];
	/// Get <https://schema.org/returnMethod> from [`Self`] as borrowed slice.
	fn r#return_method(&self) -> &[ReturnMethodProperty];
	/// Get <https://schema.org/returnPolicyCategory> from [`Self`] as borrowed slice.
	fn r#return_policy_category(&self) -> &[ReturnPolicyCategoryProperty];
	/// Get <https://schema.org/returnPolicyCountry> from [`Self`] as borrowed slice.
	fn r#return_policy_country(&self) -> &[ReturnPolicyCountryProperty];
	/// Get <https://schema.org/returnPolicySeasonalOverride> from [`Self`] as borrowed slice.
	fn r#return_policy_seasonal_override(&self) -> &[ReturnPolicySeasonalOverrideProperty];
	/// Get <https://schema.org/returnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn r#return_shipping_fees_amount(&self) -> &[ReturnShippingFeesAmountProperty];
	/// Get <https://schema.org/validForMemberTier> from [`Self`] as borrowed slice.
	fn r#valid_for_member_tier(&self) -> &[ValidForMemberTierProperty];
}
impl MerchantReturnPolicyTrait for MerchantReturnPolicy {
	fn r#additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn r#applicable_country(&self) -> &[ApplicableCountryProperty] {
		self.r#applicable_country.as_slice()
	}
	fn r#customer_remorse_return_fees(&self) -> &[CustomerRemorseReturnFeesProperty] {
		self.r#customer_remorse_return_fees.as_slice()
	}
	fn r#customer_remorse_return_label_source(
		&self,
	) -> &[CustomerRemorseReturnLabelSourceProperty] {
		self.r#customer_remorse_return_label_source.as_slice()
	}
	fn r#customer_remorse_return_shipping_fees_amount(
		&self,
	) -> &[CustomerRemorseReturnShippingFeesAmountProperty] {
		self.r#customer_remorse_return_shipping_fees_amount
			.as_slice()
	}
	fn r#in_store_returns_offered(&self) -> &[InStoreReturnsOfferedProperty] {
		self.r#in_store_returns_offered.as_slice()
	}
	fn r#item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn r#item_defect_return_fees(&self) -> &[ItemDefectReturnFeesProperty] {
		self.r#item_defect_return_fees.as_slice()
	}
	fn r#item_defect_return_label_source(&self) -> &[ItemDefectReturnLabelSourceProperty] {
		self.r#item_defect_return_label_source.as_slice()
	}
	fn r#item_defect_return_shipping_fees_amount(
		&self,
	) -> &[ItemDefectReturnShippingFeesAmountProperty] {
		self.r#item_defect_return_shipping_fees_amount.as_slice()
	}
	fn r#merchant_return_days(&self) -> &[MerchantReturnDaysProperty] {
		self.r#merchant_return_days.as_slice()
	}
	fn r#merchant_return_link(&self) -> &[MerchantReturnLinkProperty] {
		self.r#merchant_return_link.as_slice()
	}
	fn r#refund_type(&self) -> &[RefundTypeProperty] {
		self.r#refund_type.as_slice()
	}
	fn r#restocking_fee(&self) -> &[RestockingFeeProperty] {
		self.r#restocking_fee.as_slice()
	}
	fn r#return_fees(&self) -> &[ReturnFeesProperty] {
		self.r#return_fees.as_slice()
	}
	fn r#return_label_source(&self) -> &[ReturnLabelSourceProperty] {
		self.r#return_label_source.as_slice()
	}
	fn r#return_method(&self) -> &[ReturnMethodProperty] {
		self.r#return_method.as_slice()
	}
	fn r#return_policy_category(&self) -> &[ReturnPolicyCategoryProperty] {
		self.r#return_policy_category.as_slice()
	}
	fn r#return_policy_country(&self) -> &[ReturnPolicyCountryProperty] {
		self.r#return_policy_country.as_slice()
	}
	fn r#return_policy_seasonal_override(&self) -> &[ReturnPolicySeasonalOverrideProperty] {
		self.r#return_policy_seasonal_override.as_slice()
	}
	fn r#return_shipping_fees_amount(&self) -> &[ReturnShippingFeesAmountProperty] {
		self.r#return_shipping_fees_amount.as_slice()
	}
	fn r#valid_for_member_tier(&self) -> &[ValidForMemberTierProperty] {
		self.r#valid_for_member_tier.as_slice()
	}
}
impl ThingTrait for MerchantReturnPolicy {
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
