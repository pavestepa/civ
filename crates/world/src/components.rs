use bitflags::bitflags;
use civ_common::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainKind {
    Ocean,
    Coast,
    Plains,
    Grassland,
    Desert,
    Tundra,
    Snow,
    Hills,
    Mountain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Elevation(pub i16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceDeposit {
    None,
    Iron,
    Horses,
    Wheat,
    Stone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct River {
    pub flows: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Road {
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Owner(pub Option<PlayerId>);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Visibility: u8 {
        const UNEXPLORED = 0b0000_0001;
        const EXPLORED   = 0b0000_0010;
        const VISIBLE    = 0b0000_0100;
    }
}
