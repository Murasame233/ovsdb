use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "HaChassisProxy", into = "HaChassisProxy")]
pub struct HaChassis {
    chassis_name: String,
    external_ids: std::collections::BTreeMap<String, String>,
    priority: i64,
}
impl Entity for HaChassis {
    fn table_name() -> &'static str {
        "HA_Chassis"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct HaChassisProxy {
    chassis_name: String,
    external_ids: ovsdb::protocol::Map<String, String>,
    priority: i64,
}
impl From<HaChassisProxy> for HaChassis {
    fn from(other: HaChassisProxy) -> Self {
        Self {
            chassis_name: other.chassis_name,
            external_ids: other.external_ids.into(),
            priority: other.priority,
        }
    }
}
impl From<HaChassis> for HaChassisProxy {
    fn from(other: HaChassis) -> Self {
        Self {
            chassis_name: other.chassis_name,
            external_ids: other.external_ids.into(),
            priority: other.priority,
        }
    }
}
