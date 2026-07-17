pub mod engine_event;
pub mod front_api;
pub mod wire;

#[path = "../../../channel/engine-event/turn_changed/mod.rs"]
pub mod turn_changed;

#[path = "../../../channel/engine-event/player_gold_changed/mod.rs"]
pub mod player_gold_changed;

#[path = "../../../channel/front-api/end_turn/mod.rs"]
pub mod end_turn;

#[path = "../../../channel/front-api/add_gold/mod.rs"]
pub mod add_gold;

#[path = "../../../channel/front-api/spawn_unit/mod.rs"]
pub mod spawn_unit;

#[path = "../../../channel/front-api/create_city/mod.rs"]
pub mod create_city;

#[path = "../../../channel/front-api/request_snapshot/mod.rs"]
pub mod request_snapshot;

mod register;

pub use engine_event::EngineEventOutbox;
pub use front_api::{ApiContext, DispatchOutcome, FrontApiError, FrontApiRegistry};
pub use wire::WireMessage;
