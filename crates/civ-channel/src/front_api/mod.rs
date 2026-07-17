use std::collections::HashMap;

use bevy::prelude::Resource;
use civ_commands::UiCommand;
use civ_simulation::SimulationState;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum FrontApiError {
    #[error("unknown operation: {0}")]
    UnknownOp(String),
    #[error("invalid request body for {op}: {source}")]
    InvalidBody {
        op: String,
        #[source]
        source: serde_json::Error,
    },
}

pub struct ApiContext<'a> {
    pub simulation: Option<&'a SimulationState>,
}

pub struct DispatchOutcome {
    pub ui_command: Option<UiCommand>,
    pub response: Value,
}

type HandlerFn = Box<dyn Fn(Value, &ApiContext<'_>) -> Result<DispatchOutcome, FrontApiError> + Send + Sync>;

#[derive(Resource)]
pub struct FrontApiRegistry {
    handlers: HashMap<&'static str, HandlerFn>,
}

impl Default for FrontApiRegistry {
    fn default() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };
        crate::register::register_front_api(&mut registry);
        registry
    }
}

impl FrontApiRegistry {
    pub fn register<Req, Res, F>(&mut self, op: &'static str, handler: F)
    where
        Req: DeserializeOwned + Send + Sync + 'static,
        Res: Serialize + Send + Sync + 'static,
        F: Fn(Req, &ApiContext<'_>) -> (Option<UiCommand>, Res) + Send + Sync + 'static,
    {
        self.handlers.insert(
            op,
            Box::new(move |body, ctx| {
                let req: Req = serde_json::from_value(body).map_err(|source| FrontApiError::InvalidBody {
                    op: op.to_string(),
                    source,
                })?;
                let (ui_command, res) = handler(req, ctx);
                Ok(DispatchOutcome {
                    ui_command,
                    response: serde_json::to_value(res).unwrap_or(serde_json::Value::Null),
                })
            }),
        );
    }

    pub fn dispatch(
        &self,
        op: &str,
        body: Value,
        ctx: &ApiContext<'_>,
    ) -> Result<DispatchOutcome, FrontApiError> {
        let handler = self
            .handlers
            .get(op)
            .ok_or_else(|| FrontApiError::UnknownOp(op.to_string()))?;
        handler(body, ctx)
    }
}
