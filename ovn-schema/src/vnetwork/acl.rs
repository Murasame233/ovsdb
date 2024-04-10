use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Action {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "allow-related")]
    AllowRelated,
    #[serde(rename = "allow-stateless")]
    AllowStateless,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "reject")]
    Reject,
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Direction {
    #[serde(rename = "from-lport")]
    FromLport,
    #[serde(rename = "to-lport")]
    ToLport,
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Severity {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "AclProxy", into = "AclProxy")]
pub struct Acl {
    action: Action,
    direction: Direction,
    external_ids: std::collections::BTreeMap<String, String>,
    label: i64,
    log: bool,
    #[serde(rename = "match")]
    matching: String,
    meter: Option<String>,
    name: Option<String>,
    options: std::collections::BTreeMap<String, String>,
    priority: i64,
    severity: Option<Severity>,
    tier: i64,
}
impl Entity for Acl {
    fn table_name() -> &'static str {
        "ACL"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AclProxy {
    action: Action,
    direction: Direction,
    external_ids: ovsdb::protocol::Map<String, String>,
    label: i64,
    log: bool,
    #[serde(rename = "match")]
    matching: String,
    meter: ovsdb::protocol::Optional<String>,
    name: ovsdb::protocol::Optional<String>,
    options: ovsdb::protocol::Map<String, String>,
    priority: i64,
    severity: ovsdb::protocol::Optional<Severity>,
    tier: i64,
}
impl From<AclProxy> for Acl {
    fn from(other: AclProxy) -> Self {
        Self {
            action: other.action.into(),
            direction: other.direction.into(),
            external_ids: other.external_ids.into(),
            label: other.label,
            log: other.log,
            matching: other.matching,
            meter: other.meter.into(),
            name: other.name.into(),
            options: other.options.into(),
            priority: other.priority,
            severity: other.severity.into(),
            tier: other.tier,
        }
    }
}
impl From<Acl> for AclProxy {
    fn from(other: Acl) -> Self {
        Self {
            action: other.action.into(),
            direction: other.direction.into(),
            external_ids: other.external_ids.into(),
            label: other.label,
            log: other.log,
            matching: other.matching,
            meter: other.meter.into(),
            name: other.name.into(),
            options: other.options.into(),
            priority: other.priority,
            severity: other.severity.into(),
            tier: other.tier,
        }
    }
}
