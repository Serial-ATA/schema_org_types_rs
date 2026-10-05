use super::*;
/// <https://schema.org/WorkBasedProgram>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct WorkBasedProgram {
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
/// This trait is for properties from <https://schema.org/WorkBasedProgram>.
pub trait WorkBasedProgramTrait {
	/// Get <https://schema.org/occupationalCategory> from [`Self`] as borrowed slice.
	fn r#occupational_category(&self) -> &[OccupationalCategoryProperty];
	/// Get <https://schema.org/trainingSalary> from [`Self`] as borrowed slice.
	fn r#training_salary(&self) -> &[TrainingSalaryProperty];
}
impl WorkBasedProgramTrait for WorkBasedProgram {
	fn r#occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn r#training_salary(&self) -> &[TrainingSalaryProperty] {
		self.r#training_salary.as_slice()
	}
}
impl EducationalOccupationalProgramTrait for WorkBasedProgram {
	fn r#application_deadline(&self) -> &[ApplicationDeadlineProperty] {
		self.r#application_deadline.as_slice()
	}
	fn r#application_start_date(&self) -> &[ApplicationStartDateProperty] {
		self.r#application_start_date.as_slice()
	}
	fn r#day_of_week(&self) -> &[DayOfWeekProperty] {
		self.r#day_of_week.as_slice()
	}
	fn r#educational_credential_awarded(&self) -> &[EducationalCredentialAwardedProperty] {
		self.r#educational_credential_awarded.as_slice()
	}
	fn r#educational_program_mode(&self) -> &[EducationalProgramModeProperty] {
		self.r#educational_program_mode.as_slice()
	}
	fn r#end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn r#financial_aid_eligible(&self) -> &[FinancialAidEligibleProperty] {
		self.r#financial_aid_eligible.as_slice()
	}
	fn r#has_course(&self) -> &[HasCourseProperty] {
		self.r#has_course.as_slice()
	}
	fn r#maximum_enrollment(&self) -> &[MaximumEnrollmentProperty] {
		self.r#maximum_enrollment.as_slice()
	}
	fn r#number_of_credits(&self) -> &[NumberOfCreditsProperty] {
		self.r#number_of_credits.as_slice()
	}
	fn r#occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn r#occupational_credential_awarded(&self) -> &[OccupationalCredentialAwardedProperty] {
		self.r#occupational_credential_awarded.as_slice()
	}
	fn r#offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn r#program_prerequisites(&self) -> &[ProgramPrerequisitesProperty] {
		self.r#program_prerequisites.as_slice()
	}
	fn r#program_type(&self) -> &[ProgramTypeProperty] {
		self.r#program_type.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#salary_upon_completion(&self) -> &[SalaryUponCompletionProperty] {
		self.r#salary_upon_completion.as_slice()
	}
	fn r#start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn r#term_duration(&self) -> &[TermDurationProperty] {
		self.r#term_duration.as_slice()
	}
	fn r#terms_per_year(&self) -> &[TermsPerYearProperty] {
		self.r#terms_per_year.as_slice()
	}
	fn r#time_of_day(&self) -> &[TimeOfDayProperty] {
		self.r#time_of_day.as_slice()
	}
	fn r#time_to_complete(&self) -> &[TimeToCompleteProperty] {
		self.r#time_to_complete.as_slice()
	}
	fn r#training_salary(&self) -> &[TrainingSalaryProperty] {
		self.r#training_salary.as_slice()
	}
	fn r#typical_credits_per_term(&self) -> &[TypicalCreditsPerTermProperty] {
		self.r#typical_credits_per_term.as_slice()
	}
}
impl ThingTrait for WorkBasedProgram {
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
