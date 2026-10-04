use super::*;
/// <https://schema.org/HealthPlanCostSharingSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct HealthPlanCostSharingSpecification {
	/// <https://schema.org/healthPlanCoinsuranceOption>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanCoinsuranceOption"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_coinsurance_option: Vec<HealthPlanCoinsuranceOptionProperty>,
	/// <https://schema.org/healthPlanCoinsuranceRate>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanCoinsuranceRate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_coinsurance_rate: Vec<HealthPlanCoinsuranceRateProperty>,
	/// <https://schema.org/healthPlanCopay>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanCopay"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_copay: Vec<HealthPlanCopayProperty>,
	/// <https://schema.org/healthPlanCopayOption>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanCopayOption"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_copay_option: Vec<HealthPlanCopayOptionProperty>,
	/// <https://schema.org/healthPlanPharmacyCategory>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanPharmacyCategory"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_pharmacy_category: Vec<HealthPlanPharmacyCategoryProperty>,
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
/// This trait is for properties from <https://schema.org/HealthPlanCostSharingSpecification>.
pub trait HealthPlanCostSharingSpecificationTrait {
	/// Get <https://schema.org/healthPlanCoinsuranceOption> from [`Self`] as borrowed slice.
	fn get_health_plan_coinsurance_option(&self) -> &[HealthPlanCoinsuranceOptionProperty];
	/// Take <https://schema.org/healthPlanCoinsuranceOption> from [`Self`] as owned vector.
	fn take_health_plan_coinsurance_option(&mut self) -> Vec<HealthPlanCoinsuranceOptionProperty>;
	/// Get <https://schema.org/healthPlanCoinsuranceRate> from [`Self`] as borrowed slice.
	fn get_health_plan_coinsurance_rate(&self) -> &[HealthPlanCoinsuranceRateProperty];
	/// Take <https://schema.org/healthPlanCoinsuranceRate> from [`Self`] as owned vector.
	fn take_health_plan_coinsurance_rate(&mut self) -> Vec<HealthPlanCoinsuranceRateProperty>;
	/// Get <https://schema.org/healthPlanCopay> from [`Self`] as borrowed slice.
	fn get_health_plan_copay(&self) -> &[HealthPlanCopayProperty];
	/// Take <https://schema.org/healthPlanCopay> from [`Self`] as owned vector.
	fn take_health_plan_copay(&mut self) -> Vec<HealthPlanCopayProperty>;
	/// Get <https://schema.org/healthPlanCopayOption> from [`Self`] as borrowed slice.
	fn get_health_plan_copay_option(&self) -> &[HealthPlanCopayOptionProperty];
	/// Take <https://schema.org/healthPlanCopayOption> from [`Self`] as owned vector.
	fn take_health_plan_copay_option(&mut self) -> Vec<HealthPlanCopayOptionProperty>;
	/// Get <https://schema.org/healthPlanPharmacyCategory> from [`Self`] as borrowed slice.
	fn get_health_plan_pharmacy_category(&self) -> &[HealthPlanPharmacyCategoryProperty];
	/// Take <https://schema.org/healthPlanPharmacyCategory> from [`Self`] as owned vector.
	fn take_health_plan_pharmacy_category(&mut self) -> Vec<HealthPlanPharmacyCategoryProperty>;
}
impl HealthPlanCostSharingSpecificationTrait for HealthPlanCostSharingSpecification {
	fn get_health_plan_coinsurance_option(&self) -> &[HealthPlanCoinsuranceOptionProperty] {
		self.r#health_plan_coinsurance_option.as_slice()
	}
	fn take_health_plan_coinsurance_option(&mut self) -> Vec<HealthPlanCoinsuranceOptionProperty> {
		std::mem::take(&mut self.r#health_plan_coinsurance_option)
	}
	fn get_health_plan_coinsurance_rate(&self) -> &[HealthPlanCoinsuranceRateProperty] {
		self.r#health_plan_coinsurance_rate.as_slice()
	}
	fn take_health_plan_coinsurance_rate(&mut self) -> Vec<HealthPlanCoinsuranceRateProperty> {
		std::mem::take(&mut self.r#health_plan_coinsurance_rate)
	}
	fn get_health_plan_copay(&self) -> &[HealthPlanCopayProperty] {
		self.r#health_plan_copay.as_slice()
	}
	fn take_health_plan_copay(&mut self) -> Vec<HealthPlanCopayProperty> {
		std::mem::take(&mut self.r#health_plan_copay)
	}
	fn get_health_plan_copay_option(&self) -> &[HealthPlanCopayOptionProperty] {
		self.r#health_plan_copay_option.as_slice()
	}
	fn take_health_plan_copay_option(&mut self) -> Vec<HealthPlanCopayOptionProperty> {
		std::mem::take(&mut self.r#health_plan_copay_option)
	}
	fn get_health_plan_pharmacy_category(&self) -> &[HealthPlanPharmacyCategoryProperty] {
		self.r#health_plan_pharmacy_category.as_slice()
	}
	fn take_health_plan_pharmacy_category(&mut self) -> Vec<HealthPlanPharmacyCategoryProperty> {
		std::mem::take(&mut self.r#health_plan_pharmacy_category)
	}
}
impl ThingTrait for HealthPlanCostSharingSpecification {
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
