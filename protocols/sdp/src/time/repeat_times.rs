use abstractions::parsing::payload_parser::PayloadParser;
use chrono::Duration;

use crate::{TRIM, TRIM_REF};

pub struct RepeatTimes {
    repeat_interval: Duration,
    active_duration: Duration,
    offsets: Vec<Duration>,
}

impl PayloadParser<RepeatTimes> for RepeatTimes {
    fn parse(data: &[u8]) -> Result<RepeatTimes, std::io::Error> {
        if let Some((first, second)) = data.separate_trimmed(TRIM, TRIM_REF) {
            let repeat_interval = parse_duration(first)?;
            let active_duration = parse_duration(second)?;
            let offsets = data.split(|&b| b == b' ')
                .skip(2)
                .map(|s| parse_duration(s))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(RepeatTimes::new(repeat_interval, active_duration, offsets))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data"))
            
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid input format");
        }

        let repeat_interval = parse_duration(parts[0])?;
        let active_duration = parse_duration(parts[1])?;
        let offsets = parts[2..]
            .iter()
            .map(|&s| parse_duration(s))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(RepeatTimes::new(repeat_interval, active_duration, offsets))
    }
}

/// Represents the repeat times for an SDP (Session Description Protocol) timing field.
///
/// # Fields
/// - `repeat_interval`: The interval between repetitions.
/// - `active_duration`: The duration for which the session is active.
/// - `offsets`: A vector of offsets specifying the start times of the repetitions.
///
/// # Methods
/// - `new(repeat_interval: Duration, active_duration: Duration, offsets: Vec<Duration>) -> Self`
///   - Creates a new `RepeatTimes` instance with the specified repeat interval, active duration, and offsets.
/// - `repeat_interval(&self) -> Duration`
///   - Returns the repeat interval.
/// - `active_duration(&self) -> Duration`
///   - Returns the active duration.
/// - `offsets(&self) -> &[Duration]`
///   - Returns a slice of the offsets.
/// - `parse(input: &str) -> Result<Self, &'static str>`
///   - Parses a string input to create a `RepeatTimes` instance. The input should be a whitespace-separated string
///     where the first part is the repeat interval, the second part is the active duration, and the remaining parts
///     are the offsets. Returns an error if the input format is invalid.
impl RepeatTimes {
    pub fn new(repeat_interval: Duration, active_duration: Duration, offsets: Vec<Duration>) -> Self {
        RepeatTimes {
            repeat_interval,
            active_duration,
            offsets,
        }
    }

    pub fn repeat_interval(&self) -> Duration {
        self.repeat_interval
    }

    pub fn active_duration(&self) -> Duration {
        self.active_duration
    }

    pub fn offsets(&self) -> &[Duration] {
        &self.offsets
    }

    pub fn parse(input: &str) -> Result<Self, &'static str> {
        
    }
}

fn parse_duration(input: &str) -> Result<Duration, &'static str> {
    let (value, unit) = input.split_at(input.len() - 1);
    let value: i64 = value.parse().map_err(|_| "Invalid number")?;
    match unit {
        "d" => Ok(Duration::days(value)),
        "h" => Ok(Duration::hours(value)),
        "m" => Ok(Duration::minutes(value)),
        "s" => Ok(Duration::seconds(value)),
        _ => Err("Invalid unit"),
    }
}