use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CityId(pub Uuid);

impl EntityId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl UnitId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl CityId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self(0)
    }
}
