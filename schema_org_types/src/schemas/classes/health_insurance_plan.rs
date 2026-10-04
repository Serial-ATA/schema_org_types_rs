use super::*;
/// <https://schema.org/HealthInsurancePlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct HealthInsurancePlan {
	/// <https://schema.org/benefitsSummaryUrl>
	#[cfg_attr(feature = "serde", serde(rename = "benefitsSummaryUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#benefits_summary_url: Vec<BenefitsSummaryUrlProperty>,
	/// <https://schema.org/contactPoint>
	#[cfg_attr(feature = "serde", serde(rename = "contactPoint"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contact_point: Vec<ContactPointProperty>,
	/// <https://schema.org/healthPlanDrugOption>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanDrugOption"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_drug_option: Vec<HealthPlanDrugOptionProperty>,
	/// <https://schema.org/healthPlanDrugTier>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanDrugTier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_drug_tier: Vec<HealthPlanDrugTierProperty>,
	/// <https://schema.org/healthPlanId>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanId"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_id: Vec<HealthPlanIdProperty>,
	/// <https://schema.org/healthPlanMarketingUrl>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanMarketingUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_marketing_url: Vec<HealthPlanMarketingUrlProperty>,
	/// <https://schema.org/includesHealthPlanFormulary>
	#[cfg_attr(feature = "serde", serde(rename = "includesHealthPlanFormulary"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#includes_health_plan_formulary: Vec<IncludesHealthPlanFormularyProperty>,
	/// <https://schema.org/includesHealthPlanNetwork>
	#[cfg_attr(feature = "serde", serde(rename = "includesHealthPlanNetwork"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#includes_health_plan_network: Vec<IncludesHealthPlanNetworkProperty>,
	/// <https://schema.org/usesHealthPlanIdStandard>
	#[cfg_attr(feature = "serde", serde(rename = "usesHealthPlanIdStandard"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#uses_health_plan_id_standard: Vec<UsesHealthPlanIdStandardProperty>,
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
/// This trait is for properties from <https://schema.org/HealthInsurancePlan>.
pub trait HealthInsurancePlanTrait {
	/// Get <https://schema.org/benefitsSummaryUrl> from [`Self`] as borrowed slice.
	fn get_benefits_summary_url(&self) -> &[BenefitsSummaryUrlProperty];
	/// Take <https://schema.org/benefitsSummaryUrl> from [`Self`] as owned vector.
	fn take_benefits_summary_url(&mut self) -> Vec<BenefitsSummaryUrlProperty>;
	/// Get <https://schema.org/contactPoint> from [`Self`] as borrowed slice.
	fn get_contact_point(&self) -> &[ContactPointProperty];
	/// Take <https://schema.org/contactPoint> from [`Self`] as owned vector.
	fn take_contact_point(&mut self) -> Vec<ContactPointProperty>;
	/// Get <https://schema.org/healthPlanDrugOption> from [`Self`] as borrowed slice.
	fn get_health_plan_drug_option(&self) -> &[HealthPlanDrugOptionProperty];
	/// Take <https://schema.org/healthPlanDrugOption> from [`Self`] as owned vector.
	fn take_health_plan_drug_option(&mut self) -> Vec<HealthPlanDrugOptionProperty>;
	/// Get <https://schema.org/healthPlanDrugTier> from [`Self`] as borrowed slice.
	fn get_health_plan_drug_tier(&self) -> &[HealthPlanDrugTierProperty];
	/// Take <https://schema.org/healthPlanDrugTier> from [`Self`] as owned vector.
	fn take_health_plan_drug_tier(&mut self) -> Vec<HealthPlanDrugTierProperty>;
	/// Get <https://schema.org/healthPlanId> from [`Self`] as borrowed slice.
	fn get_health_plan_id(&self) -> &[HealthPlanIdProperty];
	/// Take <https://schema.org/healthPlanId> from [`Self`] as owned vector.
	fn take_health_plan_id(&mut self) -> Vec<HealthPlanIdProperty>;
	/// Get <https://schema.org/healthPlanMarketingUrl> from [`Self`] as borrowed slice.
	fn get_health_plan_marketing_url(&self) -> &[HealthPlanMarketingUrlProperty];
	/// Take <https://schema.org/healthPlanMarketingUrl> from [`Self`] as owned vector.
	fn take_health_plan_marketing_url(&mut self) -> Vec<HealthPlanMarketingUrlProperty>;
	/// Get <https://schema.org/includesHealthPlanFormulary> from [`Self`] as borrowed slice.
	fn get_includes_health_plan_formulary(&self) -> &[IncludesHealthPlanFormularyProperty];
	/// Take <https://schema.org/includesHealthPlanFormulary> from [`Self`] as owned vector.
	fn take_includes_health_plan_formulary(&mut self) -> Vec<IncludesHealthPlanFormularyProperty>;
	/// Get <https://schema.org/includesHealthPlanNetwork> from [`Self`] as borrowed slice.
	fn get_includes_health_plan_network(&self) -> &[IncludesHealthPlanNetworkProperty];
	/// Take <https://schema.org/includesHealthPlanNetwork> from [`Self`] as owned vector.
	fn take_includes_health_plan_network(&mut self) -> Vec<IncludesHealthPlanNetworkProperty>;
	/// Get <https://schema.org/usesHealthPlanIdStandard> from [`Self`] as borrowed slice.
	fn get_uses_health_plan_id_standard(&self) -> &[UsesHealthPlanIdStandardProperty];
	/// Take <https://schema.org/usesHealthPlanIdStandard> from [`Self`] as owned vector.
	fn take_uses_health_plan_id_standard(&mut self) -> Vec<UsesHealthPlanIdStandardProperty>;
}
impl HealthInsurancePlanTrait for HealthInsurancePlan {
	fn get_benefits_summary_url(&self) -> &[BenefitsSummaryUrlProperty] {
		self.r#benefits_summary_url.as_slice()
	}
	fn take_benefits_summary_url(&mut self) -> Vec<BenefitsSummaryUrlProperty> {
		std::mem::take(&mut self.r#benefits_summary_url)
	}
	fn get_contact_point(&self) -> &[ContactPointProperty] {
		self.r#contact_point.as_slice()
	}
	fn take_contact_point(&mut self) -> Vec<ContactPointProperty> {
		std::mem::take(&mut self.r#contact_point)
	}
	fn get_health_plan_drug_option(&self) -> &[HealthPlanDrugOptionProperty] {
		self.r#health_plan_drug_option.as_slice()
	}
	fn take_health_plan_drug_option(&mut self) -> Vec<HealthPlanDrugOptionProperty> {
		std::mem::take(&mut self.r#health_plan_drug_option)
	}
	fn get_health_plan_drug_tier(&self) -> &[HealthPlanDrugTierProperty] {
		self.r#health_plan_drug_tier.as_slice()
	}
	fn take_health_plan_drug_tier(&mut self) -> Vec<HealthPlanDrugTierProperty> {
		std::mem::take(&mut self.r#health_plan_drug_tier)
	}
	fn get_health_plan_id(&self) -> &[HealthPlanIdProperty] {
		self.r#health_plan_id.as_slice()
	}
	fn take_health_plan_id(&mut self) -> Vec<HealthPlanIdProperty> {
		std::mem::take(&mut self.r#health_plan_id)
	}
	fn get_health_plan_marketing_url(&self) -> &[HealthPlanMarketingUrlProperty] {
		self.r#health_plan_marketing_url.as_slice()
	}
	fn take_health_plan_marketing_url(&mut self) -> Vec<HealthPlanMarketingUrlProperty> {
		std::mem::take(&mut self.r#health_plan_marketing_url)
	}
	fn get_includes_health_plan_formulary(&self) -> &[IncludesHealthPlanFormularyProperty] {
		self.r#includes_health_plan_formulary.as_slice()
	}
	fn take_includes_health_plan_formulary(&mut self) -> Vec<IncludesHealthPlanFormularyProperty> {
		std::mem::take(&mut self.r#includes_health_plan_formulary)
	}
	fn get_includes_health_plan_network(&self) -> &[IncludesHealthPlanNetworkProperty] {
		self.r#includes_health_plan_network.as_slice()
	}
	fn take_includes_health_plan_network(&mut self) -> Vec<IncludesHealthPlanNetworkProperty> {
		std::mem::take(&mut self.r#includes_health_plan_network)
	}
	fn get_uses_health_plan_id_standard(&self) -> &[UsesHealthPlanIdStandardProperty] {
		self.r#uses_health_plan_id_standard.as_slice()
	}
	fn take_uses_health_plan_id_standard(&mut self) -> Vec<UsesHealthPlanIdStandardProperty> {
		std::mem::take(&mut self.r#uses_health_plan_id_standard)
	}
}
impl ThingTrait for HealthInsurancePlan {
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
