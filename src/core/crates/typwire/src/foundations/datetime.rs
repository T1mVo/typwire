use serde::{Deserialize, Serialize};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "datetime";

/// Represents a date, a time, or a combination of both.
///
/// Every field is optional, matching Typst's `datetime` constructor.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/datetime/).
///
/// # Examples
///
/// ```
/// use typwire::{DateTime, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn year(date: DateTime) -> Result<Option<i64>, String> {
///     Ok(date.year)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
#[serde(from = "DateTimeTypst", into = "DateTimeTypst")]
pub struct DateTime {
    /// The year, if the date component is present.
    pub year: Option<i64>,
    /// The month (1-12), if the date component is present.
    pub month: Option<i64>,
    /// The day of the month (1-31), if the date component is present.
    pub day: Option<i64>,
    /// The hour (0-23), if the time component is present.
    pub hour: Option<i64>,
    /// The minute (0-59), if the time component is present.
    pub minute: Option<i64>,
    /// The second (0-59), if the time component is present.
    pub second: Option<i64>,
}

impl DateTime {
    /// Creates a new builder for constructing a `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().year(2025).build();
    /// assert_eq!(dt.year, Some(2025));
    /// ```
    pub fn builder() -> DateTimeBuilder {
        DateTimeBuilder {
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
        }
    }

    /// Converts this date and time into a Unix timestamp: the number of
    /// seconds since 1970-01-01 00:00:00, interpreting the value as UTC.
    ///
    /// The year, month and day must all be present. A missing hour, minute or
    /// second counts as zero. Returns `None` if the date is incomplete, a
    /// component is out of range (such as February 30th or hour 24), or the
    /// result does not fit in an `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let epoch = DateTime::builder().year(1970).month(1).day(1).build();
    /// assert_eq!(epoch.unix_timestamp(), Some(0));
    ///
    /// let dt = DateTime::builder()
    ///     .year(2000)
    ///     .month(1)
    ///     .day(1)
    ///     .hour(12)
    ///     .second(30)
    ///     .build();
    /// assert_eq!(dt.unix_timestamp(), Some(946_728_030));
    ///
    /// let before = DateTime::builder()
    ///     .year(1969)
    ///     .month(12)
    ///     .day(31)
    ///     .hour(23)
    ///     .minute(59)
    ///     .second(59)
    ///     .build();
    /// assert_eq!(before.unix_timestamp(), Some(-1));
    ///
    /// let time_only = DateTime::builder().hour(12).build();
    /// assert_eq!(time_only.unix_timestamp(), None);
    /// ```
    pub fn unix_timestamp(&self) -> Option<i64> {
        let (year, month, day) = (self.year?, self.month?, self.day?);
        let (hour, minute, second) = (
            self.hour.unwrap_or(0),
            self.minute.unwrap_or(0),
            self.second.unwrap_or(0),
        );

        if !(1..=12).contains(&month)
            || !(1..=days_in_month(year, month)).contains(&day)
            || !(0..24).contains(&hour)
            || !(0..60).contains(&minute)
            || !(0..60).contains(&second)
        {
            return None;
        }

        // Days since the epoch, via the proleptic Gregorian calendar
        // algorithm from <https://howardhinnant.github.io/date_algorithms.html>.
        let shifted_year = if month <= 2 {
            year.checked_sub(1)?
        } else {
            year
        };
        let era = shifted_year.div_euclid(400);
        let year_of_era = shifted_year.rem_euclid(400);
        let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        let days = era
            .checked_mul(146_097)?
            .checked_add(day_of_era)?
            .checked_sub(719_468)?;

        // Widened so the day count times 86,400 cannot overflow before the
        // time of day brings the total back into range.
        let seconds = i128::from(days) * 86_400 + i128::from(hour * 3_600 + minute * 60 + second);

        i64::try_from(seconds).ok()
    }

    /// Creates a `DateTime` from a Unix timestamp: the number of seconds since
    /// 1970-01-01 00:00:00, interpreted as UTC.
    ///
    /// Every field of the result is set. This is the inverse of
    /// [`DateTime::unix_timestamp`].
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let epoch = DateTime::from_unix_timestamp(0);
    /// assert_eq!(
    ///     epoch,
    ///     DateTime::builder()
    ///         .year(1970)
    ///         .month(1)
    ///         .day(1)
    ///         .hour(0)
    ///         .minute(0)
    ///         .second(0)
    ///         .build()
    /// );
    ///
    /// let before = DateTime::from_unix_timestamp(-1);
    /// assert_eq!((before.year, before.month, before.day), (Some(1969), Some(12), Some(31)));
    /// assert_eq!((before.hour, before.minute, before.second), (Some(23), Some(59), Some(59)));
    ///
    /// let dt = DateTime::from_unix_timestamp(946_728_030);
    /// assert_eq!(dt.unix_timestamp(), Some(946_728_030));
    /// ```
    pub const fn from_unix_timestamp(timestamp: i64) -> Self {
        let days = timestamp.div_euclid(86_400);
        let seconds_of_day = timestamp.rem_euclid(86_400);

        // Civil date from days since the epoch, via the proleptic Gregorian
        // calendar algorithm from
        // <https://howardhinnant.github.io/date_algorithms.html>.
        let shifted_days = days + 719_468;
        let era = shifted_days.div_euclid(146_097);
        let day_of_era = shifted_days.rem_euclid(146_097);
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_index = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_index + 2) / 5 + 1;
        let month = if month_index < 10 {
            month_index + 3
        } else {
            month_index - 9
        };
        let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 };

        Self {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            hour: Some(seconds_of_day / 3_600),
            minute: Some(seconds_of_day % 3_600 / 60),
            second: Some(seconds_of_day % 60),
        }
    }
}

/// The number of days in `month` (1-12) of `year`.
const fn days_in_month(year: i64, month: i64) -> i64 {
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// A builder for [`DateTime`], created with [`DateTime::builder`].
pub struct DateTimeBuilder {
    year: Option<i64>,
    month: Option<i64>,
    day: Option<i64>,
    hour: Option<i64>,
    minute: Option<i64>,
    second: Option<i64>,
}

impl DateTimeBuilder {
    /// Sets the year component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().year(2025).build();
    /// assert_eq!(dt.year, Some(2025));
    /// ```
    pub const fn year(mut self, year: i64) -> Self {
        self.year = Some(year);

        self
    }

    /// Sets the month component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().month(12).build();
    /// assert_eq!(dt.month, Some(12));
    /// ```
    pub const fn month(mut self, month: i64) -> Self {
        self.month = Some(month);

        self
    }

    /// Sets the day component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().day(3).build();
    /// assert_eq!(dt.day, Some(3));
    /// ```
    pub const fn day(mut self, day: i64) -> Self {
        self.day = Some(day);

        self
    }

    /// Sets the hour component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().hour(14).build();
    /// assert_eq!(dt.hour, Some(14));
    /// ```
    pub const fn hour(mut self, hour: i64) -> Self {
        self.hour = Some(hour);

        self
    }

    /// Sets the minute component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().minute(30).build();
    /// assert_eq!(dt.minute, Some(30));
    /// ```
    pub const fn minute(mut self, minute: i64) -> Self {
        self.minute = Some(minute);

        self
    }

    /// Sets the second component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder().second(45).build();
    /// assert_eq!(dt.second, Some(45));
    /// ```
    pub const fn second(mut self, second: i64) -> Self {
        self.second = Some(second);

        self
    }

    /// Builds the `DateTime` instance from the configured components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::DateTime;
    ///
    /// let dt = DateTime::builder()
    ///     .year(2025)
    ///     .month(12)
    ///     .day(3)
    ///     .hour(14)
    ///     .minute(30)
    ///     .second(45)
    ///     .build();
    /// assert_eq!(dt.year, Some(2025));
    /// assert_eq!(dt.hour, Some(14));
    /// ```
    pub const fn build(self) -> DateTime {
        DateTime {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DateTimeTypst {
    typwire_type: Tag<DateTime>,
    year: Option<i64>,
    month: Option<i64>,
    day: Option<i64>,
    hour: Option<i64>,
    minute: Option<i64>,
    second: Option<i64>,
}

impl Tagged for DateTime {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize into DateTimeTypst
impl From<DateTime> for DateTimeTypst {
    fn from(value: DateTime) -> Self {
        Self {
            typwire_type: Tag::new(),
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
        }
    }
}

// Deserialize from DateTimeTypst
impl From<DateTimeTypst> for DateTime {
    fn from(value: DateTimeTypst) -> Self {
        Self {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: i64) -> DateTime {
        DateTime::builder()
            .year(year)
            .month(month)
            .day(day)
            .hour(hour)
            .minute(minute)
            .second(second)
            .build()
    }

    #[test]
    fn known_timestamps() {
        for (timestamp, expected) in [
            (0, full(1970, 1, 1, 0, 0, 0)),
            (-1, full(1969, 12, 31, 23, 59, 59)),
            (86_399, full(1970, 1, 1, 23, 59, 59)),
            (951_782_400, full(2000, 2, 29, 0, 0, 0)),
            (1_709_210_096, full(2024, 2, 29, 12, 34, 56)),
            (-62_135_596_800, full(1, 1, 1, 0, 0, 0)),
        ] {
            assert_eq!(DateTime::from_unix_timestamp(timestamp), expected);
            assert_eq!(expected.unix_timestamp(), Some(timestamp));
        }
    }

    #[test]
    fn roundtrips_across_a_wide_range() {
        let mut timestamp = -10_000_000_000_i64;
        while timestamp < 10_000_000_000 {
            let dt = DateTime::from_unix_timestamp(timestamp);
            assert_eq!(dt.unix_timestamp(), Some(timestamp), "{dt:?}");
            timestamp += 86_399;
        }
    }

    #[test]
    fn roundtrips_at_i64_extremes() {
        for timestamp in [i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX] {
            let dt = DateTime::from_unix_timestamp(timestamp);
            assert_eq!(dt.unix_timestamp(), Some(timestamp), "{dt:?}");
        }
    }

    #[test]
    fn leap_days() {
        // 1900 is not a leap year, 2000 is.
        assert_eq!(full(1900, 2, 29, 0, 0, 0).unix_timestamp(), None);
        assert!(full(2000, 2, 29, 0, 0, 0).unix_timestamp().is_some());
        assert_eq!(full(2023, 2, 29, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(
            full(2024, 3, 1, 0, 0, 0).unix_timestamp().unwrap()
                - full(2024, 2, 28, 0, 0, 0).unix_timestamp().unwrap(),
            2 * 86_400
        );
    }

    #[test]
    fn missing_date_fields_yield_none() {
        let b = DateTime::builder;
        assert_eq!(b().build().unix_timestamp(), None);
        assert_eq!(b().year(2020).build().unix_timestamp(), None);
        assert_eq!(b().year(2020).month(1).build().unix_timestamp(), None);
        assert_eq!(b().month(1).day(1).build().unix_timestamp(), None);
        assert_eq!(
            b().hour(1).minute(2).second(3).build().unix_timestamp(),
            None
        );
    }

    #[test]
    fn missing_time_fields_default_to_zero() {
        let date = DateTime::builder().year(1970).month(1).day(2).build();
        assert_eq!(date.unix_timestamp(), Some(86_400));

        let partial = DateTime::builder()
            .year(1970)
            .month(1)
            .day(1)
            .hour(1)
            .build();
        assert_eq!(partial.unix_timestamp(), Some(3_600));
    }

    #[test]
    fn out_of_range_fields_yield_none() {
        assert_eq!(full(2020, 0, 1, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(2020, 13, 1, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(2020, 1, 0, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(2020, 4, 31, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(2020, 1, 1, 24, 0, 0).unix_timestamp(), None);
        assert_eq!(full(2020, 1, 1, 0, 60, 0).unix_timestamp(), None);
        assert_eq!(full(2020, 1, 1, 0, 0, 60).unix_timestamp(), None);
        assert_eq!(full(2020, 1, 1, -1, 0, 0).unix_timestamp(), None);
    }

    #[test]
    fn unrepresentable_years_yield_none() {
        assert_eq!(full(i64::MAX, 6, 1, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(i64::MIN, 6, 1, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(i64::MIN, 1, 1, 0, 0, 0).unix_timestamp(), None);
        assert_eq!(full(300_000_000_000, 1, 1, 0, 0, 0).unix_timestamp(), None);
    }
}
