use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LogicalRouterProxy", into = "LogicalRouterProxy")]
pub struct LogicalRouter {
    copp: Option<ovsdb::protocol::Uuid>,
    enabled: Option<bool>,
    external_ids: std::collections::BTreeMap<String, String>,
    load_balancer: Vec<ovsdb::protocol::Uuid>,
    load_balancer_group: Vec<ovsdb::protocol::Uuid>,
    name: String,
    nat: Vec<ovsdb::protocol::Uuid>,
    options: std::collections::BTreeMap<String, String>,
    policies: Vec<ovsdb::protocol::Uuid>,
    ports: Vec<ovsdb::protocol::Uuid>,
    static_routes: Vec<ovsdb::protocol::Uuid>,
}
impl Entity for LogicalRouter {
    fn table_name() -> &'static str {
        "Logical_Router"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogicalRouterProxy {
    copp: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    enabled: ovsdb::protocol::Optional<bool>,
    external_ids: ovsdb::protocol::Map<String, String>,
    load_balancer: ovsdb::protocol::UuidSet,
    load_balancer_group: ovsdb::protocol::UuidSet,
    name: String,
    nat: ovsdb::protocol::UuidSet,
    options: ovsdb::protocol::Map<String, String>,
    policies: ovsdb::protocol::UuidSet,
    ports: ovsdb::protocol::UuidSet,
    static_routes: ovsdb::protocol::UuidSet,
}
impl From<LogicalRouterProxy> for LogicalRouter {
    fn from(other: LogicalRouterProxy) -> Self {
        Self {
            copp: other.copp.into(),
            enabled: other.enabled.into(),
            external_ids: other.external_ids.into(),
            load_balancer: other.load_balancer.into(),
            load_balancer_group: other.load_balancer_group.into(),
            name: other.name,
            nat: other.nat.into(),
            options: other.options.into(),
            policies: other.policies.into(),
            ports: other.ports.into(),
            static_routes: other.static_routes.into(),
        }
    }
}
impl From<LogicalRouter> for LogicalRouterProxy {
    fn from(other: LogicalRouter) -> Self {
        Self {
            copp: other.copp.into(),
            enabled: other.enabled.into(),
            external_ids: other.external_ids.into(),
            load_balancer: other.load_balancer.into(),
            load_balancer_group: other.load_balancer_group.into(),
            name: other.name,
            nat: other.nat.into(),
            options: other.options.into(),
            policies: other.policies.into(),
            ports: other.ports.into(),
            static_routes: other.static_routes.into(),
        }
    }
}
