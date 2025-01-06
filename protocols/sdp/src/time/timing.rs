use abstractions::parsing::payload_parser::PayloadParser;
use chrono::{
    DateTime, Duration, TimeDelta, TimeZone, Utc
};
use timespan::{DateTimeSpan, Spanable};

use crate::{
    TRIM, 
    TRIM_REF
};

const SUBTRAHEND: i64 = 2208988800;


/// The first and second sub-fields give the start and stop times,
/// respectively, for the session.  These values are the decimal
/// representation of Network Time Protocol (NTP) time values in seconds
/// since 1900.  To convert these values to UNIX time, subtract
/// decimal 2208988800.

///t=<start-time> <stop-time>
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

impl PayloadParser<Timing> for Timing {
    fn parse(data: &[u8]) -> Result<Timing, std::io::Error> {
        if let Some((start, stop)) = data.separate_trimmed(TRIM, TRIM_REF) {
            let start_time = std::str::from_utf8(start)?.parse::<i64>()?;
            let stop_time = std::str::from_utf8(stop)?.parse::<i64>()?;

            Ok((Timing { 
                start_time: Utc.timestamp_opt(start_time - SUBTRAHEND, 0), 
                stop_time: (stop_time != 0).then(|| Utc.timestamp_opt(stop_time - SUBTRAHEND, 0))  
            }))
        }
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Incorrect format {}", std::str::from_utf8(data)?)))
    }
}

impl Timing {
    pub fn start_time(&self) -> &DateTime {
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