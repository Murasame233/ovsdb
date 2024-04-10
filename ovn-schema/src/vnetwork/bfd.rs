use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    #[serde(rename = "admin_down")]
    AdminDown,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "up")]
    Up,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "BfdProxy", into = "BfdProxy")]
pub struct Bfd {
    detect_mult: Option<i64>,
    dst_ip: String,
    external_ids: std::collections::BTreeMap<String, String>,
    logical_port: String,
    min_rx: Option<i64>,
    min_tx: Option<i64>,
    options: std::collections::BTreeMap<String, String>,
    status: Option<Status>,
}
impl Entity for Bfd {
    fn table_name() -> &'static str {
        "BFD"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BfdProxy {
    detect_mult: ovsdb::protocol::Optional<i64>,
    dst_ip: String,
    external_ids: ovsdb::protocol::Map<String, String>,
    logical_port: String,
    min_rx: ovsdb::protocol::Optional<i64>,
    min_tx: ovsdb::protocol::Optional<i64>,
    options: ovsdb::protocol::Map<String, String>,
    status: ovsdb::protocol::Optional<Status>,
}
impl From<BfdProxy> for Bfd {
    fn from(other: BfdProxy) -> Self {
        Self {
            detect_mult: other.detect_mult.into(),
            dst_ip: other.dst_ip,
            external_ids: other.external_ids.into(),
            logical_port: other.logical_port,
            min_rx: other.min_rx.into(),
            min_tx: other.min_tx.into(),
            options: other.options.into(),
            status: other.status.into(),
        }
    }
}
impl From<Bfd> for BfdProxy {
    fn from(other: Bfd) -> Self {
        Self {
            detect_mult: other.detect_mult.into(),
            dst_ip: other.dst_ip,
            external_ids: other.external_ids.into(),
            logical_port: other.logical_port,
            min_rx: other.min_rx.into(),
            min_tx: other.min_tx.into(),
            options: other.options.into(),
            status: other.status.into(),
        }
    }
}
