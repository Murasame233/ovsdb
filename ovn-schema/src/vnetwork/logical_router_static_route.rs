use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Policy {
    #[serde(rename = "dst-ip")]
    DstIp,
    #[serde(rename = "src-ip")]
    SrcIp,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LogicalRouterStaticRouteProxy", into = "LogicalRouterStaticRouteProxy")]
pub struct LogicalRouterStaticRoute {
    bfd: Option<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    ip_prefix: String,
    nexthop: String,
    options: std::collections::BTreeMap<String, String>,
    output_port: Option<String>,
    policy: Option<Policy>,
    route_table: String,
}
impl Entity for LogicalRouterStaticRoute {
    fn table_name() -> &'static str {
        "Logical_Router_Static_Route"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogicalRouterStaticRouteProxy {
    bfd: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    external_ids: ovsdb::protocol::Map<String, String>,
    ip_prefix: String,
    nexthop: String,
    options: ovsdb::protocol::Map<String, String>,
    output_port: ovsdb::protocol::Optional<String>,
    policy: ovsdb::protocol::Optional<Policy>,
    route_table: String,
}
impl From<LogicalRouterStaticRouteProxy> for LogicalRouterStaticRoute {
    fn from(other: LogicalRouterStaticRouteProxy) -> Self {
        Self {
            bfd: other.bfd.into(),
            external_ids: other.external_ids.into(),
            ip_prefix: other.ip_prefix,
            nexthop: other.nexthop,
            options: other.options.into(),
            output_port: other.output_port.into(),
            policy: other.policy.into(),
            route_table: other.route_table,
        }
    }
}
impl From<LogicalRouterStaticRoute> for LogicalRouterStaticRouteProxy {
    fn from(other: LogicalRouterStaticRoute) -> Self {
        Self {
            bfd: other.bfd.into(),
            external_ids: other.external_ids.into(),
            ip_prefix: other.ip_prefix,
            nexthop: other.nexthop,
            options: other.options.into(),
            output_port: other.output_port.into(),
            policy: other.policy.into(),
            route_table: other.route_table,
        }
    }
}
