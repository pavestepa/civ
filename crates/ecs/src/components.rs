use bevy::prelude::*;
use civ_common::{CityId, PlayerId, UnitId};
use civ_hex::HexCoordinate;
use civ_world::components::{Elevation, Owner, TerrainKind};

/// Maps a simulation tile onto an ECS entity.
#[derive(Component, Debug, Clone)]
pub struct TerrainTile {
    pub terrain: TerrainKind,
    pub elevation: Elevation,
    pub owner: Owner,
}

#[derive(Component, Debug, Clone)]
pub struct HexPosition(pub HexCoordinate);

#[derive(Component, Debug, Clone)]
pub struct UnitEntity {
    pub id: UnitId,
    pub owner: PlayerId,
    pub name: String,
}

#[derive(Component, Debug, Clone)]
pub struct CityEntity {
    pub id: CityId,
    pub owner: PlayerId,
    pub name: String,
}

/// Links a render mesh entity to its simulation identity.
#[derive(Component, Debug, Clone, Copy)]
pub struct RenderLink(pub Entity);
