use http::header::{HeaderMap, HeaderValue};
use http::StatusCode;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct MonitoringData<'a> {
    response_time_ms: i64,
    response_code: StatusCode,
    response_body_size_bytes: usize,
    headers: &'a HeaderMap<HeaderValue>,
}

impl<'a> Serialize for MonitoringData<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MonitoringData", 3)?;
        state.serialize_field("response_time", &self.response_time_ms)?;
        state.serialize_field("response_code", &self.response_code.as_u16())?;
        state.serialize_field("body_size", &self.response_body_size_bytes)?;
        state.end()
    }
}

impl<'a> MonitoringData<'a> {
    pub fn new(
        response_time_ms: i64,
        response_code: StatusCode,
        response_body_size_bytes: usize,
        headers: &'a HeaderMap<HeaderValue>,
    ) -> MonitoringData {
        MonitoringData {
            response_time_ms,
            response_code,
            response_body_size_bytes,
            headers,
        }
    }
}
