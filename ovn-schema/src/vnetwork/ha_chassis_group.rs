use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "HaChassisGroupProxy", into = "HaChassisGroupProxy")]
pub struct HaChassisGroup {
    external_ids: std::collections::BTreeMap<String, String>,
    ha_chassis: Vec<ovsdb::protocol::Uuid>,
    name: String,
}
impl Entity for HaChassisGroup {
    fn table_name() -> &'static str {
        "HA_Chassis_Group"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct HaChassisGroupProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    ha_chassis: ovsdb::protocol::UuidSet,
    name: String,
}
impl From<HaChassisGroupProxy> for HaChassisGroup {
    fn from(other: HaChassisGroupProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            ha_chassis: other.ha_chassis.into(),
            name: other.name,
        }
    }
}
impl From<HaChassisGroup> for HaChassisGroupProxy {
    fn from(other: HaChassisGroup) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            ha_chassis: other.ha_chassis.into(),
            name: other.name,
        }
    }
}
