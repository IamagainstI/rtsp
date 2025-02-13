use abstractions::{
    extensions::{
        array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt
    }, 
    parsing::{parsing_error::ParsingError, payload_parser::PayloadParser, WHITESPACE}
};
use chrono::{
    DateTime, Duration, TimeZone, Utc
};

///The first and second sub-fields give the start and stop times,
///respectively, for the session.  These values are the decimal
///representation of Network Time Protocol (NTP) time values in seconds
///since 1900.  To convert these values to UNIX time, subtract
///decimal 2208988800.
const SUBTRAHEND: i64 = 2208988800;


/// Represents the timing information for a session.
/// 
/// The `Timing` struct is used to specify the start and stop times for a session, as defined in RFC 4566.
/// 
/// # Fields
/// 
/// * `start_time` - The start time of the session, represented as a `DateTime<Utc>`.
/// * `stop_time` - An optional stop time for the session, represented as an `Option<DateTime<Utc>>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timing {
    start_time: DateTime<Utc>,
    stop_time: Option<DateTime<Utc>>,
}

impl Default for Timing {
    fn default() -> Self {
        Timing {
            start_time: Utc::now(),
            stop_time: None,
        }
    }
}

impl PayloadParser for Timing {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> {
        if let Some((start, stop)) = data.separate_trimmed(WHITESPACE, WHITESPACE) {
            let start_time = start.utf8_to_number::<i64>()?;
            let stop_time = stop.utf8_to_number::<i64>()?;

            return Ok(Timing {
                start_time: Utc
                    .timestamp_opt(start_time - SUBTRAHEND, 0)
                    .single()
                    .ok_or_else(|| ParsingError::from_bytes(data))?, 
                stop_time: (stop_time != 0)
                    .then_some(
                        Utc
                        .timestamp_opt(stop_time - SUBTRAHEND, 0)
                        .single()
                        .ok_or_else(|| ParsingError::from_bytes(data)
                    )?)
            });
        }
        else if let Ok(start_time) = data.utf8_to_number::<i64>() {
            return Ok(Timing {
                start_time: Utc
                    .timestamp_opt(start_time - SUBTRAHEND, 0)
                    .single()
                    .ok_or_else(|| ParsingError::from_bytes(data))?,
                stop_time: None,
            });
        }
        Err(ParsingError::from_bytes(data))
    }
}

impl Timing {
    pub fn new(start_time: DateTime<Utc>, stop_time: Option<DateTime<Utc>>) -> Self {
        Self { start_time, stop_time }
    }
    
    pub fn start_time(&self) -> &DateTime<Utc> {
        &self.start_time
    }

    pub fn stop_time(&self) -> Option<&DateTime<Utc>> {
        self.stop_time.as_ref()
    }

    ///If the stop time is not set, the duration is None
    pub fn get_duration(&self) -> Option<Duration> {
        self.stop_time.map(|stop_time| stop_time.signed_duration_since(self.start_time))
    }
}