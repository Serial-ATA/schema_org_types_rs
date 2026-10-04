use super::*;
/// <https://schema.org/FinancialIncentive>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct FinancialIncentive {
	/// <https://schema.org/areaServed>
	pub r#area_served: Vec<AreaServedProperty>,
	/// <https://schema.org/eligibleWithSupplier>
	pub r#eligible_with_supplier: Vec<EligibleWithSupplierProperty>,
	/// <https://schema.org/incentiveAmount>
	pub r#incentive_amount: Vec<IncentiveAmountProperty>,
	/// <https://schema.org/incentiveStatus>
	pub r#incentive_status: Vec<IncentiveStatusProperty>,
	/// <https://schema.org/incentiveType>
	pub r#incentive_type: Vec<IncentiveTypeProperty>,
	/// <https://schema.org/incentivizedItem>
	pub r#incentivized_item: Vec<IncentivizedItemProperty>,
	/// <https://schema.org/incomeLimit>
	pub r#income_limit: Vec<IncomeLimitProperty>,
	/// <https://schema.org/provider>
	pub r#provider: Vec<ProviderProperty>,
	/// <https://schema.org/publisher>
	pub r#publisher: Vec<PublisherProperty>,
	/// <https://schema.org/purchasePriceLimit>
	pub r#purchase_price_limit: Vec<PurchasePriceLimitProperty>,
	/// <https://schema.org/purchaseType>
	pub r#purchase_type: Vec<PurchaseTypeProperty>,
	/// <https://schema.org/qualifiedExpense>
	pub r#qualified_expense: Vec<QualifiedExpenseProperty>,
	/// <https://schema.org/validFrom>
	pub r#valid_from: Vec<ValidFromProperty>,
	/// <https://schema.org/validThrough>
	pub r#valid_through: Vec<ValidThroughProperty>,
	/// <https://schema.org/additionalType>
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	/// <https://schema.org/alternateName>
	pub r#alternate_name: Vec<AlternateNameProperty>,
	/// <https://schema.org/description>
	pub r#description: Vec<DescriptionProperty>,
	/// <https://schema.org/disambiguatingDescription>
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	/// <https://schema.org/identifier>
	pub r#identifier: Vec<IdentifierProperty>,
	/// <https://schema.org/image>
	pub r#image: Vec<ImageProperty>,
	/// <https://schema.org/mainEntityOfPage>
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	/// <https://schema.org/name>
	pub r#name: Vec<NameProperty>,
	/// <https://schema.org/owner>
	pub r#owner: Vec<OwnerProperty>,
	/// <https://schema.org/potentialAction>
	pub r#potential_action: Vec<PotentialActionProperty>,
	/// <https://schema.org/sameAs>
	pub r#same_as: Vec<SameAsProperty>,
	/// <https://schema.org/subjectOf>
	pub r#subject_of: Vec<SubjectOfProperty>,
	/// <https://schema.org/url>
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
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor, ser::SerializeStruct,
	};

	use super::*;
	impl Serialize for FinancialIncentive {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#area_served) as usize,
				!Vec::is_empty(&self.r#eligible_with_supplier) as usize,
				!Vec::is_empty(&self.r#incentive_amount) as usize,
				!Vec::is_empty(&self.r#incentive_status) as usize,
				!Vec::is_empty(&self.r#incentive_type) as usize,
				!Vec::is_empty(&self.r#incentivized_item) as usize,
				!Vec::is_empty(&self.r#income_limit) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#publisher) as usize,
				!Vec::is_empty(&self.r#purchase_price_limit) as usize,
				!Vec::is_empty(&self.r#purchase_type) as usize,
				!Vec::is_empty(&self.r#qualified_expense) as usize,
				!Vec::is_empty(&self.r#valid_from) as usize,
				!Vec::is_empty(&self.r#valid_through) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#owner) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#url) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "FinancialIncentive", len)?;
			if !Vec::is_empty(&self.r#area_served) {
				serialize_struct.serialize_field("areaServed", {
					struct SerializeWith<'a>(&'a Vec<AreaServedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#area_served)
				})?;
			} else {
				serialize_struct.skip_field("areaServed")?;
			}
			if !Vec::is_empty(&self.r#eligible_with_supplier) {
				serialize_struct.serialize_field("eligibleWithSupplier", {
					struct SerializeWith<'a>(&'a Vec<EligibleWithSupplierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#eligible_with_supplier)
				})?;
			} else {
				serialize_struct.skip_field("eligibleWithSupplier")?;
			}
			if !Vec::is_empty(&self.r#incentive_amount) {
				serialize_struct.serialize_field("incentiveAmount", {
					struct SerializeWith<'a>(&'a Vec<IncentiveAmountProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#incentive_amount)
				})?;
			} else {
				serialize_struct.skip_field("incentiveAmount")?;
			}
			if !Vec::is_empty(&self.r#incentive_status) {
				serialize_struct.serialize_field("incentiveStatus", {
					struct SerializeWith<'a>(&'a Vec<IncentiveStatusProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#incentive_status)
				})?;
			} else {
				serialize_struct.skip_field("incentiveStatus")?;
			}
			if !Vec::is_empty(&self.r#incentive_type) {
				serialize_struct.serialize_field("incentiveType", {
					struct SerializeWith<'a>(&'a Vec<IncentiveTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#incentive_type)
				})?;
			} else {
				serialize_struct.skip_field("incentiveType")?;
			}
			if !Vec::is_empty(&self.r#incentivized_item) {
				serialize_struct.serialize_field("incentivizedItem", {
					struct SerializeWith<'a>(&'a Vec<IncentivizedItemProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#incentivized_item)
				})?;
			} else {
				serialize_struct.skip_field("incentivizedItem")?;
			}
			if !Vec::is_empty(&self.r#income_limit) {
				serialize_struct.serialize_field("incomeLimit", {
					struct SerializeWith<'a>(&'a Vec<IncomeLimitProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#income_limit)
				})?;
			} else {
				serialize_struct.skip_field("incomeLimit")?;
			}
			if !Vec::is_empty(&self.r#provider) {
				serialize_struct.serialize_field("provider", {
					struct SerializeWith<'a>(&'a Vec<ProviderProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#provider)
				})?;
			} else {
				serialize_struct.skip_field("provider")?;
			}
			if !Vec::is_empty(&self.r#publisher) {
				serialize_struct.serialize_field("publisher", {
					struct SerializeWith<'a>(&'a Vec<PublisherProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#publisher)
				})?;
			} else {
				serialize_struct.skip_field("publisher")?;
			}
			if !Vec::is_empty(&self.r#purchase_price_limit) {
				serialize_struct.serialize_field("purchasePriceLimit", {
					struct SerializeWith<'a>(&'a Vec<PurchasePriceLimitProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#purchase_price_limit)
				})?;
			} else {
				serialize_struct.skip_field("purchasePriceLimit")?;
			}
			if !Vec::is_empty(&self.r#purchase_type) {
				serialize_struct.serialize_field("purchaseType", {
					struct SerializeWith<'a>(&'a Vec<PurchaseTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#purchase_type)
				})?;
			} else {
				serialize_struct.skip_field("purchaseType")?;
			}
			if !Vec::is_empty(&self.r#qualified_expense) {
				serialize_struct.serialize_field("qualifiedExpense", {
					struct SerializeWith<'a>(&'a Vec<QualifiedExpenseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#qualified_expense)
				})?;
			} else {
				serialize_struct.skip_field("qualifiedExpense")?;
			}
			if !Vec::is_empty(&self.r#valid_from) {
				serialize_struct.serialize_field("validFrom", {
					struct SerializeWith<'a>(&'a Vec<ValidFromProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#valid_from)
				})?;
			} else {
				serialize_struct.skip_field("validFrom")?;
			}
			if !Vec::is_empty(&self.r#valid_through) {
				serialize_struct.serialize_field("validThrough", {
					struct SerializeWith<'a>(&'a Vec<ValidThroughProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#valid_through)
				})?;
			} else {
				serialize_struct.skip_field("validThrough")?;
			}
			if !Vec::is_empty(&self.r#additional_type) {
				serialize_struct.serialize_field("additionalType", {
					struct SerializeWith<'a>(&'a Vec<AdditionalTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#additional_type)
				})?;
			} else {
				serialize_struct.skip_field("additionalType")?;
			}
			if !Vec::is_empty(&self.r#alternate_name) {
				serialize_struct.serialize_field("alternateName", {
					struct SerializeWith<'a>(&'a Vec<AlternateNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alternate_name)
				})?;
			} else {
				serialize_struct.skip_field("alternateName")?;
			}
			if !Vec::is_empty(&self.r#description) {
				serialize_struct.serialize_field("description", {
					struct SerializeWith<'a>(&'a Vec<DescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#description)
				})?;
			} else {
				serialize_struct.skip_field("description")?;
			}
			if !Vec::is_empty(&self.r#disambiguating_description) {
				serialize_struct.serialize_field("disambiguatingDescription", {
					struct SerializeWith<'a>(&'a Vec<DisambiguatingDescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#disambiguating_description)
				})?;
			} else {
				serialize_struct.skip_field("disambiguatingDescription")?;
			}
			if !Vec::is_empty(&self.r#identifier) {
				serialize_struct.serialize_field("identifier", {
					struct SerializeWith<'a>(&'a Vec<IdentifierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#identifier)
				})?;
			} else {
				serialize_struct.skip_field("identifier")?;
			}
			if !Vec::is_empty(&self.r#image) {
				serialize_struct.serialize_field("image", {
					struct SerializeWith<'a>(&'a Vec<ImageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#image)
				})?;
			} else {
				serialize_struct.skip_field("image")?;
			}
			if !Vec::is_empty(&self.r#main_entity_of_page) {
				serialize_struct.serialize_field("mainEntityOfPage", {
					struct SerializeWith<'a>(&'a Vec<MainEntityOfPageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#main_entity_of_page)
				})?;
			} else {
				serialize_struct.skip_field("mainEntityOfPage")?;
			}
			if !Vec::is_empty(&self.r#name) {
				serialize_struct.serialize_field("name", {
					struct SerializeWith<'a>(&'a Vec<NameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#name)
				})?;
			} else {
				serialize_struct.skip_field("name")?;
			}
			if !Vec::is_empty(&self.r#owner) {
				serialize_struct.serialize_field("owner", {
					struct SerializeWith<'a>(&'a Vec<OwnerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#owner)
				})?;
			} else {
				serialize_struct.skip_field("owner")?;
			}
			if !Vec::is_empty(&self.r#potential_action) {
				serialize_struct.serialize_field("potentialAction", {
					struct SerializeWith<'a>(&'a Vec<PotentialActionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#potential_action)
				})?;
			} else {
				serialize_struct.skip_field("potentialAction")?;
			}
			if !Vec::is_empty(&self.r#same_as) {
				serialize_struct.serialize_field("sameAs", {
					struct SerializeWith<'a>(&'a Vec<SameAsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#same_as)
				})?;
			} else {
				serialize_struct.skip_field("sameAs")?;
			}
			if !Vec::is_empty(&self.r#subject_of) {
				serialize_struct.serialize_field("subjectOf", {
					struct SerializeWith<'a>(&'a Vec<SubjectOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#subject_of)
				})?;
			} else {
				serialize_struct.skip_field("subjectOf")?;
			}
			if !Vec::is_empty(&self.r#url) {
				serialize_struct.serialize_field("url", {
					struct SerializeWith<'a>(&'a Vec<UrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#url)
				})?;
			} else {
				serialize_struct.skip_field("url")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for FinancialIncentive {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AreaServed,
				EligibleWithSupplier,
				IncentiveAmount,
				IncentiveStatus,
				IncentiveType,
				IncentivizedItem,
				IncomeLimit,
				Provider,
				Publisher,
				PurchasePriceLimit,
				PurchaseType,
				QualifiedExpense,
				ValidFrom,
				ValidThrough,
				AdditionalType,
				AlternateName,
				Description,
				DisambiguatingDescription,
				Identifier,
				Image,
				MainEntityOfPage,
				Name,
				Owner,
				PotentialAction,
				SameAs,
				SubjectOf,
				Url,
				Ignore,
			}
			struct FieldVisitor;
			impl<'de> Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("field identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						"areaServed" => Ok(Field::AreaServed),
						"eligibleWithSupplier" => Ok(Field::EligibleWithSupplier),
						"incentiveAmount" => Ok(Field::IncentiveAmount),
						"incentiveStatus" => Ok(Field::IncentiveStatus),
						"incentiveType" => Ok(Field::IncentiveType),
						"incentivizedItem" => Ok(Field::IncentivizedItem),
						"incomeLimit" => Ok(Field::IncomeLimit),
						"provider" => Ok(Field::Provider),
						"publisher" => Ok(Field::Publisher),
						"purchasePriceLimit" => Ok(Field::PurchasePriceLimit),
						"purchaseType" => Ok(Field::PurchaseType),
						"qualifiedExpense" => Ok(Field::QualifiedExpense),
						"validFrom" => Ok(Field::ValidFrom),
						"validThrough" => Ok(Field::ValidThrough),
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"name" => Ok(Field::Name),
						"owner" => Ok(Field::Owner),
						"potentialAction" => Ok(Field::PotentialAction),
						"sameAs" => Ok(Field::SameAs),
						"subjectOf" => Ok(Field::SubjectOf),
						"url" => Ok(Field::Url),
						"id" | "type" => Ok(Field::Ignore),
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"areaServed" => Ok(Field::AreaServed),
						b"eligibleWithSupplier" => Ok(Field::EligibleWithSupplier),
						b"incentiveAmount" => Ok(Field::IncentiveAmount),
						b"incentiveStatus" => Ok(Field::IncentiveStatus),
						b"incentiveType" => Ok(Field::IncentiveType),
						b"incentivizedItem" => Ok(Field::IncentivizedItem),
						b"incomeLimit" => Ok(Field::IncomeLimit),
						b"provider" => Ok(Field::Provider),
						b"publisher" => Ok(Field::Publisher),
						b"purchasePriceLimit" => Ok(Field::PurchasePriceLimit),
						b"purchaseType" => Ok(Field::PurchaseType),
						b"qualifiedExpense" => Ok(Field::QualifiedExpense),
						b"validFrom" => Ok(Field::ValidFrom),
						b"validThrough" => Ok(Field::ValidThrough),
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"name" => Ok(Field::Name),
						b"owner" => Ok(Field::Owner),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"sameAs" => Ok(Field::SameAs),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"url" => Ok(Field::Url),
						b"id" | b"type" => Ok(Field::Ignore),
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_field(value, FIELDS))
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
			struct ClassVisitor;
			impl<'de> Visitor<'de> for ClassVisitor {
				type Value = FinancialIncentive;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema FinancialIncentive")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#area_served_property = None;
					let mut r#eligible_with_supplier_property = None;
					let mut r#incentive_amount_property = None;
					let mut r#incentive_status_property = None;
					let mut r#incentive_type_property = None;
					let mut r#incentivized_item_property = None;
					let mut r#income_limit_property = None;
					let mut r#provider_property = None;
					let mut r#publisher_property = None;
					let mut r#purchase_price_limit_property = None;
					let mut r#purchase_type_property = None;
					let mut r#qualified_expense_property = None;
					let mut r#valid_from_property = None;
					let mut r#valid_through_property = None;
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#name_property = None;
					let mut r#owner_property = None;
					let mut r#potential_action_property = None;
					let mut r#same_as_property = None;
					let mut r#subject_of_property = None;
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							Field::AreaServed => {
								if r#area_served_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"areaServed",
									));
								}
								r#area_served_property = Some({
									struct DeserializeWith(Vec<AreaServedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EligibleWithSupplier => {
								if r#eligible_with_supplier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibleWithSupplier",
									));
								}
								r#eligible_with_supplier_property = Some({
									struct DeserializeWith(Vec<EligibleWithSupplierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IncentiveAmount => {
								if r#incentive_amount_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incentiveAmount",
									));
								}
								r#incentive_amount_property = Some({
									struct DeserializeWith(Vec<IncentiveAmountProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IncentiveStatus => {
								if r#incentive_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incentiveStatus",
									));
								}
								r#incentive_status_property = Some({
									struct DeserializeWith(Vec<IncentiveStatusProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IncentiveType => {
								if r#incentive_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incentiveType",
									));
								}
								r#incentive_type_property = Some({
									struct DeserializeWith(Vec<IncentiveTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IncentivizedItem => {
								if r#incentivized_item_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incentivizedItem",
									));
								}
								r#incentivized_item_property = Some({
									struct DeserializeWith(Vec<IncentivizedItemProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IncomeLimit => {
								if r#income_limit_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incomeLimit",
									));
								}
								r#income_limit_property = Some({
									struct DeserializeWith(Vec<IncomeLimitProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Provider => {
								if r#provider_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"provider",
									));
								}
								r#provider_property = Some({
									struct DeserializeWith(Vec<ProviderProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Publisher => {
								if r#publisher_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publisher",
									));
								}
								r#publisher_property = Some({
									struct DeserializeWith(Vec<PublisherProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PurchasePriceLimit => {
								if r#purchase_price_limit_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"purchasePriceLimit",
									));
								}
								r#purchase_price_limit_property = Some({
									struct DeserializeWith(Vec<PurchasePriceLimitProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PurchaseType => {
								if r#purchase_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"purchaseType",
									));
								}
								r#purchase_type_property = Some({
									struct DeserializeWith(Vec<PurchaseTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::QualifiedExpense => {
								if r#qualified_expense_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"qualifiedExpense",
									));
								}
								r#qualified_expense_property = Some({
									struct DeserializeWith(Vec<QualifiedExpenseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ValidFrom => {
								if r#valid_from_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"validFrom",
									));
								}
								r#valid_from_property = Some({
									struct DeserializeWith(Vec<ValidFromProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ValidThrough => {
								if r#valid_through_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"validThrough",
									));
								}
								r#valid_through_property = Some({
									struct DeserializeWith(Vec<ValidThroughProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AdditionalType => {
								if r#additional_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalType",
									));
								}
								r#additional_type_property = Some({
									struct DeserializeWith(Vec<AdditionalTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AlternateName => {
								if r#alternate_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alternateName",
									));
								}
								r#alternate_name_property = Some({
									struct DeserializeWith(Vec<AlternateNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Description => {
								if r#description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"description",
									));
								}
								r#description_property = Some({
									struct DeserializeWith(Vec<DescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DisambiguatingDescription => {
								if r#disambiguating_description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"disambiguatingDescription",
									));
								}
								r#disambiguating_description_property = Some({
									struct DeserializeWith(Vec<DisambiguatingDescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Identifier => {
								if r#identifier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"identifier",
									));
								}
								r#identifier_property = Some({
									struct DeserializeWith(Vec<IdentifierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Image => {
								if r#image_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("image"));
								}
								r#image_property = Some({
									struct DeserializeWith(Vec<ImageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MainEntityOfPage => {
								if r#main_entity_of_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mainEntityOfPage",
									));
								}
								r#main_entity_of_page_property = Some({
									struct DeserializeWith(Vec<MainEntityOfPageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Name => {
								if r#name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("name"));
								}
								r#name_property = Some({
									struct DeserializeWith(Vec<NameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Owner => {
								if r#owner_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("owner"));
								}
								r#owner_property = Some({
									struct DeserializeWith(Vec<OwnerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PotentialAction => {
								if r#potential_action_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"potentialAction",
									));
								}
								r#potential_action_property = Some({
									struct DeserializeWith(Vec<PotentialActionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SameAs => {
								if r#same_as_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sameAs"));
								}
								r#same_as_property = Some({
									struct DeserializeWith(Vec<SameAsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SubjectOf => {
								if r#subject_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subjectOf",
									));
								}
								r#subject_of_property = Some({
									struct DeserializeWith(Vec<SubjectOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Url => {
								if r#url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("url"));
								}
								r#url_property = Some({
									struct DeserializeWith(Vec<UrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(
												deserializer
											)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Ignore => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(FinancialIncentive {
						r#area_served: r#area_served_property.unwrap_or_default(),
						r#eligible_with_supplier: r#eligible_with_supplier_property
							.unwrap_or_default(),
						r#incentive_amount: r#incentive_amount_property.unwrap_or_default(),
						r#incentive_status: r#incentive_status_property.unwrap_or_default(),
						r#incentive_type: r#incentive_type_property.unwrap_or_default(),
						r#incentivized_item: r#incentivized_item_property.unwrap_or_default(),
						r#income_limit: r#income_limit_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#publisher: r#publisher_property.unwrap_or_default(),
						r#purchase_price_limit: r#purchase_price_limit_property.unwrap_or_default(),
						r#purchase_type: r#purchase_type_property.unwrap_or_default(),
						r#qualified_expense: r#qualified_expense_property.unwrap_or_default(),
						r#valid_from: r#valid_from_property.unwrap_or_default(),
						r#valid_through: r#valid_through_property.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#owner: r#owner_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"areaServed",
				"eligibleWithSupplier",
				"incentiveAmount",
				"incentiveStatus",
				"incentiveType",
				"incentivizedItem",
				"incomeLimit",
				"provider",
				"publisher",
				"purchasePriceLimit",
				"purchaseType",
				"qualifiedExpense",
				"validFrom",
				"validThrough",
				"additionalType",
				"alternateName",
				"description",
				"disambiguatingDescription",
				"identifier",
				"image",
				"mainEntityOfPage",
				"name",
				"owner",
				"potentialAction",
				"sameAs",
				"subjectOf",
				"url",
			];
			deserializer.deserialize_struct("FinancialIncentive", FIELDS, ClassVisitor)
		}
	}
}
