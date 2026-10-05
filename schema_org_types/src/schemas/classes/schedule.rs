use super::*;
/// <https://schema.org/Schedule>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Schedule {
	/// <https://schema.org/byDay>
	#[cfg_attr(feature = "serde", serde(rename = "byDay"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#by_day: Vec<ByDayProperty>,
	/// <https://schema.org/byMonth>
	#[cfg_attr(feature = "serde", serde(rename = "byMonth"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#by_month: Vec<ByMonthProperty>,
	/// <https://schema.org/byMonthDay>
	#[cfg_attr(feature = "serde", serde(rename = "byMonthDay"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#by_month_day: Vec<ByMonthDayProperty>,
	/// <https://schema.org/byMonthWeek>
	#[cfg_attr(feature = "serde", serde(rename = "byMonthWeek"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#by_month_week: Vec<ByMonthWeekProperty>,
	/// <https://schema.org/duration>
	#[cfg_attr(feature = "serde", serde(rename = "duration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#duration: Vec<DurationProperty>,
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
	/// <https://schema.org/exceptDate>
	#[cfg_attr(feature = "serde", serde(rename = "exceptDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#except_date: Vec<ExceptDateProperty>,
	/// <https://schema.org/repeatCount>
	#[cfg_attr(feature = "serde", serde(rename = "repeatCount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#repeat_count: Vec<RepeatCountProperty>,
	/// <https://schema.org/repeatFrequency>
	#[cfg_attr(feature = "serde", serde(rename = "repeatFrequency"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#repeat_frequency: Vec<RepeatFrequencyProperty>,
	/// <https://schema.org/scheduleTimezone>
	#[cfg_attr(feature = "serde", serde(rename = "scheduleTimezone"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#schedule_timezone: Vec<ScheduleTimezoneProperty>,
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
/// This trait is for properties from <https://schema.org/Schedule>.
pub trait ScheduleTrait {
	/// Get <https://schema.org/byDay> from [`Self`] as borrowed slice.
	fn r#by_day(&self) -> &[ByDayProperty];
	/// Get <https://schema.org/byMonth> from [`Self`] as borrowed slice.
	fn r#by_month(&self) -> &[ByMonthProperty];
	/// Get <https://schema.org/byMonthDay> from [`Self`] as borrowed slice.
	fn r#by_month_day(&self) -> &[ByMonthDayProperty];
	/// Get <https://schema.org/byMonthWeek> from [`Self`] as borrowed slice.
	fn r#by_month_week(&self) -> &[ByMonthWeekProperty];
	/// Get <https://schema.org/duration> from [`Self`] as borrowed slice.
	fn r#duration(&self) -> &[DurationProperty];
	/// Get <https://schema.org/endDate> from [`Self`] as borrowed slice.
	fn r#end_date(&self) -> &[EndDateProperty];
	/// Get <https://schema.org/endTime> from [`Self`] as borrowed slice.
	fn r#end_time(&self) -> &[EndTimeProperty];
	/// Get <https://schema.org/exceptDate> from [`Self`] as borrowed slice.
	fn r#except_date(&self) -> &[ExceptDateProperty];
	/// Get <https://schema.org/repeatCount> from [`Self`] as borrowed slice.
	fn r#repeat_count(&self) -> &[RepeatCountProperty];
	/// Get <https://schema.org/repeatFrequency> from [`Self`] as borrowed slice.
	fn r#repeat_frequency(&self) -> &[RepeatFrequencyProperty];
	/// Get <https://schema.org/scheduleTimezone> from [`Self`] as borrowed slice.
	fn r#schedule_timezone(&self) -> &[ScheduleTimezoneProperty];
	/// Get <https://schema.org/startDate> from [`Self`] as borrowed slice.
	fn r#start_date(&self) -> &[StartDateProperty];
	/// Get <https://schema.org/startTime> from [`Self`] as borrowed slice.
	fn r#start_time(&self) -> &[StartTimeProperty];
}
impl ScheduleTrait for Schedule {
	fn r#by_day(&self) -> &[ByDayProperty] {
		self.r#by_day.as_slice()
	}
	fn r#by_month(&self) -> &[ByMonthProperty] {
		self.r#by_month.as_slice()
	}
	fn r#by_month_day(&self) -> &[ByMonthDayProperty] {
		self.r#by_month_day.as_slice()
	}
	fn r#by_month_week(&self) -> &[ByMonthWeekProperty] {
		self.r#by_month_week.as_slice()
	}
	fn r#duration(&self) -> &[DurationProperty] {
		self.r#duration.as_slice()
	}
	fn r#end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn r#end_time(&self) -> &[EndTimeProperty] {
		self.r#end_time.as_slice()
	}
	fn r#except_date(&self) -> &[ExceptDateProperty] {
		self.r#except_date.as_slice()
	}
	fn r#repeat_count(&self) -> &[RepeatCountProperty] {
		self.r#repeat_count.as_slice()
	}
	fn r#repeat_frequency(&self) -> &[RepeatFrequencyProperty] {
		self.r#repeat_frequency.as_slice()
	}
	fn r#schedule_timezone(&self) -> &[ScheduleTimezoneProperty] {
		self.r#schedule_timezone.as_slice()
	}
	fn r#start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn r#start_time(&self) -> &[StartTimeProperty] {
		self.r#start_time.as_slice()
	}
}
impl ThingTrait for Schedule {
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
