use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LogicalSwitchProxy", into = "LogicalSwitchProxy")]
pub struct LogicalSwitch {
    acls: Vec<ovsdb::protocol::Uuid>,
    copp: Option<ovsdb::protocol::Uuid>,
    dns_records: Vec<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    forwarding_groups: Vec<ovsdb::protocol::Uuid>,
    load_balancer: Vec<ovsdb::protocol::Uuid>,
    load_balancer_group: Vec<ovsdb::protocol::Uuid>,
    name: String,
    other_config: std::collections::BTreeMap<String, String>,
    ports: Vec<ovsdb::protocol::Uuid>,
    qos_rules: Vec<ovsdb::protocol::Uuid>,
}
impl Entity for LogicalSwitch {
    fn table_name() -> &'static str {
        "Logical_Switch"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogicalSwitchProxy {
    acls: ovsdb::protocol::UuidSet,
    copp: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    dns_records: ovsdb::protocol::UuidSet,
    external_ids: ovsdb::protocol::Map<String, String>,
    forwarding_groups: ovsdb::protocol::UuidSet,
    load_balancer: ovsdb::protocol::UuidSet,
    load_balancer_group: ovsdb::protocol::UuidSet,
    name: String,
    other_config: ovsdb::protocol::Map<String, String>,
    ports: ovsdb::protocol::UuidSet,
    qos_rules: ovsdb::protocol::UuidSet,
}
impl From<LogicalSwitchProxy> for LogicalSwitch {
    fn from(other: LogicalSwitchProxy) -> Self {
        Self {
            acls: other.acls.into(),
            copp: other.copp.into(),
            dns_records: other.dns_records.into(),
            external_ids: other.external_ids.into(),
            forwarding_groups: other.forwarding_groups.into(),
            load_balancer: other.load_balancer.into(),
            load_balancer_group: other.load_balancer_group.into(),
            name: other.name,
            other_config: other.other_config.into(),
            ports: other.ports.into(),
            qos_rules: other.qos_rules.into(),
        }
    }
}
impl From<LogicalSwitch> for LogicalSwitchProxy {
    fn from(other: LogicalSwitch) -> Self {
        Self {
            acls: other.acls.into(),
            copp: other.copp.into(),
            dns_records: other.dns_records.into(),
            external_ids: other.external_ids.into(),
            forwarding_groups: other.forwarding_groups.into(),
            load_balancer: other.load_balancer.into(),
            load_balancer_group: other.load_balancer_group.into(),
            name: other.name,
            other_config: other.other_config.into(),
            ports: other.ports.into(),
            qos_rules: other.qos_rules.into(),
        }
    }
}
