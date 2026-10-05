use super::*;
/// <https://schema.org/MedicalSign>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MedicalSign {
	/// <https://schema.org/identifyingExam>
	#[cfg_attr(feature = "serde", serde(rename = "identifyingExam"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#identifying_exam: Vec<IdentifyingExamProperty>,
	/// <https://schema.org/identifyingTest>
	#[cfg_attr(feature = "serde", serde(rename = "identifyingTest"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#identifying_test: Vec<IdentifyingTestProperty>,
	/// <https://schema.org/associatedAnatomy>
	#[cfg_attr(feature = "serde", serde(rename = "associatedAnatomy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#associated_anatomy: Vec<AssociatedAnatomyProperty>,
	/// <https://schema.org/cause>
	#[cfg_attr(feature = "serde", serde(rename = "cause"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cause: Vec<CauseProperty>,
	/// <https://schema.org/differentialDiagnosis>
	#[cfg_attr(feature = "serde", serde(rename = "differentialDiagnosis"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#differential_diagnosis: Vec<DifferentialDiagnosisProperty>,
	/// <https://schema.org/drug>
	#[cfg_attr(feature = "serde", serde(rename = "drug"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#drug: Vec<DrugProperty>,
	/// <https://schema.org/epidemiology>
	#[cfg_attr(feature = "serde", serde(rename = "epidemiology"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#epidemiology: Vec<EpidemiologyProperty>,
	/// <https://schema.org/expectedPrognosis>
	#[cfg_attr(feature = "serde", serde(rename = "expectedPrognosis"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#expected_prognosis: Vec<ExpectedPrognosisProperty>,
	/// <https://schema.org/naturalProgression>
	#[cfg_attr(feature = "serde", serde(rename = "naturalProgression"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#natural_progression: Vec<NaturalProgressionProperty>,
	/// <https://schema.org/pathophysiology>
	#[cfg_attr(feature = "serde", serde(rename = "pathophysiology"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#pathophysiology: Vec<PathophysiologyProperty>,
	/// <https://schema.org/possibleComplication>
	#[cfg_attr(feature = "serde", serde(rename = "possibleComplication"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#possible_complication: Vec<PossibleComplicationProperty>,
	/// <https://schema.org/possibleTreatment>
	#[cfg_attr(feature = "serde", serde(rename = "possibleTreatment"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#possible_treatment: Vec<PossibleTreatmentProperty>,
	/// <https://schema.org/primaryPrevention>
	#[cfg_attr(feature = "serde", serde(rename = "primaryPrevention"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#primary_prevention: Vec<PrimaryPreventionProperty>,
	/// <https://schema.org/riskFactor>
	#[cfg_attr(feature = "serde", serde(rename = "riskFactor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#risk_factor: Vec<RiskFactorProperty>,
	/// <https://schema.org/secondaryPrevention>
	#[cfg_attr(feature = "serde", serde(rename = "secondaryPrevention"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#secondary_prevention: Vec<SecondaryPreventionProperty>,
	/// <https://schema.org/signOrSymptom>
	#[cfg_attr(feature = "serde", serde(rename = "signOrSymptom"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sign_or_symptom: Vec<SignOrSymptomProperty>,
	/// <https://schema.org/stage>
	#[cfg_attr(feature = "serde", serde(rename = "stage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#stage: Vec<StageProperty>,
	/// <https://schema.org/status>
	#[cfg_attr(feature = "serde", serde(rename = "status"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#status: Vec<StatusProperty>,
	/// <https://schema.org/typicalTest>
	#[cfg_attr(feature = "serde", serde(rename = "typicalTest"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#typical_test: Vec<TypicalTestProperty>,
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
/// This trait is for properties from <https://schema.org/MedicalSign>.
pub trait MedicalSignTrait {
	/// Get <https://schema.org/identifyingExam> from [`Self`] as borrowed slice.
	fn r#identifying_exam(&self) -> &[IdentifyingExamProperty];
	/// Get <https://schema.org/identifyingTest> from [`Self`] as borrowed slice.
	fn r#identifying_test(&self) -> &[IdentifyingTestProperty];
}
impl MedicalSignTrait for MedicalSign {
	fn r#identifying_exam(&self) -> &[IdentifyingExamProperty] {
		self.r#identifying_exam.as_slice()
	}
	fn r#identifying_test(&self) -> &[IdentifyingTestProperty] {
		self.r#identifying_test.as_slice()
	}
}
impl MedicalConditionTrait for MedicalSign {
	fn r#associated_anatomy(&self) -> &[AssociatedAnatomyProperty] {
		self.r#associated_anatomy.as_slice()
	}
	fn r#cause(&self) -> &[CauseProperty] {
		self.r#cause.as_slice()
	}
	fn r#differential_diagnosis(&self) -> &[DifferentialDiagnosisProperty] {
		self.r#differential_diagnosis.as_slice()
	}
	fn r#drug(&self) -> &[DrugProperty] {
		self.r#drug.as_slice()
	}
	fn r#epidemiology(&self) -> &[EpidemiologyProperty] {
		self.r#epidemiology.as_slice()
	}
	fn r#expected_prognosis(&self) -> &[ExpectedPrognosisProperty] {
		self.r#expected_prognosis.as_slice()
	}
	fn r#natural_progression(&self) -> &[NaturalProgressionProperty] {
		self.r#natural_progression.as_slice()
	}
	fn r#pathophysiology(&self) -> &[PathophysiologyProperty] {
		self.r#pathophysiology.as_slice()
	}
	fn r#possible_complication(&self) -> &[PossibleComplicationProperty] {
		self.r#possible_complication.as_slice()
	}
	fn r#possible_treatment(&self) -> &[PossibleTreatmentProperty] {
		self.r#possible_treatment.as_slice()
	}
	fn r#primary_prevention(&self) -> &[PrimaryPreventionProperty] {
		self.r#primary_prevention.as_slice()
	}
	fn r#risk_factor(&self) -> &[RiskFactorProperty] {
		self.r#risk_factor.as_slice()
	}
	fn r#secondary_prevention(&self) -> &[SecondaryPreventionProperty] {
		self.r#secondary_prevention.as_slice()
	}
	fn r#sign_or_symptom(&self) -> &[SignOrSymptomProperty] {
		self.r#sign_or_symptom.as_slice()
	}
	fn r#stage(&self) -> &[StageProperty] {
		self.r#stage.as_slice()
	}
	fn r#status(&self) -> &[StatusProperty] {
		self.r#status.as_slice()
	}
	fn r#typical_test(&self) -> &[TypicalTestProperty] {
		self.r#typical_test.as_slice()
	}
}
impl MedicalEntityTrait for MedicalSign {
	fn r#code(&self) -> &[CodeProperty] {
		self.r#code.as_slice()
	}
	fn r#funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn r#guideline(&self) -> &[GuidelineProperty] {
		self.r#guideline.as_slice()
	}
	fn r#legal_status(&self) -> &[LegalStatusProperty] {
		self.r#legal_status.as_slice()
	}
	fn r#medicine_system(&self) -> &[MedicineSystemProperty] {
		self.r#medicine_system.as_slice()
	}
	fn r#recognizing_authority(&self) -> &[RecognizingAuthorityProperty] {
		self.r#recognizing_authority.as_slice()
	}
	fn r#relevant_specialty(&self) -> &[RelevantSpecialtyProperty] {
		self.r#relevant_specialty.as_slice()
	}
	fn r#study(&self) -> &[StudyProperty] {
		self.r#study.as_slice()
	}
}
impl MedicalSignOrSymptomTrait for MedicalSign {
	fn r#possible_treatment(&self) -> &[PossibleTreatmentProperty] {
		self.r#possible_treatment.as_slice()
	}
}
impl ThingTrait for MedicalSign {
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
