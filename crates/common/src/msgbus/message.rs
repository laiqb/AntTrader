use std::fmt::Display;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use ustr::Ustr;

use super::switchboard::CLOSE_TOPIC;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.common")
)]
pub struct BusMessage{
    pub topic: Ustr,
    pub payload: Bytes,
}

impl BusMessage {
    pub fn new(topic: Ustr, payload: Bytes) -> Self{
        debug_assert!(!topic.is_empty());
        Self { topic: topic, payload: payload }
    }

    pub fn with_str_topic<T: AsRef<str>>(topic: T, payload: Bytes) -> Self{
        Self::new(Ustr::from(topic.as_ref()), payload)
    }

    pub fn new_close() -> Self{
        Self::with_str_topic(CLOSE_TOPIC, Bytes::new())
    }
}

impl Display for BusMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "[{}]{}",
            self.topic,
            String::from_utf8_lossy(&self.payload),
        )
    }
}

#[cfg(test)]
mod tests{
    use std::clone;

    use bytes::Bytes;
    use rstest::*;
    use super::*;

    #[rstest]
    #[case("test/topic", "paload data")]
    #[case("events/trading", "Another payload")]
    fn test_with_str_topic_str(#[case] topic: &str, #[case] payload_str: &str) {
        let payload = Bytes::from(payload_str.to_string());

        let message = BusMessage::with_str_topic(topic, payload.clone());

        assert_eq!(message.topic.as_str(), topic);
        assert_eq!(message.payload, payload);
    }
}