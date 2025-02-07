use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt},
    parsing::{parsing_error::ParsingError, payload_parser::PayloadParser},
};
use chrono::Duration;

use crate::TRIM;

/// Represents the repeat times field in an SDP message.
///
/// The `RepeatTimes` struct corresponds to the `r=` field in SDP, which specifies
/// repeat times for a session. This field indicates how often and for how long
/// the session is active.
///
/// According to RFC 4566, the `r=` field has the following syntax:
///
/// ```text
/// r=<repeat interval> <active duration> <offsets from start-time>
/// ```
///
/// - `<repeat interval>`: The time between repeats of the session.
/// - `<active duration>`: The duration for which the session is active.
/// - `<offsets from start-time>`: The offsets from the start time at which the session is active.
///
/// Example:
///
/// ```text
/// r=604800 3600 0 90000
/// ```
///
/// This example indicates that the session repeats every week (604800 seconds),
/// is active for one hour (3600 seconds), and has offsets of 0 and 25 hours (90000 seconds).
pub struct RepeatTimes {
    repeat_interval: Duration,
    active_duration: Duration,
    offset1: Duration,
    offset2: Duration,
}

impl PayloadParser for RepeatTimes {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> {
        let (interval, other) = data
            .separate_trimmed(TRIM, TRIM)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let (duration, other) = other
            .separate_trimmed(TRIM, TRIM)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let (offset1, offset2) = other
            .separate_trimmed(TRIM, TRIM)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        Ok(RepeatTimes::new(
            parse_duration(interval)?,
            parse_duration(duration)?,
            parse_duration(offset1)?,
            parse_duration(offset2)?,
        ))
    }
}

impl RepeatTimes {
    /// Creates a new `RepeatTimes` instance.
    ///
    /// # Arguments
    ///
    /// * `repeat_interval` - The time between repeats of the session.
    /// * `active_duration` - The duration for which the session is active.
    /// * `offset1` - The first offset from the start time at which the session is active.
    /// * `offset2` - The second offset from the start time at which the session is active.
    pub fn new(
        repeat_interval: Duration,
        active_duration: Duration,
        offset1: Duration,
        offset2: Duration,
    ) -> Self {
        Self {
            repeat_interval,
            active_duration,
            offset1,
            offset2,
        }
    }

    pub fn repeat_interval(&self) -> Duration {
        self.repeat_interval
    }

    pub fn active_duration(&self) -> Duration {
        self.active_duration
    }

    pub fn offset1(&self) -> Duration {
        self.offset1
    }

    pub fn offset2(&self) -> Duration {
        self.offset2
    }
}

fn parse_duration(data: &[u8]) -> Result<Duration, ParsingError> {
    let Some((unit, value)) = data.split_last() else {
        return Err(ParsingError::from_str("Data is empty."));
    };
    let has_unit_specifier = is_unit_spec(*unit);
    if !has_unit_specifier {
        let value: i64 = data
            .utf8_to_number::<i64>()
            .map_err(|e| ParsingError::Utf8Error(e))?;
        return Ok(Duration::seconds(value));
    }
    let value: i64 = value
        .utf8_to_number::<i64>()
        .map_err(|e| ParsingError::Utf8Error(e))?;
    match unit {
        b'd' => Ok(Duration::days(value)),
        b'h' => Ok(Duration::hours(value)),
        b'm' => Ok(Duration::minutes(value)),
        b's' => Ok(Duration::seconds(value)),
        _ => Err(ParsingError::from_bytes(data)),
    }
}

fn is_unit_spec(byte: u8) -> bool {
    byte == b'd' || byte == b'h' || byte == b'm' || byte == b's'
}
