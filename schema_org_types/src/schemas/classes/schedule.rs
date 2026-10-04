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
	fn get_by_day(&self) -> &[ByDayProperty];
	/// Take <https://schema.org/byDay> from [`Self`] as owned vector.
	fn take_by_day(&mut self) -> Vec<ByDayProperty>;
	/// Get <https://schema.org/byMonth> from [`Self`] as borrowed slice.
	fn get_by_month(&self) -> &[ByMonthProperty];
	/// Take <https://schema.org/byMonth> from [`Self`] as owned vector.
	fn take_by_month(&mut self) -> Vec<ByMonthProperty>;
	/// Get <https://schema.org/byMonthDay> from [`Self`] as borrowed slice.
	fn get_by_month_day(&self) -> &[ByMonthDayProperty];
	/// Take <https://schema.org/byMonthDay> from [`Self`] as owned vector.
	fn take_by_month_day(&mut self) -> Vec<ByMonthDayProperty>;
	/// Get <https://schema.org/byMonthWeek> from [`Self`] as borrowed slice.
	fn get_by_month_week(&self) -> &[ByMonthWeekProperty];
	/// Take <https://schema.org/byMonthWeek> from [`Self`] as owned vector.
	fn take_by_month_week(&mut self) -> Vec<ByMonthWeekProperty>;
	/// Get <https://schema.org/duration> from [`Self`] as borrowed slice.
	fn get_duration(&self) -> &[DurationProperty];
	/// Take <https://schema.org/duration> from [`Self`] as owned vector.
	fn take_duration(&mut self) -> Vec<DurationProperty>;
	/// Get <https://schema.org/endDate> from [`Self`] as borrowed slice.
	fn get_end_date(&self) -> &[EndDateProperty];
	/// Take <https://schema.org/endDate> from [`Self`] as owned vector.
	fn take_end_date(&mut self) -> Vec<EndDateProperty>;
	/// Get <https://schema.org/endTime> from [`Self`] as borrowed slice.
	fn get_end_time(&self) -> &[EndTimeProperty];
	/// Take <https://schema.org/endTime> from [`Self`] as owned vector.
	fn take_end_time(&mut self) -> Vec<EndTimeProperty>;
	/// Get <https://schema.org/exceptDate> from [`Self`] as borrowed slice.
	fn get_except_date(&self) -> &[ExceptDateProperty];
	/// Take <https://schema.org/exceptDate> from [`Self`] as owned vector.
	fn take_except_date(&mut self) -> Vec<ExceptDateProperty>;
	/// Get <https://schema.org/repeatCount> from [`Self`] as borrowed slice.
	fn get_repeat_count(&self) -> &[RepeatCountProperty];
	/// Take <https://schema.org/repeatCount> from [`Self`] as owned vector.
	fn take_repeat_count(&mut self) -> Vec<RepeatCountProperty>;
	/// Get <https://schema.org/repeatFrequency> from [`Self`] as borrowed slice.
	fn get_repeat_frequency(&self) -> &[RepeatFrequencyProperty];
	/// Take <https://schema.org/repeatFrequency> from [`Self`] as owned vector.
	fn take_repeat_frequency(&mut self) -> Vec<RepeatFrequencyProperty>;
	/// Get <https://schema.org/scheduleTimezone> from [`Self`] as borrowed slice.
	fn get_schedule_timezone(&self) -> &[ScheduleTimezoneProperty];
	/// Take <https://schema.org/scheduleTimezone> from [`Self`] as owned vector.
	fn take_schedule_timezone(&mut self) -> Vec<ScheduleTimezoneProperty>;
	/// Get <https://schema.org/startDate> from [`Self`] as borrowed slice.
	fn get_start_date(&self) -> &[StartDateProperty];
	/// Take <https://schema.org/startDate> from [`Self`] as owned vector.
	fn take_start_date(&mut self) -> Vec<StartDateProperty>;
	/// Get <https://schema.org/startTime> from [`Self`] as borrowed slice.
	fn get_start_time(&self) -> &[StartTimeProperty];
	/// Take <https://schema.org/startTime> from [`Self`] as owned vector.
	fn take_start_time(&mut self) -> Vec<StartTimeProperty>;
}
impl ScheduleTrait for Schedule {
	fn get_by_day(&self) -> &[ByDayProperty] {
		self.r#by_day.as_slice()
	}
	fn take_by_day(&mut self) -> Vec<ByDayProperty> {
		std::mem::take(&mut self.r#by_day)
	}
	fn get_by_month(&self) -> &[ByMonthProperty] {
		self.r#by_month.as_slice()
	}
	fn take_by_month(&mut self) -> Vec<ByMonthProperty> {
		std::mem::take(&mut self.r#by_month)
	}
	fn get_by_month_day(&self) -> &[ByMonthDayProperty] {
		self.r#by_month_day.as_slice()
	}
	fn take_by_month_day(&mut self) -> Vec<ByMonthDayProperty> {
		std::mem::take(&mut self.r#by_month_day)
	}
	fn get_by_month_week(&self) -> &[ByMonthWeekProperty] {
		self.r#by_month_week.as_slice()
	}
	fn take_by_month_week(&mut self) -> Vec<ByMonthWeekProperty> {
		std::mem::take(&mut self.r#by_month_week)
	}
	fn get_duration(&self) -> &[DurationProperty] {
		self.r#duration.as_slice()
	}
	fn take_duration(&mut self) -> Vec<DurationProperty> {
		std::mem::take(&mut self.r#duration)
	}
	fn get_end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn take_end_date(&mut self) -> Vec<EndDateProperty> {
		std::mem::take(&mut self.r#end_date)
	}
	fn get_end_time(&self) -> &[EndTimeProperty] {
		self.r#end_time.as_slice()
	}
	fn take_end_time(&mut self) -> Vec<EndTimeProperty> {
		std::mem::take(&mut self.r#end_time)
	}
	fn get_except_date(&self) -> &[ExceptDateProperty] {
		self.r#except_date.as_slice()
	}
	fn take_except_date(&mut self) -> Vec<ExceptDateProperty> {
		std::mem::take(&mut self.r#except_date)
	}
	fn get_repeat_count(&self) -> &[RepeatCountProperty] {
		self.r#repeat_count.as_slice()
	}
	fn take_repeat_count(&mut self) -> Vec<RepeatCountProperty> {
		std::mem::take(&mut self.r#repeat_count)
	}
	fn get_repeat_frequency(&self) -> &[RepeatFrequencyProperty] {
		self.r#repeat_frequency.as_slice()
	}
	fn take_repeat_frequency(&mut self) -> Vec<RepeatFrequencyProperty> {
		std::mem::take(&mut self.r#repeat_frequency)
	}
	fn get_schedule_timezone(&self) -> &[ScheduleTimezoneProperty] {
		self.r#schedule_timezone.as_slice()
	}
	fn take_schedule_timezone(&mut self) -> Vec<ScheduleTimezoneProperty> {
		std::mem::take(&mut self.r#schedule_timezone)
	}
	fn get_start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn take_start_date(&mut self) -> Vec<StartDateProperty> {
		std::mem::take(&mut self.r#start_date)
	}
	fn get_start_time(&self) -> &[StartTimeProperty] {
		self.r#start_time.as_slice()
	}
	fn take_start_time(&mut self) -> Vec<StartTimeProperty> {
		std::mem::take(&mut self.r#start_time)
	}
}
impl ThingTrait for Schedule {
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
