use super::*;
/// <https://schema.org/JobPosting>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct JobPosting {
	/// <https://schema.org/applicantLocationRequirements>
	#[cfg_attr(feature = "serde", serde(rename = "applicantLocationRequirements"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#applicant_location_requirements: Vec<ApplicantLocationRequirementsProperty>,
	/// <https://schema.org/applicationContact>
	#[cfg_attr(feature = "serde", serde(rename = "applicationContact"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#application_contact: Vec<ApplicationContactProperty>,
	/// <https://schema.org/baseSalary>
	#[cfg_attr(feature = "serde", serde(rename = "baseSalary"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#base_salary: Vec<BaseSalaryProperty>,
	/// <https://schema.org/benefits>
	#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
	#[cfg_attr(feature = "serde", serde(rename = "benefits"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#benefits: Vec<BenefitsProperty>,
	/// <https://schema.org/datePosted>
	#[cfg_attr(feature = "serde", serde(rename = "datePosted"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#date_posted: Vec<DatePostedProperty>,
	/// <https://schema.org/directApply>
	#[cfg_attr(feature = "serde", serde(rename = "directApply"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#direct_apply: Vec<DirectApplyProperty>,
	/// <https://schema.org/educationRequirements>
	#[cfg_attr(feature = "serde", serde(rename = "educationRequirements"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#education_requirements: Vec<EducationRequirementsProperty>,
	/// <https://schema.org/eligibilityToWorkRequirement>
	#[cfg_attr(feature = "serde", serde(rename = "eligibilityToWorkRequirement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligibility_to_work_requirement: Vec<EligibilityToWorkRequirementProperty>,
	/// <https://schema.org/employerOverview>
	#[cfg_attr(feature = "serde", serde(rename = "employerOverview"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#employer_overview: Vec<EmployerOverviewProperty>,
	/// <https://schema.org/employmentType>
	#[cfg_attr(feature = "serde", serde(rename = "employmentType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#employment_type: Vec<EmploymentTypeProperty>,
	/// <https://schema.org/employmentUnit>
	#[cfg_attr(feature = "serde", serde(rename = "employmentUnit"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#employment_unit: Vec<EmploymentUnitProperty>,
	/// <https://schema.org/estimatedSalary>
	#[cfg_attr(feature = "serde", serde(rename = "estimatedSalary"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#estimated_salary: Vec<EstimatedSalaryProperty>,
	/// <https://schema.org/experienceInPlaceOfEducation>
	#[cfg_attr(feature = "serde", serde(rename = "experienceInPlaceOfEducation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#experience_in_place_of_education: Vec<ExperienceInPlaceOfEducationProperty>,
	/// <https://schema.org/experienceRequirements>
	#[cfg_attr(feature = "serde", serde(rename = "experienceRequirements"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#experience_requirements: Vec<ExperienceRequirementsProperty>,
	/// <https://schema.org/hiringOrganization>
	#[cfg_attr(feature = "serde", serde(rename = "hiringOrganization"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#hiring_organization: Vec<HiringOrganizationProperty>,
	/// <https://schema.org/incentiveCompensation>
	#[cfg_attr(feature = "serde", serde(rename = "incentiveCompensation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#incentive_compensation: Vec<IncentiveCompensationProperty>,
	/// <https://schema.org/incentives>
	#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
	#[cfg_attr(feature = "serde", serde(rename = "incentives"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#incentives: Vec<IncentivesProperty>,
	/// <https://schema.org/industry>
	#[cfg_attr(feature = "serde", serde(rename = "industry"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#industry: Vec<IndustryProperty>,
	/// <https://schema.org/jobBenefits>
	#[cfg_attr(feature = "serde", serde(rename = "jobBenefits"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_benefits: Vec<JobBenefitsProperty>,
	/// <https://schema.org/jobDuration>
	#[cfg_attr(feature = "serde", serde(rename = "jobDuration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_duration: Vec<JobDurationProperty>,
	/// <https://schema.org/jobImmediateStart>
	#[cfg_attr(feature = "serde", serde(rename = "jobImmediateStart"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_immediate_start: Vec<JobImmediateStartProperty>,
	/// <https://schema.org/jobLocation>
	#[cfg_attr(feature = "serde", serde(rename = "jobLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_location: Vec<JobLocationProperty>,
	/// <https://schema.org/jobLocationType>
	#[cfg_attr(feature = "serde", serde(rename = "jobLocationType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_location_type: Vec<JobLocationTypeProperty>,
	/// <https://schema.org/jobStartDate>
	#[cfg_attr(feature = "serde", serde(rename = "jobStartDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_start_date: Vec<JobStartDateProperty>,
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
	/// <https://schema.org/physicalRequirement>
	#[cfg_attr(feature = "serde", serde(rename = "physicalRequirement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#physical_requirement: Vec<PhysicalRequirementProperty>,
	/// <https://schema.org/qualifications>
	#[cfg_attr(feature = "serde", serde(rename = "qualifications"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#qualifications: Vec<QualificationsProperty>,
	/// <https://schema.org/relevantOccupation>
	#[cfg_attr(feature = "serde", serde(rename = "relevantOccupation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#relevant_occupation: Vec<RelevantOccupationProperty>,
	/// <https://schema.org/responsibilities>
	#[cfg_attr(feature = "serde", serde(rename = "responsibilities"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#responsibilities: Vec<ResponsibilitiesProperty>,
	/// <https://schema.org/salaryCurrency>
	#[cfg_attr(feature = "serde", serde(rename = "salaryCurrency"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#salary_currency: Vec<SalaryCurrencyProperty>,
	/// <https://schema.org/securityClearanceRequirement>
	#[cfg_attr(feature = "serde", serde(rename = "securityClearanceRequirement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#security_clearance_requirement: Vec<SecurityClearanceRequirementProperty>,
	/// <https://schema.org/sensoryRequirement>
	#[cfg_attr(feature = "serde", serde(rename = "sensoryRequirement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sensory_requirement: Vec<SensoryRequirementProperty>,
	/// <https://schema.org/skills>
	#[cfg_attr(feature = "serde", serde(rename = "skills"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#skills: Vec<SkillsProperty>,
	/// <https://schema.org/specialCommitments>
	#[cfg_attr(feature = "serde", serde(rename = "specialCommitments"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#special_commitments: Vec<SpecialCommitmentsProperty>,
	/// <https://schema.org/title>
	#[cfg_attr(feature = "serde", serde(rename = "title"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#title: Vec<TitleProperty>,
	/// <https://schema.org/totalJobOpenings>
	#[cfg_attr(feature = "serde", serde(rename = "totalJobOpenings"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#total_job_openings: Vec<TotalJobOpeningsProperty>,
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
	/// <https://schema.org/workHours>
	#[cfg_attr(feature = "serde", serde(rename = "workHours"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#work_hours: Vec<WorkHoursProperty>,
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
/// This trait is for properties from <https://schema.org/JobPosting>.
pub trait JobPostingTrait {
	/// Get <https://schema.org/applicantLocationRequirements> from [`Self`] as borrowed slice.
	fn get_applicant_location_requirements(&self) -> &[ApplicantLocationRequirementsProperty];
	/// Take <https://schema.org/applicantLocationRequirements> from [`Self`] as owned vector.
	fn take_applicant_location_requirements(
		&mut self,
	) -> Vec<ApplicantLocationRequirementsProperty>;
	/// Get <https://schema.org/applicationContact> from [`Self`] as borrowed slice.
	fn get_application_contact(&self) -> &[ApplicationContactProperty];
	/// Take <https://schema.org/applicationContact> from [`Self`] as owned vector.
	fn take_application_contact(&mut self) -> Vec<ApplicationContactProperty>;
	/// Get <https://schema.org/baseSalary> from [`Self`] as borrowed slice.
	fn get_base_salary(&self) -> &[BaseSalaryProperty];
	/// Take <https://schema.org/baseSalary> from [`Self`] as owned vector.
	fn take_base_salary(&mut self) -> Vec<BaseSalaryProperty>;
	/// Get <https://schema.org/benefits> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
	fn get_benefits(&self) -> &[BenefitsProperty];
	/// Take <https://schema.org/benefits> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
	fn take_benefits(&mut self) -> Vec<BenefitsProperty>;
	/// Get <https://schema.org/datePosted> from [`Self`] as borrowed slice.
	fn get_date_posted(&self) -> &[DatePostedProperty];
	/// Take <https://schema.org/datePosted> from [`Self`] as owned vector.
	fn take_date_posted(&mut self) -> Vec<DatePostedProperty>;
	/// Get <https://schema.org/directApply> from [`Self`] as borrowed slice.
	fn get_direct_apply(&self) -> &[DirectApplyProperty];
	/// Take <https://schema.org/directApply> from [`Self`] as owned vector.
	fn take_direct_apply(&mut self) -> Vec<DirectApplyProperty>;
	/// Get <https://schema.org/educationRequirements> from [`Self`] as borrowed slice.
	fn get_education_requirements(&self) -> &[EducationRequirementsProperty];
	/// Take <https://schema.org/educationRequirements> from [`Self`] as owned vector.
	fn take_education_requirements(&mut self) -> Vec<EducationRequirementsProperty>;
	/// Get <https://schema.org/eligibilityToWorkRequirement> from [`Self`] as borrowed slice.
	fn get_eligibility_to_work_requirement(&self) -> &[EligibilityToWorkRequirementProperty];
	/// Take <https://schema.org/eligibilityToWorkRequirement> from [`Self`] as owned vector.
	fn take_eligibility_to_work_requirement(&mut self)
	-> Vec<EligibilityToWorkRequirementProperty>;
	/// Get <https://schema.org/employerOverview> from [`Self`] as borrowed slice.
	fn get_employer_overview(&self) -> &[EmployerOverviewProperty];
	/// Take <https://schema.org/employerOverview> from [`Self`] as owned vector.
	fn take_employer_overview(&mut self) -> Vec<EmployerOverviewProperty>;
	/// Get <https://schema.org/employmentType> from [`Self`] as borrowed slice.
	fn get_employment_type(&self) -> &[EmploymentTypeProperty];
	/// Take <https://schema.org/employmentType> from [`Self`] as owned vector.
	fn take_employment_type(&mut self) -> Vec<EmploymentTypeProperty>;
	/// Get <https://schema.org/employmentUnit> from [`Self`] as borrowed slice.
	fn get_employment_unit(&self) -> &[EmploymentUnitProperty];
	/// Take <https://schema.org/employmentUnit> from [`Self`] as owned vector.
	fn take_employment_unit(&mut self) -> Vec<EmploymentUnitProperty>;
	/// Get <https://schema.org/estimatedSalary> from [`Self`] as borrowed slice.
	fn get_estimated_salary(&self) -> &[EstimatedSalaryProperty];
	/// Take <https://schema.org/estimatedSalary> from [`Self`] as owned vector.
	fn take_estimated_salary(&mut self) -> Vec<EstimatedSalaryProperty>;
	/// Get <https://schema.org/experienceInPlaceOfEducation> from [`Self`] as borrowed slice.
	fn get_experience_in_place_of_education(&self) -> &[ExperienceInPlaceOfEducationProperty];
	/// Take <https://schema.org/experienceInPlaceOfEducation> from [`Self`] as owned vector.
	fn take_experience_in_place_of_education(
		&mut self,
	) -> Vec<ExperienceInPlaceOfEducationProperty>;
	/// Get <https://schema.org/experienceRequirements> from [`Self`] as borrowed slice.
	fn get_experience_requirements(&self) -> &[ExperienceRequirementsProperty];
	/// Take <https://schema.org/experienceRequirements> from [`Self`] as owned vector.
	fn take_experience_requirements(&mut self) -> Vec<ExperienceRequirementsProperty>;
	/// Get <https://schema.org/hiringOrganization> from [`Self`] as borrowed slice.
	fn get_hiring_organization(&self) -> &[HiringOrganizationProperty];
	/// Take <https://schema.org/hiringOrganization> from [`Self`] as owned vector.
	fn take_hiring_organization(&mut self) -> Vec<HiringOrganizationProperty>;
	/// Get <https://schema.org/incentiveCompensation> from [`Self`] as borrowed slice.
	fn get_incentive_compensation(&self) -> &[IncentiveCompensationProperty];
	/// Take <https://schema.org/incentiveCompensation> from [`Self`] as owned vector.
	fn take_incentive_compensation(&mut self) -> Vec<IncentiveCompensationProperty>;
	/// Get <https://schema.org/incentives> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
	fn get_incentives(&self) -> &[IncentivesProperty];
	/// Take <https://schema.org/incentives> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
	fn take_incentives(&mut self) -> Vec<IncentivesProperty>;
	/// Get <https://schema.org/industry> from [`Self`] as borrowed slice.
	fn get_industry(&self) -> &[IndustryProperty];
	/// Take <https://schema.org/industry> from [`Self`] as owned vector.
	fn take_industry(&mut self) -> Vec<IndustryProperty>;
	/// Get <https://schema.org/jobBenefits> from [`Self`] as borrowed slice.
	fn get_job_benefits(&self) -> &[JobBenefitsProperty];
	/// Take <https://schema.org/jobBenefits> from [`Self`] as owned vector.
	fn take_job_benefits(&mut self) -> Vec<JobBenefitsProperty>;
	/// Get <https://schema.org/jobDuration> from [`Self`] as borrowed slice.
	fn get_job_duration(&self) -> &[JobDurationProperty];
	/// Take <https://schema.org/jobDuration> from [`Self`] as owned vector.
	fn take_job_duration(&mut self) -> Vec<JobDurationProperty>;
	/// Get <https://schema.org/jobImmediateStart> from [`Self`] as borrowed slice.
	fn get_job_immediate_start(&self) -> &[JobImmediateStartProperty];
	/// Take <https://schema.org/jobImmediateStart> from [`Self`] as owned vector.
	fn take_job_immediate_start(&mut self) -> Vec<JobImmediateStartProperty>;
	/// Get <https://schema.org/jobLocation> from [`Self`] as borrowed slice.
	fn get_job_location(&self) -> &[JobLocationProperty];
	/// Take <https://schema.org/jobLocation> from [`Self`] as owned vector.
	fn take_job_location(&mut self) -> Vec<JobLocationProperty>;
	/// Get <https://schema.org/jobLocationType> from [`Self`] as borrowed slice.
	fn get_job_location_type(&self) -> &[JobLocationTypeProperty];
	/// Take <https://schema.org/jobLocationType> from [`Self`] as owned vector.
	fn take_job_location_type(&mut self) -> Vec<JobLocationTypeProperty>;
	/// Get <https://schema.org/jobStartDate> from [`Self`] as borrowed slice.
	fn get_job_start_date(&self) -> &[JobStartDateProperty];
	/// Take <https://schema.org/jobStartDate> from [`Self`] as owned vector.
	fn take_job_start_date(&mut self) -> Vec<JobStartDateProperty>;
	/// Get <https://schema.org/occupationalCategory> from [`Self`] as borrowed slice.
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty];
	/// Take <https://schema.org/occupationalCategory> from [`Self`] as owned vector.
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty>;
	/// Get <https://schema.org/physicalRequirement> from [`Self`] as borrowed slice.
	fn get_physical_requirement(&self) -> &[PhysicalRequirementProperty];
	/// Take <https://schema.org/physicalRequirement> from [`Self`] as owned vector.
	fn take_physical_requirement(&mut self) -> Vec<PhysicalRequirementProperty>;
	/// Get <https://schema.org/qualifications> from [`Self`] as borrowed slice.
	fn get_qualifications(&self) -> &[QualificationsProperty];
	/// Take <https://schema.org/qualifications> from [`Self`] as owned vector.
	fn take_qualifications(&mut self) -> Vec<QualificationsProperty>;
	/// Get <https://schema.org/relevantOccupation> from [`Self`] as borrowed slice.
	fn get_relevant_occupation(&self) -> &[RelevantOccupationProperty];
	/// Take <https://schema.org/relevantOccupation> from [`Self`] as owned vector.
	fn take_relevant_occupation(&mut self) -> Vec<RelevantOccupationProperty>;
	/// Get <https://schema.org/responsibilities> from [`Self`] as borrowed slice.
	fn get_responsibilities(&self) -> &[ResponsibilitiesProperty];
	/// Take <https://schema.org/responsibilities> from [`Self`] as owned vector.
	fn take_responsibilities(&mut self) -> Vec<ResponsibilitiesProperty>;
	/// Get <https://schema.org/salaryCurrency> from [`Self`] as borrowed slice.
	fn get_salary_currency(&self) -> &[SalaryCurrencyProperty];
	/// Take <https://schema.org/salaryCurrency> from [`Self`] as owned vector.
	fn take_salary_currency(&mut self) -> Vec<SalaryCurrencyProperty>;
	/// Get <https://schema.org/securityClearanceRequirement> from [`Self`] as borrowed slice.
	fn get_security_clearance_requirement(&self) -> &[SecurityClearanceRequirementProperty];
	/// Take <https://schema.org/securityClearanceRequirement> from [`Self`] as owned vector.
	fn take_security_clearance_requirement(&mut self) -> Vec<SecurityClearanceRequirementProperty>;
	/// Get <https://schema.org/sensoryRequirement> from [`Self`] as borrowed slice.
	fn get_sensory_requirement(&self) -> &[SensoryRequirementProperty];
	/// Take <https://schema.org/sensoryRequirement> from [`Self`] as owned vector.
	fn take_sensory_requirement(&mut self) -> Vec<SensoryRequirementProperty>;
	/// Get <https://schema.org/skills> from [`Self`] as borrowed slice.
	fn get_skills(&self) -> &[SkillsProperty];
	/// Take <https://schema.org/skills> from [`Self`] as owned vector.
	fn take_skills(&mut self) -> Vec<SkillsProperty>;
	/// Get <https://schema.org/specialCommitments> from [`Self`] as borrowed slice.
	fn get_special_commitments(&self) -> &[SpecialCommitmentsProperty];
	/// Take <https://schema.org/specialCommitments> from [`Self`] as owned vector.
	fn take_special_commitments(&mut self) -> Vec<SpecialCommitmentsProperty>;
	/// Get <https://schema.org/title> from [`Self`] as borrowed slice.
	fn get_title(&self) -> &[TitleProperty];
	/// Take <https://schema.org/title> from [`Self`] as owned vector.
	fn take_title(&mut self) -> Vec<TitleProperty>;
	/// Get <https://schema.org/totalJobOpenings> from [`Self`] as borrowed slice.
	fn get_total_job_openings(&self) -> &[TotalJobOpeningsProperty];
	/// Take <https://schema.org/totalJobOpenings> from [`Self`] as owned vector.
	fn take_total_job_openings(&mut self) -> Vec<TotalJobOpeningsProperty>;
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn get_valid_through(&self) -> &[ValidThroughProperty];
	/// Take <https://schema.org/validThrough> from [`Self`] as owned vector.
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty>;
	/// Get <https://schema.org/workHours> from [`Self`] as borrowed slice.
	fn get_work_hours(&self) -> &[WorkHoursProperty];
	/// Take <https://schema.org/workHours> from [`Self`] as owned vector.
	fn take_work_hours(&mut self) -> Vec<WorkHoursProperty>;
}
impl JobPostingTrait for JobPosting {
	fn get_applicant_location_requirements(&self) -> &[ApplicantLocationRequirementsProperty] {
		self.r#applicant_location_requirements.as_slice()
	}
	fn take_applicant_location_requirements(
		&mut self,
	) -> Vec<ApplicantLocationRequirementsProperty> {
		std::mem::take(&mut self.r#applicant_location_requirements)
	}
	fn get_application_contact(&self) -> &[ApplicationContactProperty] {
		self.r#application_contact.as_slice()
	}
	fn take_application_contact(&mut self) -> Vec<ApplicationContactProperty> {
		std::mem::take(&mut self.r#application_contact)
	}
	fn get_base_salary(&self) -> &[BaseSalaryProperty] {
		self.r#base_salary.as_slice()
	}
	fn take_base_salary(&mut self) -> Vec<BaseSalaryProperty> {
		std::mem::take(&mut self.r#base_salary)
	}
	fn get_benefits(&self) -> &[BenefitsProperty] {
		self.r#benefits.as_slice()
	}
	fn take_benefits(&mut self) -> Vec<BenefitsProperty> {
		std::mem::take(&mut self.r#benefits)
	}
	fn get_date_posted(&self) -> &[DatePostedProperty] {
		self.r#date_posted.as_slice()
	}
	fn take_date_posted(&mut self) -> Vec<DatePostedProperty> {
		std::mem::take(&mut self.r#date_posted)
	}
	fn get_direct_apply(&self) -> &[DirectApplyProperty] {
		self.r#direct_apply.as_slice()
	}
	fn take_direct_apply(&mut self) -> Vec<DirectApplyProperty> {
		std::mem::take(&mut self.r#direct_apply)
	}
	fn get_education_requirements(&self) -> &[EducationRequirementsProperty] {
		self.r#education_requirements.as_slice()
	}
	fn take_education_requirements(&mut self) -> Vec<EducationRequirementsProperty> {
		std::mem::take(&mut self.r#education_requirements)
	}
	fn get_eligibility_to_work_requirement(&self) -> &[EligibilityToWorkRequirementProperty] {
		self.r#eligibility_to_work_requirement.as_slice()
	}
	fn take_eligibility_to_work_requirement(
		&mut self,
	) -> Vec<EligibilityToWorkRequirementProperty> {
		std::mem::take(&mut self.r#eligibility_to_work_requirement)
	}
	fn get_employer_overview(&self) -> &[EmployerOverviewProperty] {
		self.r#employer_overview.as_slice()
	}
	fn take_employer_overview(&mut self) -> Vec<EmployerOverviewProperty> {
		std::mem::take(&mut self.r#employer_overview)
	}
	fn get_employment_type(&self) -> &[EmploymentTypeProperty] {
		self.r#employment_type.as_slice()
	}
	fn take_employment_type(&mut self) -> Vec<EmploymentTypeProperty> {
		std::mem::take(&mut self.r#employment_type)
	}
	fn get_employment_unit(&self) -> &[EmploymentUnitProperty] {
		self.r#employment_unit.as_slice()
	}
	fn take_employment_unit(&mut self) -> Vec<EmploymentUnitProperty> {
		std::mem::take(&mut self.r#employment_unit)
	}
	fn get_estimated_salary(&self) -> &[EstimatedSalaryProperty] {
		self.r#estimated_salary.as_slice()
	}
	fn take_estimated_salary(&mut self) -> Vec<EstimatedSalaryProperty> {
		std::mem::take(&mut self.r#estimated_salary)
	}
	fn get_experience_in_place_of_education(&self) -> &[ExperienceInPlaceOfEducationProperty] {
		self.r#experience_in_place_of_education.as_slice()
	}
	fn take_experience_in_place_of_education(
		&mut self,
	) -> Vec<ExperienceInPlaceOfEducationProperty> {
		std::mem::take(&mut self.r#experience_in_place_of_education)
	}
	fn get_experience_requirements(&self) -> &[ExperienceRequirementsProperty] {
		self.r#experience_requirements.as_slice()
	}
	fn take_experience_requirements(&mut self) -> Vec<ExperienceRequirementsProperty> {
		std::mem::take(&mut self.r#experience_requirements)
	}
	fn get_hiring_organization(&self) -> &[HiringOrganizationProperty] {
		self.r#hiring_organization.as_slice()
	}
	fn take_hiring_organization(&mut self) -> Vec<HiringOrganizationProperty> {
		std::mem::take(&mut self.r#hiring_organization)
	}
	fn get_incentive_compensation(&self) -> &[IncentiveCompensationProperty] {
		self.r#incentive_compensation.as_slice()
	}
	fn take_incentive_compensation(&mut self) -> Vec<IncentiveCompensationProperty> {
		std::mem::take(&mut self.r#incentive_compensation)
	}
	fn get_incentives(&self) -> &[IncentivesProperty] {
		self.r#incentives.as_slice()
	}
	fn take_incentives(&mut self) -> Vec<IncentivesProperty> {
		std::mem::take(&mut self.r#incentives)
	}
	fn get_industry(&self) -> &[IndustryProperty] {
		self.r#industry.as_slice()
	}
	fn take_industry(&mut self) -> Vec<IndustryProperty> {
		std::mem::take(&mut self.r#industry)
	}
	fn get_job_benefits(&self) -> &[JobBenefitsProperty] {
		self.r#job_benefits.as_slice()
	}
	fn take_job_benefits(&mut self) -> Vec<JobBenefitsProperty> {
		std::mem::take(&mut self.r#job_benefits)
	}
	fn get_job_duration(&self) -> &[JobDurationProperty] {
		self.r#job_duration.as_slice()
	}
	fn take_job_duration(&mut self) -> Vec<JobDurationProperty> {
		std::mem::take(&mut self.r#job_duration)
	}
	fn get_job_immediate_start(&self) -> &[JobImmediateStartProperty] {
		self.r#job_immediate_start.as_slice()
	}
	fn take_job_immediate_start(&mut self) -> Vec<JobImmediateStartProperty> {
		std::mem::take(&mut self.r#job_immediate_start)
	}
	fn get_job_location(&self) -> &[JobLocationProperty] {
		self.r#job_location.as_slice()
	}
	fn take_job_location(&mut self) -> Vec<JobLocationProperty> {
		std::mem::take(&mut self.r#job_location)
	}
	fn get_job_location_type(&self) -> &[JobLocationTypeProperty] {
		self.r#job_location_type.as_slice()
	}
	fn take_job_location_type(&mut self) -> Vec<JobLocationTypeProperty> {
		std::mem::take(&mut self.r#job_location_type)
	}
	fn get_job_start_date(&self) -> &[JobStartDateProperty] {
		self.r#job_start_date.as_slice()
	}
	fn take_job_start_date(&mut self) -> Vec<JobStartDateProperty> {
		std::mem::take(&mut self.r#job_start_date)
	}
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty> {
		std::mem::take(&mut self.r#occupational_category)
	}
	fn get_physical_requirement(&self) -> &[PhysicalRequirementProperty] {
		self.r#physical_requirement.as_slice()
	}
	fn take_physical_requirement(&mut self) -> Vec<PhysicalRequirementProperty> {
		std::mem::take(&mut self.r#physical_requirement)
	}
	fn get_qualifications(&self) -> &[QualificationsProperty] {
		self.r#qualifications.as_slice()
	}
	fn take_qualifications(&mut self) -> Vec<QualificationsProperty> {
		std::mem::take(&mut self.r#qualifications)
	}
	fn get_relevant_occupation(&self) -> &[RelevantOccupationProperty] {
		self.r#relevant_occupation.as_slice()
	}
	fn take_relevant_occupation(&mut self) -> Vec<RelevantOccupationProperty> {
		std::mem::take(&mut self.r#relevant_occupation)
	}
	fn get_responsibilities(&self) -> &[ResponsibilitiesProperty] {
		self.r#responsibilities.as_slice()
	}
	fn take_responsibilities(&mut self) -> Vec<ResponsibilitiesProperty> {
		std::mem::take(&mut self.r#responsibilities)
	}
	fn get_salary_currency(&self) -> &[SalaryCurrencyProperty] {
		self.r#salary_currency.as_slice()
	}
	fn take_salary_currency(&mut self) -> Vec<SalaryCurrencyProperty> {
		std::mem::take(&mut self.r#salary_currency)
	}
	fn get_security_clearance_requirement(&self) -> &[SecurityClearanceRequirementProperty] {
		self.r#security_clearance_requirement.as_slice()
	}
	fn take_security_clearance_requirement(&mut self) -> Vec<SecurityClearanceRequirementProperty> {
		std::mem::take(&mut self.r#security_clearance_requirement)
	}
	fn get_sensory_requirement(&self) -> &[SensoryRequirementProperty] {
		self.r#sensory_requirement.as_slice()
	}
	fn take_sensory_requirement(&mut self) -> Vec<SensoryRequirementProperty> {
		std::mem::take(&mut self.r#sensory_requirement)
	}
	fn get_skills(&self) -> &[SkillsProperty] {
		self.r#skills.as_slice()
	}
	fn take_skills(&mut self) -> Vec<SkillsProperty> {
		std::mem::take(&mut self.r#skills)
	}
	fn get_special_commitments(&self) -> &[SpecialCommitmentsProperty] {
		self.r#special_commitments.as_slice()
	}
	fn take_special_commitments(&mut self) -> Vec<SpecialCommitmentsProperty> {
		std::mem::take(&mut self.r#special_commitments)
	}
	fn get_title(&self) -> &[TitleProperty] {
		self.r#title.as_slice()
	}
	fn take_title(&mut self) -> Vec<TitleProperty> {
		std::mem::take(&mut self.r#title)
	}
	fn get_total_job_openings(&self) -> &[TotalJobOpeningsProperty] {
		self.r#total_job_openings.as_slice()
	}
	fn take_total_job_openings(&mut self) -> Vec<TotalJobOpeningsProperty> {
		std::mem::take(&mut self.r#total_job_openings)
	}
	fn get_valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty> {
		std::mem::take(&mut self.r#valid_through)
	}
	fn get_work_hours(&self) -> &[WorkHoursProperty] {
		self.r#work_hours.as_slice()
	}
	fn take_work_hours(&mut self) -> Vec<WorkHoursProperty> {
		std::mem::take(&mut self.r#work_hours)
	}
}
impl ThingTrait for JobPosting {
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
