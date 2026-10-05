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
	fn r#applicant_location_requirements(&self) -> &[ApplicantLocationRequirementsProperty];
	/// Get <https://schema.org/applicationContact> from [`Self`] as borrowed slice.
	fn r#application_contact(&self) -> &[ApplicationContactProperty];
	/// Get <https://schema.org/baseSalary> from [`Self`] as borrowed slice.
	fn r#base_salary(&self) -> &[BaseSalaryProperty];
	/// Get <https://schema.org/benefits> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
	fn r#benefits(&self) -> &[BenefitsProperty];
	/// Get <https://schema.org/datePosted> from [`Self`] as borrowed slice.
	fn r#date_posted(&self) -> &[DatePostedProperty];
	/// Get <https://schema.org/directApply> from [`Self`] as borrowed slice.
	fn r#direct_apply(&self) -> &[DirectApplyProperty];
	/// Get <https://schema.org/educationRequirements> from [`Self`] as borrowed slice.
	fn r#education_requirements(&self) -> &[EducationRequirementsProperty];
	/// Get <https://schema.org/eligibilityToWorkRequirement> from [`Self`] as borrowed slice.
	fn r#eligibility_to_work_requirement(&self) -> &[EligibilityToWorkRequirementProperty];
	/// Get <https://schema.org/employerOverview> from [`Self`] as borrowed slice.
	fn r#employer_overview(&self) -> &[EmployerOverviewProperty];
	/// Get <https://schema.org/employmentType> from [`Self`] as borrowed slice.
	fn r#employment_type(&self) -> &[EmploymentTypeProperty];
	/// Get <https://schema.org/employmentUnit> from [`Self`] as borrowed slice.
	fn r#employment_unit(&self) -> &[EmploymentUnitProperty];
	/// Get <https://schema.org/estimatedSalary> from [`Self`] as borrowed slice.
	fn r#estimated_salary(&self) -> &[EstimatedSalaryProperty];
	/// Get <https://schema.org/experienceInPlaceOfEducation> from [`Self`] as borrowed slice.
	fn r#experience_in_place_of_education(&self) -> &[ExperienceInPlaceOfEducationProperty];
	/// Get <https://schema.org/experienceRequirements> from [`Self`] as borrowed slice.
	fn r#experience_requirements(&self) -> &[ExperienceRequirementsProperty];
	/// Get <https://schema.org/hiringOrganization> from [`Self`] as borrowed slice.
	fn r#hiring_organization(&self) -> &[HiringOrganizationProperty];
	/// Get <https://schema.org/incentiveCompensation> from [`Self`] as borrowed slice.
	fn r#incentive_compensation(&self) -> &[IncentiveCompensationProperty];
	/// Get <https://schema.org/incentives> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
	fn r#incentives(&self) -> &[IncentivesProperty];
	/// Get <https://schema.org/industry> from [`Self`] as borrowed slice.
	fn r#industry(&self) -> &[IndustryProperty];
	/// Get <https://schema.org/jobBenefits> from [`Self`] as borrowed slice.
	fn r#job_benefits(&self) -> &[JobBenefitsProperty];
	/// Get <https://schema.org/jobDuration> from [`Self`] as borrowed slice.
	fn r#job_duration(&self) -> &[JobDurationProperty];
	/// Get <https://schema.org/jobImmediateStart> from [`Self`] as borrowed slice.
	fn r#job_immediate_start(&self) -> &[JobImmediateStartProperty];
	/// Get <https://schema.org/jobLocation> from [`Self`] as borrowed slice.
	fn r#job_location(&self) -> &[JobLocationProperty];
	/// Get <https://schema.org/jobLocationType> from [`Self`] as borrowed slice.
	fn r#job_location_type(&self) -> &[JobLocationTypeProperty];
	/// Get <https://schema.org/jobStartDate> from [`Self`] as borrowed slice.
	fn r#job_start_date(&self) -> &[JobStartDateProperty];
	/// Get <https://schema.org/occupationalCategory> from [`Self`] as borrowed slice.
	fn r#occupational_category(&self) -> &[OccupationalCategoryProperty];
	/// Get <https://schema.org/physicalRequirement> from [`Self`] as borrowed slice.
	fn r#physical_requirement(&self) -> &[PhysicalRequirementProperty];
	/// Get <https://schema.org/qualifications> from [`Self`] as borrowed slice.
	fn r#qualifications(&self) -> &[QualificationsProperty];
	/// Get <https://schema.org/relevantOccupation> from [`Self`] as borrowed slice.
	fn r#relevant_occupation(&self) -> &[RelevantOccupationProperty];
	/// Get <https://schema.org/responsibilities> from [`Self`] as borrowed slice.
	fn r#responsibilities(&self) -> &[ResponsibilitiesProperty];
	/// Get <https://schema.org/salaryCurrency> from [`Self`] as borrowed slice.
	fn r#salary_currency(&self) -> &[SalaryCurrencyProperty];
	/// Get <https://schema.org/securityClearanceRequirement> from [`Self`] as borrowed slice.
	fn r#security_clearance_requirement(&self) -> &[SecurityClearanceRequirementProperty];
	/// Get <https://schema.org/sensoryRequirement> from [`Self`] as borrowed slice.
	fn r#sensory_requirement(&self) -> &[SensoryRequirementProperty];
	/// Get <https://schema.org/skills> from [`Self`] as borrowed slice.
	fn r#skills(&self) -> &[SkillsProperty];
	/// Get <https://schema.org/specialCommitments> from [`Self`] as borrowed slice.
	fn r#special_commitments(&self) -> &[SpecialCommitmentsProperty];
	/// Get <https://schema.org/title> from [`Self`] as borrowed slice.
	fn r#title(&self) -> &[TitleProperty];
	/// Get <https://schema.org/totalJobOpenings> from [`Self`] as borrowed slice.
	fn r#total_job_openings(&self) -> &[TotalJobOpeningsProperty];
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn r#valid_through(&self) -> &[ValidThroughProperty];
	/// Get <https://schema.org/workHours> from [`Self`] as borrowed slice.
	fn r#work_hours(&self) -> &[WorkHoursProperty];
}
impl JobPostingTrait for JobPosting {
	fn r#applicant_location_requirements(&self) -> &[ApplicantLocationRequirementsProperty] {
		self.r#applicant_location_requirements.as_slice()
	}
	fn r#application_contact(&self) -> &[ApplicationContactProperty] {
		self.r#application_contact.as_slice()
	}
	fn r#base_salary(&self) -> &[BaseSalaryProperty] {
		self.r#base_salary.as_slice()
	}
	fn r#benefits(&self) -> &[BenefitsProperty] {
		self.r#benefits.as_slice()
	}
	fn r#date_posted(&self) -> &[DatePostedProperty] {
		self.r#date_posted.as_slice()
	}
	fn r#direct_apply(&self) -> &[DirectApplyProperty] {
		self.r#direct_apply.as_slice()
	}
	fn r#education_requirements(&self) -> &[EducationRequirementsProperty] {
		self.r#education_requirements.as_slice()
	}
	fn r#eligibility_to_work_requirement(&self) -> &[EligibilityToWorkRequirementProperty] {
		self.r#eligibility_to_work_requirement.as_slice()
	}
	fn r#employer_overview(&self) -> &[EmployerOverviewProperty] {
		self.r#employer_overview.as_slice()
	}
	fn r#employment_type(&self) -> &[EmploymentTypeProperty] {
		self.r#employment_type.as_slice()
	}
	fn r#employment_unit(&self) -> &[EmploymentUnitProperty] {
		self.r#employment_unit.as_slice()
	}
	fn r#estimated_salary(&self) -> &[EstimatedSalaryProperty] {
		self.r#estimated_salary.as_slice()
	}
	fn r#experience_in_place_of_education(&self) -> &[ExperienceInPlaceOfEducationProperty] {
		self.r#experience_in_place_of_education.as_slice()
	}
	fn r#experience_requirements(&self) -> &[ExperienceRequirementsProperty] {
		self.r#experience_requirements.as_slice()
	}
	fn r#hiring_organization(&self) -> &[HiringOrganizationProperty] {
		self.r#hiring_organization.as_slice()
	}
	fn r#incentive_compensation(&self) -> &[IncentiveCompensationProperty] {
		self.r#incentive_compensation.as_slice()
	}
	fn r#incentives(&self) -> &[IncentivesProperty] {
		self.r#incentives.as_slice()
	}
	fn r#industry(&self) -> &[IndustryProperty] {
		self.r#industry.as_slice()
	}
	fn r#job_benefits(&self) -> &[JobBenefitsProperty] {
		self.r#job_benefits.as_slice()
	}
	fn r#job_duration(&self) -> &[JobDurationProperty] {
		self.r#job_duration.as_slice()
	}
	fn r#job_immediate_start(&self) -> &[JobImmediateStartProperty] {
		self.r#job_immediate_start.as_slice()
	}
	fn r#job_location(&self) -> &[JobLocationProperty] {
		self.r#job_location.as_slice()
	}
	fn r#job_location_type(&self) -> &[JobLocationTypeProperty] {
		self.r#job_location_type.as_slice()
	}
	fn r#job_start_date(&self) -> &[JobStartDateProperty] {
		self.r#job_start_date.as_slice()
	}
	fn r#occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn r#physical_requirement(&self) -> &[PhysicalRequirementProperty] {
		self.r#physical_requirement.as_slice()
	}
	fn r#qualifications(&self) -> &[QualificationsProperty] {
		self.r#qualifications.as_slice()
	}
	fn r#relevant_occupation(&self) -> &[RelevantOccupationProperty] {
		self.r#relevant_occupation.as_slice()
	}
	fn r#responsibilities(&self) -> &[ResponsibilitiesProperty] {
		self.r#responsibilities.as_slice()
	}
	fn r#salary_currency(&self) -> &[SalaryCurrencyProperty] {
		self.r#salary_currency.as_slice()
	}
	fn r#security_clearance_requirement(&self) -> &[SecurityClearanceRequirementProperty] {
		self.r#security_clearance_requirement.as_slice()
	}
	fn r#sensory_requirement(&self) -> &[SensoryRequirementProperty] {
		self.r#sensory_requirement.as_slice()
	}
	fn r#skills(&self) -> &[SkillsProperty] {
		self.r#skills.as_slice()
	}
	fn r#special_commitments(&self) -> &[SpecialCommitmentsProperty] {
		self.r#special_commitments.as_slice()
	}
	fn r#title(&self) -> &[TitleProperty] {
		self.r#title.as_slice()
	}
	fn r#total_job_openings(&self) -> &[TotalJobOpeningsProperty] {
		self.r#total_job_openings.as_slice()
	}
	fn r#valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn r#work_hours(&self) -> &[WorkHoursProperty] {
		self.r#work_hours.as_slice()
	}
}
impl ThingTrait for JobPosting {
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
