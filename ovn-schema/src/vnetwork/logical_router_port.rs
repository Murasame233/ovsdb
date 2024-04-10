use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LogicalRouterPortProxy", into = "LogicalRouterPortProxy")]
pub struct LogicalRouterPort {
    enabled: Option<bool>,
    external_ids: std::collections::BTreeMap<String, String>,
    gateway_chassis: Vec<ovsdb::protocol::Uuid>,
    ha_chassis_group: Option<ovsdb::protocol::Uuid>,
    ipv6_prefix: Vec<String>,
    ipv6_ra_configs: std::collections::BTreeMap<String, String>,
    mac: String,
    name: String,
    networks: Vec<String>,
    options: std::collections::BTreeMap<String, String>,
    peer: Option<String>,
    status: std::collections::BTreeMap<String, String>,
}
impl Entity for LogicalRouterPort {
    fn table_name() -> &'static str {
        "Logical_Router_Port"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogicalRouterPortProxy {
    enabled: ovsdb::protocol::Optional<bool>,
    external_ids: ovsdb::protocol::Map<String, String>,
    gateway_chassis: ovsdb::protocol::UuidSet,
    ha_chassis_group: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    ipv6_prefix: ovsdb::protocol::Set<String>,
    ipv6_ra_configs: ovsdb::protocol::Map<String, String>,
    mac: String,
    name: String,
    networks: ovsdb::protocol::Set<String>,
    options: ovsdb::protocol::Map<String, String>,
    peer: ovsdb::protocol::Optional<String>,
    status: ovsdb::protocol::Map<String, String>,
}
impl From<LogicalRouterPortProxy> for LogicalRouterPort {
    fn from(other: LogicalRouterPortProxy) -> Self {
        Self {
            enabled: other.enabled.into(),
            external_ids: other.external_ids.into(),
            gateway_chassis: other.gateway_chassis.into(),
            ha_chassis_group: other.ha_chassis_group.into(),
            ipv6_prefix: other.ipv6_prefix.into(),
            ipv6_ra_configs: other.ipv6_ra_configs.into(),
            mac: other.mac,
            name: other.name,
            networks: other.networks.into(),
            options: other.options.into(),
            peer: other.peer.into(),
            status: other.status.into(),
        }
    }
}
impl From<LogicalRouterPort> for LogicalRouterPortProxy {
    fn from(other: LogicalRouterPort) -> Self {
        Self {
            enabled: other.enabled.into(),
            external_ids: other.external_ids.into(),
            gateway_chassis: other.gateway_chassis.into(),
            ha_chassis_group: other.ha_chassis_group.into(),
            ipv6_prefix: other.ipv6_prefix.into(),
            ipv6_ra_configs: other.ipv6_ra_configs.into(),
            mac: other.mac,
            name: other.name,
            networks: other.networks.into(),
            options: other.options.into(),
            peer: other.peer.into(),
            status: other.status.into(),
        }
    }
}
