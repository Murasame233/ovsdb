use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Action {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "reroute")]
    Reroute,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LogicalRouterPolicyProxy", into = "LogicalRouterPolicyProxy")]
pub struct LogicalRouterPolicy {
    action: Action,
    bfd_sessions: Vec<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    #[serde(rename = "match")]
    matching: String,
    nexthop: Option<String>,
    nexthops: Vec<String>,
    options: std::collections::BTreeMap<String, String>,
    priority: i64,
}
impl Entity for LogicalRouterPolicy {
    fn table_name() -> &'static str {
        "Logical_Router_Policy"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogicalRouterPolicyProxy {
    action: Action,
    bfd_sessions: ovsdb::protocol::UuidSet,
    external_ids: ovsdb::protocol::Map<String, String>,
    #[serde(rename = "match")]
    matching: String,
    nexthop: ovsdb::protocol::Optional<String>,
    nexthops: ovsdb::protocol::Set<String>,
    options: ovsdb::protocol::Map<String, String>,
    priority: i64,
}
impl From<LogicalRouterPolicyProxy> for LogicalRouterPolicy {
    fn from(other: LogicalRouterPolicyProxy) -> Self {
        Self {
            action: other.action.into(),
            bfd_sessions: other.bfd_sessions.into(),
            external_ids: other.external_ids.into(),
            matching: other.matching,
            nexthop: other.nexthop.into(),
            nexthops: other.nexthops.into(),
            options: other.options.into(),
            priority: other.priority,
        }
    }
}
impl From<LogicalRouterPolicy> for LogicalRouterPolicyProxy {
    fn from(other: LogicalRouterPolicy) -> Self {
        Self {
            action: other.action.into(),
            bfd_sessions: other.bfd_sessions.into(),
            external_ids: other.external_ids.into(),
            matching: other.matching,
            nexthop: other.nexthop.into(),
            nexthops: other.nexthops.into(),
            options: other.options.into(),
            priority: other.priority,
        }
    }
}
