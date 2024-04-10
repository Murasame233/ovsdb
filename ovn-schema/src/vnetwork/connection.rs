use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "ConnectionProxy", into = "ConnectionProxy")]
pub struct Connection {
    external_ids: std::collections::BTreeMap<String, String>,
    inactivity_probe: Option<i64>,
    is_connected: bool,
    max_backoff: Option<i64>,
    other_config: std::collections::BTreeMap<String, String>,
    status: std::collections::BTreeMap<String, String>,
    target: String,
}
impl Entity for Connection {
    fn table_name() -> &'static str {
        "Connection"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    inactivity_probe: ovsdb::protocol::Optional<i64>,
    is_connected: bool,
    max_backoff: ovsdb::protocol::Optional<i64>,
    other_config: ovsdb::protocol::Map<String, String>,
    status: ovsdb::protocol::Map<String, String>,
    target: String,
}
impl From<ConnectionProxy> for Connection {
    fn from(other: ConnectionProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            inactivity_probe: other.inactivity_probe.into(),
            is_connected: other.is_connected,
            max_backoff: other.max_backoff.into(),
            other_config: other.other_config.into(),
            status: other.status.into(),
            target: other.target,
        }
    }
}
impl From<Connection> for ConnectionProxy {
    fn from(other: Connection) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            inactivity_probe: other.inactivity_probe.into(),
            is_connected: other.is_connected,
            max_backoff: other.max_backoff.into(),
            other_config: other.other_config.into(),
            status: other.status.into(),
            target: other.target,
        }
    }
}
