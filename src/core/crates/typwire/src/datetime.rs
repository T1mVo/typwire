use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "datetime";

/// A struct representing a date and time with optional fields.
///
/// # Examples
///
/// ```
/// use typwire::DateTime;
///
/// let dt = DateTime::builder().year(2025).month(12).day(3).build();
/// assert_eq!(dt.year, Some(2025));
/// assert_eq!(dt.month, Some(12));
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
#[serde(try_from = "DateTimeCbor", into = "DateTimeCbor")]
pub struct DateTime {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
    pub hour: Option<i64>,
    pub minute: Option<i64>,
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
}

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
struct DateTimeCbor {
    typwire_type: String,
    year: Option<i64>,
    month: Option<i64>,
    day: Option<i64>,
    hour: Option<i64>,
    minute: Option<i64>,
    second: Option<i64>,
}

// Serialize into DateTimeCbor
impl From<DateTime> for DateTimeCbor {
    fn from(value: DateTime) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
        }
    }
}

// Deserialize from DateTimeCbor
impl TryFrom<DateTimeCbor> for DateTime {
    type Error = String;

    fn try_from(value: DateTimeCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for DateTime: {}",
                value.typwire_type
            ));
        }

        Ok(Self {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
        })
    }
}
