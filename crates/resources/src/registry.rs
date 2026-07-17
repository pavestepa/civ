use bevy::prelude::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDefinition {
    pub id: String,
    pub display_name: String,
    pub movement: u32,
    pub strength: u32,
}

#[derive(Resource, Debug, Default)]
pub struct GameDataRegistry {
    pub units: IndexMap<String, UnitDefinition>,
}

impl GameDataRegistry {
    pub fn with_defaults() -> Self {
        let mut units = IndexMap::new();
        units.insert(
            "warrior".into(),
            UnitDefinition {
                id: "warrior".into(),
                display_name: "Warrior".into(),
                movement: 2,
                strength: 20,
            },
        );
        Self { units }
    }
}
