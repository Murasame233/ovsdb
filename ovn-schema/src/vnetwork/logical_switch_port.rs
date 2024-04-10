use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LogicalSwitchPortProxy", into = "LogicalSwitchPortProxy")]
pub struct LogicalSwitchPort {
    addresses: Vec<String>,
    dhcpv4_options: Option<ovsdb::protocol::Uuid>,
    dhcpv6_options: Option<ovsdb::protocol::Uuid>,
    dynamic_addresses: Option<String>,
    enabled: Option<bool>,
    external_ids: std::collections::BTreeMap<String, String>,
    ha_chassis_group: Option<ovsdb::protocol::Uuid>,
    mirror_rules: Vec<ovsdb::protocol::Uuid>,
    name: String,
    options: std::collections::BTreeMap<String, String>,
    parent_name: Option<String>,
    port_security: Vec<String>,
    tag: Option<i64>,
    tag_request: Option<i64>,
    #[serde(rename = "type")]
    kind: String,
    up: Option<bool>,
}
impl Entity for LogicalSwitchPort {
    fn table_name() -> &'static str {
        "Logical_Switch_Port"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogicalSwitchPortProxy {
    addresses: ovsdb::protocol::Set<String>,
    dhcpv4_options: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    dhcpv6_options: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    dynamic_addresses: ovsdb::protocol::Optional<String>,
    enabled: ovsdb::protocol::Optional<bool>,
    external_ids: ovsdb::protocol::Map<String, String>,
    ha_chassis_group: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    mirror_rules: ovsdb::protocol::UuidSet,
    name: String,
    options: ovsdb::protocol::Map<String, String>,
    parent_name: ovsdb::protocol::Optional<String>,
    port_security: ovsdb::protocol::Set<String>,
    tag: ovsdb::protocol::Optional<i64>,
    tag_request: ovsdb::protocol::Optional<i64>,
    #[serde(rename = "type")]
    kind: String,
    up: ovsdb::protocol::Optional<bool>,
}
impl From<LogicalSwitchPortProxy> for LogicalSwitchPort {
    fn from(other: LogicalSwitchPortProxy) -> Self {
        Self {
            addresses: other.addresses.into(),
            dhcpv4_options: other.dhcpv4_options.into(),
            dhcpv6_options: other.dhcpv6_options.into(),
            dynamic_addresses: other.dynamic_addresses.into(),
            enabled: other.enabled.into(),
            external_ids: other.external_ids.into(),
            ha_chassis_group: other.ha_chassis_group.into(),
            mirror_rules: other.mirror_rules.into(),
            name: other.name,
            options: other.options.into(),
            parent_name: other.parent_name.into(),
            port_security: other.port_security.into(),
            tag: other.tag.into(),
            tag_request: other.tag_request.into(),
            kind: other.kind,
            up: other.up.into(),
        }
    }
}
impl From<LogicalSwitchPort> for LogicalSwitchPortProxy {
    fn from(other: LogicalSwitchPort) -> Self {
        Self {
            addresses: other.addresses.into(),
            dhcpv4_options: other.dhcpv4_options.into(),
            dhcpv6_options: other.dhcpv6_options.into(),
            dynamic_addresses: other.dynamic_addresses.into(),
            enabled: other.enabled.into(),
            external_ids: other.external_ids.into(),
            ha_chassis_group: other.ha_chassis_group.into(),
            mirror_rules: other.mirror_rules.into(),
            name: other.name,
            options: other.options.into(),
            parent_name: other.parent_name.into(),
            port_security: other.port_security.into(),
            tag: other.tag.into(),
            tag_request: other.tag_request.into(),
            kind: other.kind,
            up: other.up.into(),
        }
    }
}
