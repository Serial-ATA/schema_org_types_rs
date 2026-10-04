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
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty];
	/// Take <https://schema.org/additionalProperty> from [`Self`] as owned vector.
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty>;
	/// Get <https://schema.org/applicableCountry> from [`Self`] as borrowed slice.
	fn get_applicable_country(&self) -> &[ApplicableCountryProperty];
	/// Take <https://schema.org/applicableCountry> from [`Self`] as owned vector.
	fn take_applicable_country(&mut self) -> Vec<ApplicableCountryProperty>;
	/// Get <https://schema.org/customerRemorseReturnFees> from [`Self`] as borrowed slice.
	fn get_customer_remorse_return_fees(&self) -> &[CustomerRemorseReturnFeesProperty];
	/// Take <https://schema.org/customerRemorseReturnFees> from [`Self`] as owned vector.
	fn take_customer_remorse_return_fees(&mut self) -> Vec<CustomerRemorseReturnFeesProperty>;
	/// Get <https://schema.org/customerRemorseReturnLabelSource> from [`Self`] as borrowed slice.
	fn get_customer_remorse_return_label_source(
		&self,
	) -> &[CustomerRemorseReturnLabelSourceProperty];
	/// Take <https://schema.org/customerRemorseReturnLabelSource> from [`Self`] as owned vector.
	fn take_customer_remorse_return_label_source(
		&mut self,
	) -> Vec<CustomerRemorseReturnLabelSourceProperty>;
	/// Get <https://schema.org/customerRemorseReturnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn get_customer_remorse_return_shipping_fees_amount(
		&self,
	) -> &[CustomerRemorseReturnShippingFeesAmountProperty];
	/// Take <https://schema.org/customerRemorseReturnShippingFeesAmount> from [`Self`] as owned vector.
	fn take_customer_remorse_return_shipping_fees_amount(
		&mut self,
	) -> Vec<CustomerRemorseReturnShippingFeesAmountProperty>;
	/// Get <https://schema.org/inStoreReturnsOffered> from [`Self`] as borrowed slice.
	fn get_in_store_returns_offered(&self) -> &[InStoreReturnsOfferedProperty];
	/// Take <https://schema.org/inStoreReturnsOffered> from [`Self`] as owned vector.
	fn take_in_store_returns_offered(&mut self) -> Vec<InStoreReturnsOfferedProperty>;
	/// Get <https://schema.org/itemCondition> from [`Self`] as borrowed slice.
	fn get_item_condition(&self) -> &[ItemConditionProperty];
	/// Take <https://schema.org/itemCondition> from [`Self`] as owned vector.
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty>;
	/// Get <https://schema.org/itemDefectReturnFees> from [`Self`] as borrowed slice.
	fn get_item_defect_return_fees(&self) -> &[ItemDefectReturnFeesProperty];
	/// Take <https://schema.org/itemDefectReturnFees> from [`Self`] as owned vector.
	fn take_item_defect_return_fees(&mut self) -> Vec<ItemDefectReturnFeesProperty>;
	/// Get <https://schema.org/itemDefectReturnLabelSource> from [`Self`] as borrowed slice.
	fn get_item_defect_return_label_source(&self) -> &[ItemDefectReturnLabelSourceProperty];
	/// Take <https://schema.org/itemDefectReturnLabelSource> from [`Self`] as owned vector.
	fn take_item_defect_return_label_source(&mut self) -> Vec<ItemDefectReturnLabelSourceProperty>;
	/// Get <https://schema.org/itemDefectReturnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn get_item_defect_return_shipping_fees_amount(
		&self,
	) -> &[ItemDefectReturnShippingFeesAmountProperty];
	/// Take <https://schema.org/itemDefectReturnShippingFeesAmount> from [`Self`] as owned vector.
	fn take_item_defect_return_shipping_fees_amount(
		&mut self,
	) -> Vec<ItemDefectReturnShippingFeesAmountProperty>;
	/// Get <https://schema.org/merchantReturnDays> from [`Self`] as borrowed slice.
	fn get_merchant_return_days(&self) -> &[MerchantReturnDaysProperty];
	/// Take <https://schema.org/merchantReturnDays> from [`Self`] as owned vector.
	fn take_merchant_return_days(&mut self) -> Vec<MerchantReturnDaysProperty>;
	/// Get <https://schema.org/merchantReturnLink> from [`Self`] as borrowed slice.
	fn get_merchant_return_link(&self) -> &[MerchantReturnLinkProperty];
	/// Take <https://schema.org/merchantReturnLink> from [`Self`] as owned vector.
	fn take_merchant_return_link(&mut self) -> Vec<MerchantReturnLinkProperty>;
	/// Get <https://schema.org/refundType> from [`Self`] as borrowed slice.
	fn get_refund_type(&self) -> &[RefundTypeProperty];
	/// Take <https://schema.org/refundType> from [`Self`] as owned vector.
	fn take_refund_type(&mut self) -> Vec<RefundTypeProperty>;
	/// Get <https://schema.org/restockingFee> from [`Self`] as borrowed slice.
	fn get_restocking_fee(&self) -> &[RestockingFeeProperty];
	/// Take <https://schema.org/restockingFee> from [`Self`] as owned vector.
	fn take_restocking_fee(&mut self) -> Vec<RestockingFeeProperty>;
	/// Get <https://schema.org/returnFees> from [`Self`] as borrowed slice.
	fn get_return_fees(&self) -> &[ReturnFeesProperty];
	/// Take <https://schema.org/returnFees> from [`Self`] as owned vector.
	fn take_return_fees(&mut self) -> Vec<ReturnFeesProperty>;
	/// Get <https://schema.org/returnLabelSource> from [`Self`] as borrowed slice.
	fn get_return_label_source(&self) -> &[ReturnLabelSourceProperty];
	/// Take <https://schema.org/returnLabelSource> from [`Self`] as owned vector.
	fn take_return_label_source(&mut self) -> Vec<ReturnLabelSourceProperty>;
	/// Get <https://schema.org/returnMethod> from [`Self`] as borrowed slice.
	fn get_return_method(&self) -> &[ReturnMethodProperty];
	/// Take <https://schema.org/returnMethod> from [`Self`] as owned vector.
	fn take_return_method(&mut self) -> Vec<ReturnMethodProperty>;
	/// Get <https://schema.org/returnPolicyCategory> from [`Self`] as borrowed slice.
	fn get_return_policy_category(&self) -> &[ReturnPolicyCategoryProperty];
	/// Take <https://schema.org/returnPolicyCategory> from [`Self`] as owned vector.
	fn take_return_policy_category(&mut self) -> Vec<ReturnPolicyCategoryProperty>;
	/// Get <https://schema.org/returnPolicyCountry> from [`Self`] as borrowed slice.
	fn get_return_policy_country(&self) -> &[ReturnPolicyCountryProperty];
	/// Take <https://schema.org/returnPolicyCountry> from [`Self`] as owned vector.
	fn take_return_policy_country(&mut self) -> Vec<ReturnPolicyCountryProperty>;
	/// Get <https://schema.org/returnPolicySeasonalOverride> from [`Self`] as borrowed slice.
	fn get_return_policy_seasonal_override(&self) -> &[ReturnPolicySeasonalOverrideProperty];
	/// Take <https://schema.org/returnPolicySeasonalOverride> from [`Self`] as owned vector.
	fn take_return_policy_seasonal_override(&mut self)
	-> Vec<ReturnPolicySeasonalOverrideProperty>;
	/// Get <https://schema.org/returnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn get_return_shipping_fees_amount(&self) -> &[ReturnShippingFeesAmountProperty];
	/// Take <https://schema.org/returnShippingFeesAmount> from [`Self`] as owned vector.
	fn take_return_shipping_fees_amount(&mut self) -> Vec<ReturnShippingFeesAmountProperty>;
	/// Get <https://schema.org/validForMemberTier> from [`Self`] as borrowed slice.
	fn get_valid_for_member_tier(&self) -> &[ValidForMemberTierProperty];
	/// Take <https://schema.org/validForMemberTier> from [`Self`] as owned vector.
	fn take_valid_for_member_tier(&mut self) -> Vec<ValidForMemberTierProperty>;
}
impl MerchantReturnPolicyTrait for MerchantReturnPolicy {
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty> {
		std::mem::take(&mut self.r#additional_property)
	}
	fn get_applicable_country(&self) -> &[ApplicableCountryProperty] {
		self.r#applicable_country.as_slice()
	}
	fn take_applicable_country(&mut self) -> Vec<ApplicableCountryProperty> {
		std::mem::take(&mut self.r#applicable_country)
	}
	fn get_customer_remorse_return_fees(&self) -> &[CustomerRemorseReturnFeesProperty] {
		self.r#customer_remorse_return_fees.as_slice()
	}
	fn take_customer_remorse_return_fees(&mut self) -> Vec<CustomerRemorseReturnFeesProperty> {
		std::mem::take(&mut self.r#customer_remorse_return_fees)
	}
	fn get_customer_remorse_return_label_source(
		&self,
	) -> &[CustomerRemorseReturnLabelSourceProperty] {
		self.r#customer_remorse_return_label_source.as_slice()
	}
	fn take_customer_remorse_return_label_source(
		&mut self,
	) -> Vec<CustomerRemorseReturnLabelSourceProperty> {
		std::mem::take(&mut self.r#customer_remorse_return_label_source)
	}
	fn get_customer_remorse_return_shipping_fees_amount(
		&self,
	) -> &[CustomerRemorseReturnShippingFeesAmountProperty] {
		self.r#customer_remorse_return_shipping_fees_amount
			.as_slice()
	}
	fn take_customer_remorse_return_shipping_fees_amount(
		&mut self,
	) -> Vec<CustomerRemorseReturnShippingFeesAmountProperty> {
		std::mem::take(&mut self.r#customer_remorse_return_shipping_fees_amount)
	}
	fn get_in_store_returns_offered(&self) -> &[InStoreReturnsOfferedProperty] {
		self.r#in_store_returns_offered.as_slice()
	}
	fn take_in_store_returns_offered(&mut self) -> Vec<InStoreReturnsOfferedProperty> {
		std::mem::take(&mut self.r#in_store_returns_offered)
	}
	fn get_item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty> {
		std::mem::take(&mut self.r#item_condition)
	}
	fn get_item_defect_return_fees(&self) -> &[ItemDefectReturnFeesProperty] {
		self.r#item_defect_return_fees.as_slice()
	}
	fn take_item_defect_return_fees(&mut self) -> Vec<ItemDefectReturnFeesProperty> {
		std::mem::take(&mut self.r#item_defect_return_fees)
	}
	fn get_item_defect_return_label_source(&self) -> &[ItemDefectReturnLabelSourceProperty] {
		self.r#item_defect_return_label_source.as_slice()
	}
	fn take_item_defect_return_label_source(&mut self) -> Vec<ItemDefectReturnLabelSourceProperty> {
		std::mem::take(&mut self.r#item_defect_return_label_source)
	}
	fn get_item_defect_return_shipping_fees_amount(
		&self,
	) -> &[ItemDefectReturnShippingFeesAmountProperty] {
		self.r#item_defect_return_shipping_fees_amount.as_slice()
	}
	fn take_item_defect_return_shipping_fees_amount(
		&mut self,
	) -> Vec<ItemDefectReturnShippingFeesAmountProperty> {
		std::mem::take(&mut self.r#item_defect_return_shipping_fees_amount)
	}
	fn get_merchant_return_days(&self) -> &[MerchantReturnDaysProperty] {
		self.r#merchant_return_days.as_slice()
	}
	fn take_merchant_return_days(&mut self) -> Vec<MerchantReturnDaysProperty> {
		std::mem::take(&mut self.r#merchant_return_days)
	}
	fn get_merchant_return_link(&self) -> &[MerchantReturnLinkProperty] {
		self.r#merchant_return_link.as_slice()
	}
	fn take_merchant_return_link(&mut self) -> Vec<MerchantReturnLinkProperty> {
		std::mem::take(&mut self.r#merchant_return_link)
	}
	fn get_refund_type(&self) -> &[RefundTypeProperty] {
		self.r#refund_type.as_slice()
	}
	fn take_refund_type(&mut self) -> Vec<RefundTypeProperty> {
		std::mem::take(&mut self.r#refund_type)
	}
	fn get_restocking_fee(&self) -> &[RestockingFeeProperty] {
		self.r#restocking_fee.as_slice()
	}
	fn take_restocking_fee(&mut self) -> Vec<RestockingFeeProperty> {
		std::mem::take(&mut self.r#restocking_fee)
	}
	fn get_return_fees(&self) -> &[ReturnFeesProperty] {
		self.r#return_fees.as_slice()
	}
	fn take_return_fees(&mut self) -> Vec<ReturnFeesProperty> {
		std::mem::take(&mut self.r#return_fees)
	}
	fn get_return_label_source(&self) -> &[ReturnLabelSourceProperty] {
		self.r#return_label_source.as_slice()
	}
	fn take_return_label_source(&mut self) -> Vec<ReturnLabelSourceProperty> {
		std::mem::take(&mut self.r#return_label_source)
	}
	fn get_return_method(&self) -> &[ReturnMethodProperty] {
		self.r#return_method.as_slice()
	}
	fn take_return_method(&mut self) -> Vec<ReturnMethodProperty> {
		std::mem::take(&mut self.r#return_method)
	}
	fn get_return_policy_category(&self) -> &[ReturnPolicyCategoryProperty] {
		self.r#return_policy_category.as_slice()
	}
	fn take_return_policy_category(&mut self) -> Vec<ReturnPolicyCategoryProperty> {
		std::mem::take(&mut self.r#return_policy_category)
	}
	fn get_return_policy_country(&self) -> &[ReturnPolicyCountryProperty] {
		self.r#return_policy_country.as_slice()
	}
	fn take_return_policy_country(&mut self) -> Vec<ReturnPolicyCountryProperty> {
		std::mem::take(&mut self.r#return_policy_country)
	}
	fn get_return_policy_seasonal_override(&self) -> &[ReturnPolicySeasonalOverrideProperty] {
		self.r#return_policy_seasonal_override.as_slice()
	}
	fn take_return_policy_seasonal_override(
		&mut self,
	) -> Vec<ReturnPolicySeasonalOverrideProperty> {
		std::mem::take(&mut self.r#return_policy_seasonal_override)
	}
	fn get_return_shipping_fees_amount(&self) -> &[ReturnShippingFeesAmountProperty] {
		self.r#return_shipping_fees_amount.as_slice()
	}
	fn take_return_shipping_fees_amount(&mut self) -> Vec<ReturnShippingFeesAmountProperty> {
		std::mem::take(&mut self.r#return_shipping_fees_amount)
	}
	fn get_valid_for_member_tier(&self) -> &[ValidForMemberTierProperty] {
		self.r#valid_for_member_tier.as_slice()
	}
	fn take_valid_for_member_tier(&mut self) -> Vec<ValidForMemberTierProperty> {
		std::mem::take(&mut self.r#valid_for_member_tier)
	}
}
impl ThingTrait for MerchantReturnPolicy {
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
