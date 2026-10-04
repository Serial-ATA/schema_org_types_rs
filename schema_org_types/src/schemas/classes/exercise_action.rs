use super::*;
/// <https://schema.org/ExerciseAction>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ExerciseAction {
	/// <https://schema.org/course>
	#[deprecated = "This schema is superseded by <https://schema.org/exerciseCourse>."]
	#[cfg_attr(feature = "serde", serde(rename = "course"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#course: Vec<CourseProperty>,
	/// <https://schema.org/diet>
	#[cfg_attr(feature = "serde", serde(rename = "diet"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#diet: Vec<DietProperty>,
	/// <https://schema.org/distance>
	#[cfg_attr(feature = "serde", serde(rename = "distance"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#distance: Vec<DistanceProperty>,
	/// <https://schema.org/exerciseCourse>
	#[cfg_attr(feature = "serde", serde(rename = "exerciseCourse"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#exercise_course: Vec<ExerciseCourseProperty>,
	/// <https://schema.org/exercisePlan>
	#[cfg_attr(feature = "serde", serde(rename = "exercisePlan"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#exercise_plan: Vec<ExercisePlanProperty>,
	/// <https://schema.org/exerciseRelatedDiet>
	#[cfg_attr(feature = "serde", serde(rename = "exerciseRelatedDiet"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#exercise_related_diet: Vec<ExerciseRelatedDietProperty>,
	/// <https://schema.org/exerciseType>
	#[cfg_attr(feature = "serde", serde(rename = "exerciseType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#exercise_type: Vec<ExerciseTypeProperty>,
	/// <https://schema.org/fromLocation>
	#[cfg_attr(feature = "serde", serde(rename = "fromLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#from_location: Vec<FromLocationProperty>,
	/// <https://schema.org/opponent>
	#[cfg_attr(feature = "serde", serde(rename = "opponent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#opponent: Vec<OpponentProperty>,
	/// <https://schema.org/sportsActivityLocation>
	#[cfg_attr(feature = "serde", serde(rename = "sportsActivityLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sports_activity_location: Vec<SportsActivityLocationProperty>,
	/// <https://schema.org/sportsEvent>
	#[cfg_attr(feature = "serde", serde(rename = "sportsEvent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sports_event: Vec<SportsEventProperty>,
	/// <https://schema.org/sportsTeam>
	#[cfg_attr(feature = "serde", serde(rename = "sportsTeam"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sports_team: Vec<SportsTeamProperty>,
	/// <https://schema.org/toLocation>
	#[cfg_attr(feature = "serde", serde(rename = "toLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#to_location: Vec<ToLocationProperty>,
	/// <https://schema.org/actionProcess>
	#[cfg_attr(feature = "serde", serde(rename = "actionProcess"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#action_process: Vec<ActionProcessProperty>,
	/// <https://schema.org/actionStatus>
	#[cfg_attr(feature = "serde", serde(rename = "actionStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#action_status: Vec<ActionStatusProperty>,
	/// <https://schema.org/agent>
	#[cfg_attr(feature = "serde", serde(rename = "agent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#agent: Vec<AgentProperty>,
	/// <https://schema.org/endTime>
	#[cfg_attr(feature = "serde", serde(rename = "endTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#end_time: Vec<EndTimeProperty>,
	/// <https://schema.org/error>
	#[cfg_attr(feature = "serde", serde(rename = "error"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#error: Vec<ErrorProperty>,
	/// <https://schema.org/instrument>
	#[cfg_attr(feature = "serde", serde(rename = "instrument"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#instrument: Vec<InstrumentProperty>,
	/// <https://schema.org/location>
	#[cfg_attr(feature = "serde", serde(rename = "location"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#location: Vec<LocationProperty>,
	/// <https://schema.org/object>
	#[cfg_attr(feature = "serde", serde(rename = "object"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#object: Vec<ObjectProperty>,
	/// <https://schema.org/participant>
	#[cfg_attr(feature = "serde", serde(rename = "participant"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#participant: Vec<ParticipantProperty>,
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
	/// <https://schema.org/result>
	#[cfg_attr(feature = "serde", serde(rename = "result"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#result: Vec<ResultProperty>,
	/// <https://schema.org/startTime>
	#[cfg_attr(feature = "serde", serde(rename = "startTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#start_time: Vec<StartTimeProperty>,
	/// <https://schema.org/target>
	#[cfg_attr(feature = "serde", serde(rename = "target"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#target: Vec<TargetProperty>,
	/// <https://schema.org/audience>
	#[cfg_attr(feature = "serde", serde(rename = "audience"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#audience: Vec<AudienceProperty>,
	/// <https://schema.org/event>
	#[cfg_attr(feature = "serde", serde(rename = "event"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#event: Vec<EventProperty>,
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
/// This trait is for properties from <https://schema.org/ExerciseAction>.
pub trait ExerciseActionTrait {
	/// Get <https://schema.org/course> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/exerciseCourse>."]
	fn get_course(&self) -> &[CourseProperty];
	/// Take <https://schema.org/course> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/exerciseCourse>."]
	fn take_course(&mut self) -> Vec<CourseProperty>;
	/// Get <https://schema.org/diet> from [`Self`] as borrowed slice.
	fn get_diet(&self) -> &[DietProperty];
	/// Take <https://schema.org/diet> from [`Self`] as owned vector.
	fn take_diet(&mut self) -> Vec<DietProperty>;
	/// Get <https://schema.org/distance> from [`Self`] as borrowed slice.
	fn get_distance(&self) -> &[DistanceProperty];
	/// Take <https://schema.org/distance> from [`Self`] as owned vector.
	fn take_distance(&mut self) -> Vec<DistanceProperty>;
	/// Get <https://schema.org/exerciseCourse> from [`Self`] as borrowed slice.
	fn get_exercise_course(&self) -> &[ExerciseCourseProperty];
	/// Take <https://schema.org/exerciseCourse> from [`Self`] as owned vector.
	fn take_exercise_course(&mut self) -> Vec<ExerciseCourseProperty>;
	/// Get <https://schema.org/exercisePlan> from [`Self`] as borrowed slice.
	fn get_exercise_plan(&self) -> &[ExercisePlanProperty];
	/// Take <https://schema.org/exercisePlan> from [`Self`] as owned vector.
	fn take_exercise_plan(&mut self) -> Vec<ExercisePlanProperty>;
	/// Get <https://schema.org/exerciseRelatedDiet> from [`Self`] as borrowed slice.
	fn get_exercise_related_diet(&self) -> &[ExerciseRelatedDietProperty];
	/// Take <https://schema.org/exerciseRelatedDiet> from [`Self`] as owned vector.
	fn take_exercise_related_diet(&mut self) -> Vec<ExerciseRelatedDietProperty>;
	/// Get <https://schema.org/exerciseType> from [`Self`] as borrowed slice.
	fn get_exercise_type(&self) -> &[ExerciseTypeProperty];
	/// Take <https://schema.org/exerciseType> from [`Self`] as owned vector.
	fn take_exercise_type(&mut self) -> Vec<ExerciseTypeProperty>;
	/// Get <https://schema.org/fromLocation> from [`Self`] as borrowed slice.
	fn get_from_location(&self) -> &[FromLocationProperty];
	/// Take <https://schema.org/fromLocation> from [`Self`] as owned vector.
	fn take_from_location(&mut self) -> Vec<FromLocationProperty>;
	/// Get <https://schema.org/opponent> from [`Self`] as borrowed slice.
	fn get_opponent(&self) -> &[OpponentProperty];
	/// Take <https://schema.org/opponent> from [`Self`] as owned vector.
	fn take_opponent(&mut self) -> Vec<OpponentProperty>;
	/// Get <https://schema.org/sportsActivityLocation> from [`Self`] as borrowed slice.
	fn get_sports_activity_location(&self) -> &[SportsActivityLocationProperty];
	/// Take <https://schema.org/sportsActivityLocation> from [`Self`] as owned vector.
	fn take_sports_activity_location(&mut self) -> Vec<SportsActivityLocationProperty>;
	/// Get <https://schema.org/sportsEvent> from [`Self`] as borrowed slice.
	fn get_sports_event(&self) -> &[SportsEventProperty];
	/// Take <https://schema.org/sportsEvent> from [`Self`] as owned vector.
	fn take_sports_event(&mut self) -> Vec<SportsEventProperty>;
	/// Get <https://schema.org/sportsTeam> from [`Self`] as borrowed slice.
	fn get_sports_team(&self) -> &[SportsTeamProperty];
	/// Take <https://schema.org/sportsTeam> from [`Self`] as owned vector.
	fn take_sports_team(&mut self) -> Vec<SportsTeamProperty>;
	/// Get <https://schema.org/toLocation> from [`Self`] as borrowed slice.
	fn get_to_location(&self) -> &[ToLocationProperty];
	/// Take <https://schema.org/toLocation> from [`Self`] as owned vector.
	fn take_to_location(&mut self) -> Vec<ToLocationProperty>;
}
impl ExerciseActionTrait for ExerciseAction {
	fn get_course(&self) -> &[CourseProperty] {
		self.r#course.as_slice()
	}
	fn take_course(&mut self) -> Vec<CourseProperty> {
		std::mem::take(&mut self.r#course)
	}
	fn get_diet(&self) -> &[DietProperty] {
		self.r#diet.as_slice()
	}
	fn take_diet(&mut self) -> Vec<DietProperty> {
		std::mem::take(&mut self.r#diet)
	}
	fn get_distance(&self) -> &[DistanceProperty] {
		self.r#distance.as_slice()
	}
	fn take_distance(&mut self) -> Vec<DistanceProperty> {
		std::mem::take(&mut self.r#distance)
	}
	fn get_exercise_course(&self) -> &[ExerciseCourseProperty] {
		self.r#exercise_course.as_slice()
	}
	fn take_exercise_course(&mut self) -> Vec<ExerciseCourseProperty> {
		std::mem::take(&mut self.r#exercise_course)
	}
	fn get_exercise_plan(&self) -> &[ExercisePlanProperty] {
		self.r#exercise_plan.as_slice()
	}
	fn take_exercise_plan(&mut self) -> Vec<ExercisePlanProperty> {
		std::mem::take(&mut self.r#exercise_plan)
	}
	fn get_exercise_related_diet(&self) -> &[ExerciseRelatedDietProperty] {
		self.r#exercise_related_diet.as_slice()
	}
	fn take_exercise_related_diet(&mut self) -> Vec<ExerciseRelatedDietProperty> {
		std::mem::take(&mut self.r#exercise_related_diet)
	}
	fn get_exercise_type(&self) -> &[ExerciseTypeProperty] {
		self.r#exercise_type.as_slice()
	}
	fn take_exercise_type(&mut self) -> Vec<ExerciseTypeProperty> {
		std::mem::take(&mut self.r#exercise_type)
	}
	fn get_from_location(&self) -> &[FromLocationProperty] {
		self.r#from_location.as_slice()
	}
	fn take_from_location(&mut self) -> Vec<FromLocationProperty> {
		std::mem::take(&mut self.r#from_location)
	}
	fn get_opponent(&self) -> &[OpponentProperty] {
		self.r#opponent.as_slice()
	}
	fn take_opponent(&mut self) -> Vec<OpponentProperty> {
		std::mem::take(&mut self.r#opponent)
	}
	fn get_sports_activity_location(&self) -> &[SportsActivityLocationProperty] {
		self.r#sports_activity_location.as_slice()
	}
	fn take_sports_activity_location(&mut self) -> Vec<SportsActivityLocationProperty> {
		std::mem::take(&mut self.r#sports_activity_location)
	}
	fn get_sports_event(&self) -> &[SportsEventProperty] {
		self.r#sports_event.as_slice()
	}
	fn take_sports_event(&mut self) -> Vec<SportsEventProperty> {
		std::mem::take(&mut self.r#sports_event)
	}
	fn get_sports_team(&self) -> &[SportsTeamProperty] {
		self.r#sports_team.as_slice()
	}
	fn take_sports_team(&mut self) -> Vec<SportsTeamProperty> {
		std::mem::take(&mut self.r#sports_team)
	}
	fn get_to_location(&self) -> &[ToLocationProperty] {
		self.r#to_location.as_slice()
	}
	fn take_to_location(&mut self) -> Vec<ToLocationProperty> {
		std::mem::take(&mut self.r#to_location)
	}
}
impl ActionTrait for ExerciseAction {
	fn get_action_process(&self) -> &[ActionProcessProperty] {
		self.r#action_process.as_slice()
	}
	fn take_action_process(&mut self) -> Vec<ActionProcessProperty> {
		std::mem::take(&mut self.r#action_process)
	}
	fn get_action_status(&self) -> &[ActionStatusProperty] {
		self.r#action_status.as_slice()
	}
	fn take_action_status(&mut self) -> Vec<ActionStatusProperty> {
		std::mem::take(&mut self.r#action_status)
	}
	fn get_agent(&self) -> &[AgentProperty] {
		self.r#agent.as_slice()
	}
	fn take_agent(&mut self) -> Vec<AgentProperty> {
		std::mem::take(&mut self.r#agent)
	}
	fn get_end_time(&self) -> &[EndTimeProperty] {
		self.r#end_time.as_slice()
	}
	fn take_end_time(&mut self) -> Vec<EndTimeProperty> {
		std::mem::take(&mut self.r#end_time)
	}
	fn get_error(&self) -> &[ErrorProperty] {
		self.r#error.as_slice()
	}
	fn take_error(&mut self) -> Vec<ErrorProperty> {
		std::mem::take(&mut self.r#error)
	}
	fn get_instrument(&self) -> &[InstrumentProperty] {
		self.r#instrument.as_slice()
	}
	fn take_instrument(&mut self) -> Vec<InstrumentProperty> {
		std::mem::take(&mut self.r#instrument)
	}
	fn get_location(&self) -> &[LocationProperty] {
		self.r#location.as_slice()
	}
	fn take_location(&mut self) -> Vec<LocationProperty> {
		std::mem::take(&mut self.r#location)
	}
	fn get_object(&self) -> &[ObjectProperty] {
		self.r#object.as_slice()
	}
	fn take_object(&mut self) -> Vec<ObjectProperty> {
		std::mem::take(&mut self.r#object)
	}
	fn get_participant(&self) -> &[ParticipantProperty] {
		self.r#participant.as_slice()
	}
	fn take_participant(&mut self) -> Vec<ParticipantProperty> {
		std::mem::take(&mut self.r#participant)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_result(&self) -> &[ResultProperty] {
		self.r#result.as_slice()
	}
	fn take_result(&mut self) -> Vec<ResultProperty> {
		std::mem::take(&mut self.r#result)
	}
	fn get_start_time(&self) -> &[StartTimeProperty] {
		self.r#start_time.as_slice()
	}
	fn take_start_time(&mut self) -> Vec<StartTimeProperty> {
		std::mem::take(&mut self.r#start_time)
	}
	fn get_target(&self) -> &[TargetProperty] {
		self.r#target.as_slice()
	}
	fn take_target(&mut self) -> Vec<TargetProperty> {
		std::mem::take(&mut self.r#target)
	}
}
impl PlayActionTrait for ExerciseAction {
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
	}
	fn get_event(&self) -> &[EventProperty] {
		self.r#event.as_slice()
	}
	fn take_event(&mut self) -> Vec<EventProperty> {
		std::mem::take(&mut self.r#event)
	}
}
impl ThingTrait for ExerciseAction {
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
