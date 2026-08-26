use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "duration";
const SECONDS_IN_MINUTE: f64 = 60.0;
const MINUTES_IN_HOUR: f64 = 60.0;
const HOURS_IN_DAY: f64 = 24.0;
const DAYS_IN_WEEK: f64 = 7.0;

/// A structure representing a duration of time in seconds.
///
/// # Examples
///
/// ```
/// use typwire::Duration;
///
/// let d = Duration::new(3600.0);
/// assert_eq!(d.seconds(), 3600.0);
/// assert_eq!(d.hours(), 1.0);
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(try_from = "DurationCbor", into = "DurationCbor")]
pub struct Duration {
    seconds: f64,
}

impl Duration {
    /// Creates a new `Duration` instance from the given seconds.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The duration in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::new(120.0);
    /// assert_eq!(d.seconds(), 120.0);
    /// ```
    pub const fn new(seconds: f64) -> Self {
        Self { seconds }
    }

    /// Creates a new builder for constructing a `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder().seconds(30.0).minutes(1.0).build();
    /// assert_eq!(d.seconds(), 90.0);
    /// ```
    pub const fn builder() -> DurationBuilder {
        DurationBuilder {
            seconds: 0.0,
            minutes: 0.0,
            hours: 0.0,
            days: 0.0,
            weeks: 0.0,
        }
    }

    /// Returns the total duration in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::new(120.0);
    /// assert_eq!(d.seconds(), 120.0);
    /// ```
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// Returns the total duration in minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::new(120.0);
    /// assert_eq!(d.minutes(), 2.0);
    /// ```
    pub const fn minutes(&self) -> f64 {
        self.seconds() / SECONDS_IN_MINUTE
    }

    /// Returns the total duration in hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::new(3600.0);
    /// assert_eq!(d.hours(), 1.0);
    /// ```
    pub const fn hours(&self) -> f64 {
        self.minutes() / MINUTES_IN_HOUR
    }

    /// Returns the total duration in days.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::new(86400.0);
    /// assert_eq!(d.days(), 1.0);
    /// ```
    pub const fn days(&self) -> f64 {
        self.hours() / HOURS_IN_DAY
    }

    /// Returns the total duration in weeks.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::new(604800.0);
    /// assert_eq!(d.weeks(), 1.0);
    /// ```
    pub const fn weeks(&self) -> f64 {
        self.days() / DAYS_IN_WEEK
    }
}

pub struct DurationBuilder {
    seconds: f64,
    minutes: f64,
    hours: f64,
    days: f64,
    weeks: f64,
}

impl DurationBuilder {
    /// Sets the seconds component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder().seconds(30.0).build();
    /// assert_eq!(d.seconds(), 30.0);
    /// ```
    pub const fn seconds(mut self, seconds: f64) -> Self {
        self.seconds = seconds;

        self
    }

    /// Sets the minutes component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder().minutes(2.0).build();
    /// assert_eq!(d.minutes(), 2.0);
    /// ```
    pub const fn minutes(mut self, minutes: f64) -> Self {
        self.minutes = minutes;

        self
    }

    /// Sets the hours component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder().hours(1.0).build();
    /// assert_eq!(d.hours(), 1.0);
    /// ```
    pub const fn hours(mut self, hours: f64) -> Self {
        self.hours = hours;

        self
    }

    /// Sets the days component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder().days(1.0).build();
    /// assert_eq!(d.days(), 1.0);
    /// ```
    pub const fn days(mut self, days: f64) -> Self {
        self.days = days;

        self
    }

    /// Sets the weeks component.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder().weeks(1.0).build();
    /// assert_eq!(d.weeks(), 1.0);
    /// ```
    pub const fn weeks(mut self, weeks: f64) -> Self {
        self.weeks = weeks;

        self
    }

    /// Builds the `Duration` instance from the configured components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Duration;
    ///
    /// let d = Duration::builder()
    ///     .hours(1.0)
    ///     .minutes(30.0)
    ///     .seconds(45.0)
    ///     .build();
    /// assert_eq!(d.seconds(), 5445.0);
    /// ```
    pub const fn build(self) -> Duration {
        Duration::new(
            self.seconds
                + self.minutes * SECONDS_IN_MINUTE
                + self.hours * MINUTES_IN_HOUR * SECONDS_IN_MINUTE
                + self.days * HOURS_IN_DAY * MINUTES_IN_HOUR * SECONDS_IN_MINUTE
                + self.weeks * DAYS_IN_WEEK * HOURS_IN_DAY * MINUTES_IN_HOUR * SECONDS_IN_MINUTE,
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DurationCbor {
    typwire_type: String,
    seconds: f64,
}

// Serialize into DurationCbor
impl From<Duration> for DurationCbor {
    fn from(value: Duration) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            seconds: value.seconds,
        }
    }
}

// Deserialize from DurationCbor
impl TryFrom<DurationCbor> for Duration {
    type Error = String;

    fn try_from(value: DurationCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Duration: {}",
                value.typwire_type
            ));
        }

        Ok(Self {
            seconds: value.seconds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let duration = Duration::builder()
            .seconds(4.0)
            .minutes(2.0)
            .hours(1.0)
            .build();

        assert_eq!(Duration { seconds: 3724.0 }, duration)
    }
}
