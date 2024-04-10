use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Action {
    #[serde(rename = "dscp")]
    Dscp,
    #[serde(rename = "mark")]
    Mark,
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Bandwidth {
    #[serde(rename = "burst")]
    Burst,
    #[serde(rename = "rate")]
    Rate,
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Direction {
    #[serde(rename = "from-lport")]
    FromLport,
    #[serde(rename = "to-lport")]
    ToLport,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "QoSProxy", into = "QoSProxy")]
pub struct QoS {
    action: std::collections::BTreeMap<String, i64>,
    bandwidth: std::collections::BTreeMap<String, i64>,
    direction: Direction,
    external_ids: std::collections::BTreeMap<String, String>,
    #[serde(rename = "match")]
    matching: String,
    priority: i64,
}
impl Entity for QoS {
    fn table_name() -> &'static str {
        "QoS"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct QoSProxy {
    action: ovsdb::protocol::Map<String, i64>,
    bandwidth: ovsdb::protocol::Map<String, i64>,
    direction: Direction,
    external_ids: ovsdb::protocol::Map<String, String>,
    #[serde(rename = "match")]
    matching: String,
    priority: i64,
}
impl From<QoSProxy> for QoS {
    fn from(other: QoSProxy) -> Self {
        Self {
            action: other.action.into(),
            bandwidth: other.bandwidth.into(),
            direction: other.direction.into(),
            external_ids: other.external_ids.into(),
            matching: other.matching,
            priority: other.priority,
        }
    }
}
impl From<QoS> for QoSProxy {
    fn from(other: QoS) -> Self {
        Self {
            action: other.action.into(),
            bandwidth: other.bandwidth.into(),
            direction: other.direction.into(),
            external_ids: other.external_ids.into(),
            matching: other.matching,
            priority: other.priority,
        }
    }
}
