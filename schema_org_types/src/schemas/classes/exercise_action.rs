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
	fn r#course(&self) -> &[CourseProperty];
	/// Get <https://schema.org/diet> from [`Self`] as borrowed slice.
	fn r#diet(&self) -> &[DietProperty];
	/// Get <https://schema.org/distance> from [`Self`] as borrowed slice.
	fn r#distance(&self) -> &[DistanceProperty];
	/// Get <https://schema.org/exerciseCourse> from [`Self`] as borrowed slice.
	fn r#exercise_course(&self) -> &[ExerciseCourseProperty];
	/// Get <https://schema.org/exercisePlan> from [`Self`] as borrowed slice.
	fn r#exercise_plan(&self) -> &[ExercisePlanProperty];
	/// Get <https://schema.org/exerciseRelatedDiet> from [`Self`] as borrowed slice.
	fn r#exercise_related_diet(&self) -> &[ExerciseRelatedDietProperty];
	/// Get <https://schema.org/exerciseType> from [`Self`] as borrowed slice.
	fn r#exercise_type(&self) -> &[ExerciseTypeProperty];
	/// Get <https://schema.org/fromLocation> from [`Self`] as borrowed slice.
	fn r#from_location(&self) -> &[FromLocationProperty];
	/// Get <https://schema.org/opponent> from [`Self`] as borrowed slice.
	fn r#opponent(&self) -> &[OpponentProperty];
	/// Get <https://schema.org/sportsActivityLocation> from [`Self`] as borrowed slice.
	fn r#sports_activity_location(&self) -> &[SportsActivityLocationProperty];
	/// Get <https://schema.org/sportsEvent> from [`Self`] as borrowed slice.
	fn r#sports_event(&self) -> &[SportsEventProperty];
	/// Get <https://schema.org/sportsTeam> from [`Self`] as borrowed slice.
	fn r#sports_team(&self) -> &[SportsTeamProperty];
	/// Get <https://schema.org/toLocation> from [`Self`] as borrowed slice.
	fn r#to_location(&self) -> &[ToLocationProperty];
}
impl ExerciseActionTrait for ExerciseAction {
	fn r#course(&self) -> &[CourseProperty] {
		self.r#course.as_slice()
	}
	fn r#diet(&self) -> &[DietProperty] {
		self.r#diet.as_slice()
	}
	fn r#distance(&self) -> &[DistanceProperty] {
		self.r#distance.as_slice()
	}
	fn r#exercise_course(&self) -> &[ExerciseCourseProperty] {
		self.r#exercise_course.as_slice()
	}
	fn r#exercise_plan(&self) -> &[ExercisePlanProperty] {
		self.r#exercise_plan.as_slice()
	}
	fn r#exercise_related_diet(&self) -> &[ExerciseRelatedDietProperty] {
		self.r#exercise_related_diet.as_slice()
	}
	fn r#exercise_type(&self) -> &[ExerciseTypeProperty] {
		self.r#exercise_type.as_slice()
	}
	fn r#from_location(&self) -> &[FromLocationProperty] {
		self.r#from_location.as_slice()
	}
	fn r#opponent(&self) -> &[OpponentProperty] {
		self.r#opponent.as_slice()
	}
	fn r#sports_activity_location(&self) -> &[SportsActivityLocationProperty] {
		self.r#sports_activity_location.as_slice()
	}
	fn r#sports_event(&self) -> &[SportsEventProperty] {
		self.r#sports_event.as_slice()
	}
	fn r#sports_team(&self) -> &[SportsTeamProperty] {
		self.r#sports_team.as_slice()
	}
	fn r#to_location(&self) -> &[ToLocationProperty] {
		self.r#to_location.as_slice()
	}
}
impl ActionTrait for ExerciseAction {
	fn r#action_process(&self) -> &[ActionProcessProperty] {
		self.r#action_process.as_slice()
	}
	fn r#action_status(&self) -> &[ActionStatusProperty] {
		self.r#action_status.as_slice()
	}
	fn r#agent(&self) -> &[AgentProperty] {
		self.r#agent.as_slice()
	}
	fn r#end_time(&self) -> &[EndTimeProperty] {
		self.r#end_time.as_slice()
	}
	fn r#error(&self) -> &[ErrorProperty] {
		self.r#error.as_slice()
	}
	fn r#instrument(&self) -> &[InstrumentProperty] {
		self.r#instrument.as_slice()
	}
	fn r#location(&self) -> &[LocationProperty] {
		self.r#location.as_slice()
	}
	fn r#object(&self) -> &[ObjectProperty] {
		self.r#object.as_slice()
	}
	fn r#participant(&self) -> &[ParticipantProperty] {
		self.r#participant.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#result(&self) -> &[ResultProperty] {
		self.r#result.as_slice()
	}
	fn r#start_time(&self) -> &[StartTimeProperty] {
		self.r#start_time.as_slice()
	}
	fn r#target(&self) -> &[TargetProperty] {
		self.r#target.as_slice()
	}
}
impl PlayActionTrait for ExerciseAction {
	fn r#audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn r#event(&self) -> &[EventProperty] {
		self.r#event.as_slice()
	}
}
impl ThingTrait for ExerciseAction {
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
