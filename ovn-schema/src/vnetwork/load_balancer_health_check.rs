use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LoadBalancerHealthCheckProxy", into = "LoadBalancerHealthCheckProxy")]
pub struct LoadBalancerHealthCheck {
    external_ids: std::collections::BTreeMap<String, String>,
    options: std::collections::BTreeMap<String, String>,
    vip: String,
}
impl Entity for LoadBalancerHealthCheck {
    fn table_name() -> &'static str {
        "Load_Balancer_Health_Check"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadBalancerHealthCheckProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    options: ovsdb::protocol::Map<String, String>,
    vip: String,
}
impl From<LoadBalancerHealthCheckProxy> for LoadBalancerHealthCheck {
    fn from(other: LoadBalancerHealthCheckProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            options: other.options.into(),
            vip: other.vip,
        }
    }
}
impl From<LoadBalancerHealthCheck> for LoadBalancerHealthCheckProxy {
    fn from(other: LoadBalancerHealthCheck) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            options: other.options.into(),
            vip: other.vip,
        }
    }
}
