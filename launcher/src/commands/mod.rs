use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "action", content = "data")]
pub enum UiAction {
    EndTurn,
    AddGold {
        player: u32,
        amount: i32,
    },
    SpawnUnit {
        owner: u32,
        q: i32,
        r: i32,
        kind: String,
    },
    CreateCity {
        owner: u32,
        q: i32,
        r: i32,
        name: String,
    },
    RequestSnapshot,
}
