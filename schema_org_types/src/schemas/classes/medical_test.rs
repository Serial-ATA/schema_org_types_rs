use super::*;
/// <https://schema.org/MedicalTest>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MedicalTest {
	/// <https://schema.org/affectedBy>
	#[cfg_attr(feature = "serde", serde(rename = "affectedBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#affected_by: Vec<AffectedByProperty>,
	/// <https://schema.org/normalRange>
	#[cfg_attr(feature = "serde", serde(rename = "normalRange"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#normal_range: Vec<NormalRangeProperty>,
	/// <https://schema.org/signDetected>
	#[cfg_attr(feature = "serde", serde(rename = "signDetected"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sign_detected: Vec<SignDetectedProperty>,
	/// <https://schema.org/usedToDiagnose>
	#[cfg_attr(feature = "serde", serde(rename = "usedToDiagnose"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#used_to_diagnose: Vec<UsedToDiagnoseProperty>,
	/// <https://schema.org/usesDevice>
	#[cfg_attr(feature = "serde", serde(rename = "usesDevice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#uses_device: Vec<UsesDeviceProperty>,
	/// <https://schema.org/code>
	#[cfg_attr(feature = "serde", serde(rename = "code"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#code: Vec<CodeProperty>,
	/// <https://schema.org/funding>
	#[cfg_attr(feature = "serde", serde(rename = "funding"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#funding: Vec<FundingProperty>,
	/// <https://schema.org/guideline>
	#[cfg_attr(feature = "serde", serde(rename = "guideline"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#guideline: Vec<GuidelineProperty>,
	/// <https://schema.org/legalStatus>
	#[cfg_attr(feature = "serde", serde(rename = "legalStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#legal_status: Vec<LegalStatusProperty>,
	/// <https://schema.org/medicineSystem>
	#[cfg_attr(feature = "serde", serde(rename = "medicineSystem"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#medicine_system: Vec<MedicineSystemProperty>,
	/// <https://schema.org/recognizingAuthority>
	#[cfg_attr(feature = "serde", serde(rename = "recognizingAuthority"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
	/// <https://schema.org/relevantSpecialty>
	#[cfg_attr(feature = "serde", serde(rename = "relevantSpecialty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
	/// <https://schema.org/study>
	#[cfg_attr(feature = "serde", serde(rename = "study"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#study: Vec<StudyProperty>,
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
/// This trait is for properties from <https://schema.org/MedicalTest>.
pub trait MedicalTestTrait {
	/// Get <https://schema.org/affectedBy> from [`Self`] as borrowed slice.
	fn get_affected_by(&self) -> &[AffectedByProperty];
	/// Take <https://schema.org/affectedBy> from [`Self`] as owned vector.
	fn take_affected_by(&mut self) -> Vec<AffectedByProperty>;
	/// Get <https://schema.org/normalRange> from [`Self`] as borrowed slice.
	fn get_normal_range(&self) -> &[NormalRangeProperty];
	/// Take <https://schema.org/normalRange> from [`Self`] as owned vector.
	fn take_normal_range(&mut self) -> Vec<NormalRangeProperty>;
	/// Get <https://schema.org/signDetected> from [`Self`] as borrowed slice.
	fn get_sign_detected(&self) -> &[SignDetectedProperty];
	/// Take <https://schema.org/signDetected> from [`Self`] as owned vector.
	fn take_sign_detected(&mut self) -> Vec<SignDetectedProperty>;
	/// Get <https://schema.org/usedToDiagnose> from [`Self`] as borrowed slice.
	fn get_used_to_diagnose(&self) -> &[UsedToDiagnoseProperty];
	/// Take <https://schema.org/usedToDiagnose> from [`Self`] as owned vector.
	fn take_used_to_diagnose(&mut self) -> Vec<UsedToDiagnoseProperty>;
	/// Get <https://schema.org/usesDevice> from [`Self`] as borrowed slice.
	fn get_uses_device(&self) -> &[UsesDeviceProperty];
	/// Take <https://schema.org/usesDevice> from [`Self`] as owned vector.
	fn take_uses_device(&mut self) -> Vec<UsesDeviceProperty>;
}
impl MedicalTestTrait for MedicalTest {
	fn get_affected_by(&self) -> &[AffectedByProperty] {
		self.r#affected_by.as_slice()
	}
	fn take_affected_by(&mut self) -> Vec<AffectedByProperty> {
		std::mem::take(&mut self.r#affected_by)
	}
	fn get_normal_range(&self) -> &[NormalRangeProperty] {
		self.r#normal_range.as_slice()
	}
	fn take_normal_range(&mut self) -> Vec<NormalRangeProperty> {
		std::mem::take(&mut self.r#normal_range)
	}
	fn get_sign_detected(&self) -> &[SignDetectedProperty] {
		self.r#sign_detected.as_slice()
	}
	fn take_sign_detected(&mut self) -> Vec<SignDetectedProperty> {
		std::mem::take(&mut self.r#sign_detected)
	}
	fn get_used_to_diagnose(&self) -> &[UsedToDiagnoseProperty] {
		self.r#used_to_diagnose.as_slice()
	}
	fn take_used_to_diagnose(&mut self) -> Vec<UsedToDiagnoseProperty> {
		std::mem::take(&mut self.r#used_to_diagnose)
	}
	fn get_uses_device(&self) -> &[UsesDeviceProperty] {
		self.r#uses_device.as_slice()
	}
	fn take_uses_device(&mut self) -> Vec<UsesDeviceProperty> {
		std::mem::take(&mut self.r#uses_device)
	}
}
impl MedicalEntityTrait for MedicalTest {
	fn get_code(&self) -> &[CodeProperty] {
		self.r#code.as_slice()
	}
	fn take_code(&mut self) -> Vec<CodeProperty> {
		std::mem::take(&mut self.r#code)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_guideline(&self) -> &[GuidelineProperty] {
		self.r#guideline.as_slice()
	}
	fn take_guideline(&mut self) -> Vec<GuidelineProperty> {
		std::mem::take(&mut self.r#guideline)
	}
	fn get_legal_status(&self) -> &[LegalStatusProperty] {
		self.r#legal_status.as_slice()
	}
	fn take_legal_status(&mut self) -> Vec<LegalStatusProperty> {
		std::mem::take(&mut self.r#legal_status)
	}
	fn get_medicine_system(&self) -> &[MedicineSystemProperty] {
		self.r#medicine_system.as_slice()
	}
	fn take_medicine_system(&mut self) -> Vec<MedicineSystemProperty> {
		std::mem::take(&mut self.r#medicine_system)
	}
	fn get_recognizing_authority(&self) -> &[RecognizingAuthorityProperty] {
		self.r#recognizing_authority.as_slice()
	}
	fn take_recognizing_authority(&mut self) -> Vec<RecognizingAuthorityProperty> {
		std::mem::take(&mut self.r#recognizing_authority)
	}
	fn get_relevant_specialty(&self) -> &[RelevantSpecialtyProperty] {
		self.r#relevant_specialty.as_slice()
	}
	fn take_relevant_specialty(&mut self) -> Vec<RelevantSpecialtyProperty> {
		std::mem::take(&mut self.r#relevant_specialty)
	}
	fn get_study(&self) -> &[StudyProperty] {
		self.r#study.as_slice()
	}
	fn take_study(&mut self) -> Vec<StudyProperty> {
		std::mem::take(&mut self.r#study)
	}
}
impl ThingTrait for MedicalTest {
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
