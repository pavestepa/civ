use civ_commands::GameCommand;
use civ_common::PlayerId;
use civ_hex::HexCoordinate;
use civ_scripting::ScriptGameApi;
use rquickjs::{Context, Runtime};
use std::sync::Mutex;
use tracing::info;

/// QuickJS runtime wrapper. Executes scripts and dispatches `GameCommand`s.
pub struct QuickJsRuntime {
    runtime: Mutex<Runtime>,
}

impl QuickJsRuntime {
    pub fn new() -> Result<Self, rquickjs::Error> {
        Ok(Self {
            runtime: Mutex::new(Runtime::new()?),
        })
    }

    pub fn execute(&self, source: &str) -> Result<(), rquickjs::Error> {
        let runtime = match self.runtime.lock() {
            Ok(runtime) => runtime,
            Err(_) => return Ok(()),
        };
        let context = Context::full(&runtime)?;
        context.with(|ctx| ctx.eval::<(), _>(source))?;
        Ok(())
    }

    /// Registers the `game` global namespace. Full class bindings are expanded later.
    pub fn register_game_api(&self, _api: &ScriptGameApi) -> Result<(), rquickjs::Error> {
        let runtime = match self.runtime.lock() {
            Ok(runtime) => runtime,
            Err(_) => return Ok(()),
        };
        let context = Context::full(&runtime)?;
        context.with(|ctx| {
            let globals = ctx.globals();
            globals.set("game", rquickjs::Object::new(ctx.clone())?)?;
            info!("QuickJS game API namespace registered");
            Ok(())
        })
    }
}

/// JavaScript-callable bindings. Infrastructure only — no script logic.
#[derive(Clone)]
pub struct GameBindings {
    api: ScriptGameApi,
}

impl GameBindings {
    pub fn new(api: &ScriptGameApi) -> Self {
        Self { api: api.clone() }
    }

    pub fn spawn_unit(&self, owner: u32, q: i32, r: i32, kind: String) -> String {
        let cmd = self
            .api
            .spawn_unit(PlayerId(owner), HexCoordinate::new(q, r), &kind);
        format!("{:?}", cmd)
    }

    pub fn create_city(&self, owner: u32, q: i32, r: i32, name: String) -> String {
        let cmd = self
            .api
            .create_city(PlayerId(owner), HexCoordinate::new(q, r), &name);
        format!("{:?}", cmd)
    }

    pub fn end_turn(&self) -> GameCommand {
        self.api.end_turn()
    }

    pub fn add_gold(&self, player: u32, amount: i32) -> GameCommand {
        self.api.add_gold(PlayerId(player), amount)
    }

    pub fn get_player(&self, id: u32) -> GameCommand {
        self.api.get_player(PlayerId(id))
    }

    pub fn find_city(&self, name: String) -> GameCommand {
        self.api.find_city(&name)
    }
}
