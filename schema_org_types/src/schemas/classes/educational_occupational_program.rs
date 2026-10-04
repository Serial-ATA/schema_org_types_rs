use super::*;
/// <https://schema.org/EducationalOccupationalProgram>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct EducationalOccupationalProgram {
	/// <https://schema.org/applicationDeadline>
	#[cfg_attr(feature = "serde", serde(rename = "applicationDeadline"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#application_deadline: Vec<ApplicationDeadlineProperty>,
	/// <https://schema.org/applicationStartDate>
	#[cfg_attr(feature = "serde", serde(rename = "applicationStartDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#application_start_date: Vec<ApplicationStartDateProperty>,
	/// <https://schema.org/dayOfWeek>
	#[cfg_attr(feature = "serde", serde(rename = "dayOfWeek"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#day_of_week: Vec<DayOfWeekProperty>,
	/// <https://schema.org/educationalCredentialAwarded>
	#[cfg_attr(feature = "serde", serde(rename = "educationalCredentialAwarded"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#educational_credential_awarded: Vec<EducationalCredentialAwardedProperty>,
	/// <https://schema.org/educationalProgramMode>
	#[cfg_attr(feature = "serde", serde(rename = "educationalProgramMode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#educational_program_mode: Vec<EducationalProgramModeProperty>,
	/// <https://schema.org/endDate>
	#[cfg_attr(feature = "serde", serde(rename = "endDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#end_date: Vec<EndDateProperty>,
	/// <https://schema.org/financialAidEligible>
	#[cfg_attr(feature = "serde", serde(rename = "financialAidEligible"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#financial_aid_eligible: Vec<FinancialAidEligibleProperty>,
	/// <https://schema.org/hasCourse>
	#[cfg_attr(feature = "serde", serde(rename = "hasCourse"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_course: Vec<HasCourseProperty>,
	/// <https://schema.org/maximumEnrollment>
	#[cfg_attr(feature = "serde", serde(rename = "maximumEnrollment"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#maximum_enrollment: Vec<MaximumEnrollmentProperty>,
	/// <https://schema.org/numberOfCredits>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfCredits"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_credits: Vec<NumberOfCreditsProperty>,
	/// <https://schema.org/occupationalCategory>
	#[cfg_attr(feature = "serde", serde(rename = "occupationalCategory"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#occupational_category: Vec<OccupationalCategoryProperty>,
	/// <https://schema.org/occupationalCredentialAwarded>
	#[cfg_attr(feature = "serde", serde(rename = "occupationalCredentialAwarded"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#occupational_credential_awarded: Vec<OccupationalCredentialAwardedProperty>,
	/// <https://schema.org/offers>
	#[cfg_attr(feature = "serde", serde(rename = "offers"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#offers: Vec<OffersProperty>,
	/// <https://schema.org/programPrerequisites>
	#[cfg_attr(feature = "serde", serde(rename = "programPrerequisites"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#program_prerequisites: Vec<ProgramPrerequisitesProperty>,
	/// <https://schema.org/programType>
	#[cfg_attr(feature = "serde", serde(rename = "programType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#program_type: Vec<ProgramTypeProperty>,
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
	/// <https://schema.org/salaryUponCompletion>
	#[cfg_attr(feature = "serde", serde(rename = "salaryUponCompletion"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#salary_upon_completion: Vec<SalaryUponCompletionProperty>,
	/// <https://schema.org/startDate>
	#[cfg_attr(feature = "serde", serde(rename = "startDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#start_date: Vec<StartDateProperty>,
	/// <https://schema.org/termDuration>
	#[cfg_attr(feature = "serde", serde(rename = "termDuration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#term_duration: Vec<TermDurationProperty>,
	/// <https://schema.org/termsPerYear>
	#[cfg_attr(feature = "serde", serde(rename = "termsPerYear"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#terms_per_year: Vec<TermsPerYearProperty>,
	/// <https://schema.org/timeOfDay>
	#[cfg_attr(feature = "serde", serde(rename = "timeOfDay"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#time_of_day: Vec<TimeOfDayProperty>,
	/// <https://schema.org/timeToComplete>
	#[cfg_attr(feature = "serde", serde(rename = "timeToComplete"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#time_to_complete: Vec<TimeToCompleteProperty>,
	/// <https://schema.org/trainingSalary>
	#[cfg_attr(feature = "serde", serde(rename = "trainingSalary"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#training_salary: Vec<TrainingSalaryProperty>,
	/// <https://schema.org/typicalCreditsPerTerm>
	#[cfg_attr(feature = "serde", serde(rename = "typicalCreditsPerTerm"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#typical_credits_per_term: Vec<TypicalCreditsPerTermProperty>,
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
/// This trait is for properties from <https://schema.org/EducationalOccupationalProgram>.
pub trait EducationalOccupationalProgramTrait {
	/// Get <https://schema.org/applicationDeadline> from [`Self`] as borrowed slice.
	fn get_application_deadline(&self) -> &[ApplicationDeadlineProperty];
	/// Take <https://schema.org/applicationDeadline> from [`Self`] as owned vector.
	fn take_application_deadline(&mut self) -> Vec<ApplicationDeadlineProperty>;
	/// Get <https://schema.org/applicationStartDate> from [`Self`] as borrowed slice.
	fn get_application_start_date(&self) -> &[ApplicationStartDateProperty];
	/// Take <https://schema.org/applicationStartDate> from [`Self`] as owned vector.
	fn take_application_start_date(&mut self) -> Vec<ApplicationStartDateProperty>;
	/// Get <https://schema.org/dayOfWeek> from [`Self`] as borrowed slice.
	fn get_day_of_week(&self) -> &[DayOfWeekProperty];
	/// Take <https://schema.org/dayOfWeek> from [`Self`] as owned vector.
	fn take_day_of_week(&mut self) -> Vec<DayOfWeekProperty>;
	/// Get <https://schema.org/educationalCredentialAwarded> from [`Self`] as borrowed slice.
	fn get_educational_credential_awarded(&self) -> &[EducationalCredentialAwardedProperty];
	/// Take <https://schema.org/educationalCredentialAwarded> from [`Self`] as owned vector.
	fn take_educational_credential_awarded(&mut self) -> Vec<EducationalCredentialAwardedProperty>;
	/// Get <https://schema.org/educationalProgramMode> from [`Self`] as borrowed slice.
	fn get_educational_program_mode(&self) -> &[EducationalProgramModeProperty];
	/// Take <https://schema.org/educationalProgramMode> from [`Self`] as owned vector.
	fn take_educational_program_mode(&mut self) -> Vec<EducationalProgramModeProperty>;
	/// Get <https://schema.org/endDate> from [`Self`] as borrowed slice.
	fn get_end_date(&self) -> &[EndDateProperty];
	/// Take <https://schema.org/endDate> from [`Self`] as owned vector.
	fn take_end_date(&mut self) -> Vec<EndDateProperty>;
	/// Get <https://schema.org/financialAidEligible> from [`Self`] as borrowed slice.
	fn get_financial_aid_eligible(&self) -> &[FinancialAidEligibleProperty];
	/// Take <https://schema.org/financialAidEligible> from [`Self`] as owned vector.
	fn take_financial_aid_eligible(&mut self) -> Vec<FinancialAidEligibleProperty>;
	/// Get <https://schema.org/hasCourse> from [`Self`] as borrowed slice.
	fn get_has_course(&self) -> &[HasCourseProperty];
	/// Take <https://schema.org/hasCourse> from [`Self`] as owned vector.
	fn take_has_course(&mut self) -> Vec<HasCourseProperty>;
	/// Get <https://schema.org/maximumEnrollment> from [`Self`] as borrowed slice.
	fn get_maximum_enrollment(&self) -> &[MaximumEnrollmentProperty];
	/// Take <https://schema.org/maximumEnrollment> from [`Self`] as owned vector.
	fn take_maximum_enrollment(&mut self) -> Vec<MaximumEnrollmentProperty>;
	/// Get <https://schema.org/numberOfCredits> from [`Self`] as borrowed slice.
	fn get_number_of_credits(&self) -> &[NumberOfCreditsProperty];
	/// Take <https://schema.org/numberOfCredits> from [`Self`] as owned vector.
	fn take_number_of_credits(&mut self) -> Vec<NumberOfCreditsProperty>;
	/// Get <https://schema.org/occupationalCategory> from [`Self`] as borrowed slice.
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty];
	/// Take <https://schema.org/occupationalCategory> from [`Self`] as owned vector.
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty>;
	/// Get <https://schema.org/occupationalCredentialAwarded> from [`Self`] as borrowed slice.
	fn get_occupational_credential_awarded(&self) -> &[OccupationalCredentialAwardedProperty];
	/// Take <https://schema.org/occupationalCredentialAwarded> from [`Self`] as owned vector.
	fn take_occupational_credential_awarded(
		&mut self,
	) -> Vec<OccupationalCredentialAwardedProperty>;
	/// Get <https://schema.org/offers> from [`Self`] as borrowed slice.
	fn get_offers(&self) -> &[OffersProperty];
	/// Take <https://schema.org/offers> from [`Self`] as owned vector.
	fn take_offers(&mut self) -> Vec<OffersProperty>;
	/// Get <https://schema.org/programPrerequisites> from [`Self`] as borrowed slice.
	fn get_program_prerequisites(&self) -> &[ProgramPrerequisitesProperty];
	/// Take <https://schema.org/programPrerequisites> from [`Self`] as owned vector.
	fn take_program_prerequisites(&mut self) -> Vec<ProgramPrerequisitesProperty>;
	/// Get <https://schema.org/programType> from [`Self`] as borrowed slice.
	fn get_program_type(&self) -> &[ProgramTypeProperty];
	/// Take <https://schema.org/programType> from [`Self`] as owned vector.
	fn take_program_type(&mut self) -> Vec<ProgramTypeProperty>;
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn get_provider(&self) -> &[ProviderProperty];
	/// Take <https://schema.org/provider> from [`Self`] as owned vector.
	fn take_provider(&mut self) -> Vec<ProviderProperty>;
	/// Get <https://schema.org/salaryUponCompletion> from [`Self`] as borrowed slice.
	fn get_salary_upon_completion(&self) -> &[SalaryUponCompletionProperty];
	/// Take <https://schema.org/salaryUponCompletion> from [`Self`] as owned vector.
	fn take_salary_upon_completion(&mut self) -> Vec<SalaryUponCompletionProperty>;
	/// Get <https://schema.org/startDate> from [`Self`] as borrowed slice.
	fn get_start_date(&self) -> &[StartDateProperty];
	/// Take <https://schema.org/startDate> from [`Self`] as owned vector.
	fn take_start_date(&mut self) -> Vec<StartDateProperty>;
	/// Get <https://schema.org/termDuration> from [`Self`] as borrowed slice.
	fn get_term_duration(&self) -> &[TermDurationProperty];
	/// Take <https://schema.org/termDuration> from [`Self`] as owned vector.
	fn take_term_duration(&mut self) -> Vec<TermDurationProperty>;
	/// Get <https://schema.org/termsPerYear> from [`Self`] as borrowed slice.
	fn get_terms_per_year(&self) -> &[TermsPerYearProperty];
	/// Take <https://schema.org/termsPerYear> from [`Self`] as owned vector.
	fn take_terms_per_year(&mut self) -> Vec<TermsPerYearProperty>;
	/// Get <https://schema.org/timeOfDay> from [`Self`] as borrowed slice.
	fn get_time_of_day(&self) -> &[TimeOfDayProperty];
	/// Take <https://schema.org/timeOfDay> from [`Self`] as owned vector.
	fn take_time_of_day(&mut self) -> Vec<TimeOfDayProperty>;
	/// Get <https://schema.org/timeToComplete> from [`Self`] as borrowed slice.
	fn get_time_to_complete(&self) -> &[TimeToCompleteProperty];
	/// Take <https://schema.org/timeToComplete> from [`Self`] as owned vector.
	fn take_time_to_complete(&mut self) -> Vec<TimeToCompleteProperty>;
	/// Get <https://schema.org/trainingSalary> from [`Self`] as borrowed slice.
	fn get_training_salary(&self) -> &[TrainingSalaryProperty];
	/// Take <https://schema.org/trainingSalary> from [`Self`] as owned vector.
	fn take_training_salary(&mut self) -> Vec<TrainingSalaryProperty>;
	/// Get <https://schema.org/typicalCreditsPerTerm> from [`Self`] as borrowed slice.
	fn get_typical_credits_per_term(&self) -> &[TypicalCreditsPerTermProperty];
	/// Take <https://schema.org/typicalCreditsPerTerm> from [`Self`] as owned vector.
	fn take_typical_credits_per_term(&mut self) -> Vec<TypicalCreditsPerTermProperty>;
}
impl EducationalOccupationalProgramTrait for EducationalOccupationalProgram {
	fn get_application_deadline(&self) -> &[ApplicationDeadlineProperty] {
		self.r#application_deadline.as_slice()
	}
	fn take_application_deadline(&mut self) -> Vec<ApplicationDeadlineProperty> {
		std::mem::take(&mut self.r#application_deadline)
	}
	fn get_application_start_date(&self) -> &[ApplicationStartDateProperty] {
		self.r#application_start_date.as_slice()
	}
	fn take_application_start_date(&mut self) -> Vec<ApplicationStartDateProperty> {
		std::mem::take(&mut self.r#application_start_date)
	}
	fn get_day_of_week(&self) -> &[DayOfWeekProperty] {
		self.r#day_of_week.as_slice()
	}
	fn take_day_of_week(&mut self) -> Vec<DayOfWeekProperty> {
		std::mem::take(&mut self.r#day_of_week)
	}
	fn get_educational_credential_awarded(&self) -> &[EducationalCredentialAwardedProperty] {
		self.r#educational_credential_awarded.as_slice()
	}
	fn take_educational_credential_awarded(&mut self) -> Vec<EducationalCredentialAwardedProperty> {
		std::mem::take(&mut self.r#educational_credential_awarded)
	}
	fn get_educational_program_mode(&self) -> &[EducationalProgramModeProperty] {
		self.r#educational_program_mode.as_slice()
	}
	fn take_educational_program_mode(&mut self) -> Vec<EducationalProgramModeProperty> {
		std::mem::take(&mut self.r#educational_program_mode)
	}
	fn get_end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn take_end_date(&mut self) -> Vec<EndDateProperty> {
		std::mem::take(&mut self.r#end_date)
	}
	fn get_financial_aid_eligible(&self) -> &[FinancialAidEligibleProperty] {
		self.r#financial_aid_eligible.as_slice()
	}
	fn take_financial_aid_eligible(&mut self) -> Vec<FinancialAidEligibleProperty> {
		std::mem::take(&mut self.r#financial_aid_eligible)
	}
	fn get_has_course(&self) -> &[HasCourseProperty] {
		self.r#has_course.as_slice()
	}
	fn take_has_course(&mut self) -> Vec<HasCourseProperty> {
		std::mem::take(&mut self.r#has_course)
	}
	fn get_maximum_enrollment(&self) -> &[MaximumEnrollmentProperty] {
		self.r#maximum_enrollment.as_slice()
	}
	fn take_maximum_enrollment(&mut self) -> Vec<MaximumEnrollmentProperty> {
		std::mem::take(&mut self.r#maximum_enrollment)
	}
	fn get_number_of_credits(&self) -> &[NumberOfCreditsProperty] {
		self.r#number_of_credits.as_slice()
	}
	fn take_number_of_credits(&mut self) -> Vec<NumberOfCreditsProperty> {
		std::mem::take(&mut self.r#number_of_credits)
	}
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty> {
		std::mem::take(&mut self.r#occupational_category)
	}
	fn get_occupational_credential_awarded(&self) -> &[OccupationalCredentialAwardedProperty] {
		self.r#occupational_credential_awarded.as_slice()
	}
	fn take_occupational_credential_awarded(
		&mut self,
	) -> Vec<OccupationalCredentialAwardedProperty> {
		std::mem::take(&mut self.r#occupational_credential_awarded)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_program_prerequisites(&self) -> &[ProgramPrerequisitesProperty] {
		self.r#program_prerequisites.as_slice()
	}
	fn take_program_prerequisites(&mut self) -> Vec<ProgramPrerequisitesProperty> {
		std::mem::take(&mut self.r#program_prerequisites)
	}
	fn get_program_type(&self) -> &[ProgramTypeProperty] {
		self.r#program_type.as_slice()
	}
	fn take_program_type(&mut self) -> Vec<ProgramTypeProperty> {
		std::mem::take(&mut self.r#program_type)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_salary_upon_completion(&self) -> &[SalaryUponCompletionProperty] {
		self.r#salary_upon_completion.as_slice()
	}
	fn take_salary_upon_completion(&mut self) -> Vec<SalaryUponCompletionProperty> {
		std::mem::take(&mut self.r#salary_upon_completion)
	}
	fn get_start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn take_start_date(&mut self) -> Vec<StartDateProperty> {
		std::mem::take(&mut self.r#start_date)
	}
	fn get_term_duration(&self) -> &[TermDurationProperty] {
		self.r#term_duration.as_slice()
	}
	fn take_term_duration(&mut self) -> Vec<TermDurationProperty> {
		std::mem::take(&mut self.r#term_duration)
	}
	fn get_terms_per_year(&self) -> &[TermsPerYearProperty] {
		self.r#terms_per_year.as_slice()
	}
	fn take_terms_per_year(&mut self) -> Vec<TermsPerYearProperty> {
		std::mem::take(&mut self.r#terms_per_year)
	}
	fn get_time_of_day(&self) -> &[TimeOfDayProperty] {
		self.r#time_of_day.as_slice()
	}
	fn take_time_of_day(&mut self) -> Vec<TimeOfDayProperty> {
		std::mem::take(&mut self.r#time_of_day)
	}
	fn get_time_to_complete(&self) -> &[TimeToCompleteProperty] {
		self.r#time_to_complete.as_slice()
	}
	fn take_time_to_complete(&mut self) -> Vec<TimeToCompleteProperty> {
		std::mem::take(&mut self.r#time_to_complete)
	}
	fn get_training_salary(&self) -> &[TrainingSalaryProperty] {
		self.r#training_salary.as_slice()
	}
	fn take_training_salary(&mut self) -> Vec<TrainingSalaryProperty> {
		std::mem::take(&mut self.r#training_salary)
	}
	fn get_typical_credits_per_term(&self) -> &[TypicalCreditsPerTermProperty] {
		self.r#typical_credits_per_term.as_slice()
	}
	fn take_typical_credits_per_term(&mut self) -> Vec<TypicalCreditsPerTermProperty> {
		std::mem::take(&mut self.r#typical_credits_per_term)
	}
}
impl ThingTrait for EducationalOccupationalProgram {
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
