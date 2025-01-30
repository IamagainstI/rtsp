use abstractions::{
    extensions::{
        array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt
    }, 
    parsing::{payload_parser::PayloadParser, ParsingError}
};
use chrono::{
    DateTime, Duration, TimeZone, Utc
};

use crate::{
    TRIM, TRIM_REF
};

const SUBTRAHEND: i64 = 2208988800;

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
    fn parse(data: &[u8]) -> Result<Timing, ParsingError> {
        if let Some((start, stop)) = data.separate_trimmed(TRIM, TRIM_REF) {
            let start_time = start.utf8_to_number::<i64>()?;
            let stop_time = stop.utf8_to_number::<i64>()?;

            return Ok(Timing {
                start_time: Utc.timestamp_opt(start_time - SUBTRAHEND, 0).unwrap(), 
                stop_time: (stop_time != 0).then_some(Utc.timestamp_opt(stop_time - SUBTRAHEND, 0).unwrap())
            });
        }
        Err(ParsingError::from_bytes(data))
    }
}

impl Timing {
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