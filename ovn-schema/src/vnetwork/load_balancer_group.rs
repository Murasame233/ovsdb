use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LoadBalancerGroupProxy", into = "LoadBalancerGroupProxy")]
pub struct LoadBalancerGroup {
    load_balancer: Vec<ovsdb::protocol::Uuid>,
    name: String,
}
impl Entity for LoadBalancerGroup {
    fn table_name() -> &'static str {
        "Load_Balancer_Group"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadBalancerGroupProxy {
    load_balancer: ovsdb::protocol::UuidSet,
    name: String,
}
impl From<LoadBalancerGroupProxy> for LoadBalancerGroup {
    fn from(other: LoadBalancerGroupProxy) -> Self {
        Self {
            load_balancer: other.load_balancer.into(),
            name: other.name,
        }
    }
}
impl From<LoadBalancerGroup> for LoadBalancerGroupProxy {
    fn from(other: LoadBalancerGroup) -> Self {
        Self {
            load_balancer: other.load_balancer.into(),
            name: other.name,
        }
    }
}
