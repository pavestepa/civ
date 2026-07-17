use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution};

use civ_ai::AiPlugin;
use civ_assets::AssetsPlugin;
use civ_combat::CombatPlugin;
use civ_commands::CommandsPlugin;
use civ_culture::CulturePlugin;
use civ_diplomacy::DiplomacyPlugin;
use civ_ecs::EcsPlugin;
use civ_economy::EconomyPlugin;
use civ_events::EventsPlugin;
use civ_fog::FogPlugin;
use civ_gameplay::GameplayPlugin;
use civ_government::GovernmentPlugin;
use civ_great_people::GreatPeoplePlugin;
use civ_movement::MovementPlugin;
use civ_network::NetworkPlugin;
use civ_pathfinding::PathfindingPlugin;
use civ_production::ProductionPlugin;
use civ_quickjs::QuickJsPlugin;
use civ_religion::ReligionPlugin;
use civ_render::RenderPlugin;
use civ_research::ResearchPlugin;
use civ_resources::GameResourcesPlugin;
use civ_save::SavePlugin;
use civ_scripting::ScriptingPlugin;
use civ_simulation::SimulationPlugin;
use civ_terrain::TerrainPlugin;
use civ_trade::TradePlugin;
use civ_ui_bridge::UiBridgePlugin;

use super::window::WindowConfigPlugin;

/// Root plugin group wiring all subsystems.
pub struct EnginePluginGroup;

impl Plugin for EnginePluginGroup {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Civ".into(),
                        resolution: WindowResolution::new(1600.0, 900.0),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .disable::<bevy::audio::AudioPlugin>()
                .disable::<bevy::log::LogPlugin>(),
        )
        .add_plugins(WindowConfigPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(CommandsPlugin)
        .add_plugins(GameResourcesPlugin)
        .add_plugins(SavePlugin)
        .add_plugins(PathfindingPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(EcsPlugin)
        .add_plugins(RenderPlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(EconomyPlugin)
        .add_plugins(ResearchPlugin)
        .add_plugins(ProductionPlugin)
        .add_plugins(CulturePlugin)
        .add_plugins(ReligionPlugin)
        .add_plugins(DiplomacyPlugin)
        .add_plugins(GovernmentPlugin)
        .add_plugins(GreatPeoplePlugin)
        .add_plugins(TradePlugin)
        .add_plugins(FogPlugin)
        .add_plugins(AiPlugin)
        .add_plugins(NetworkPlugin)
        .add_plugins(ScriptingPlugin)
        .add_plugins(QuickJsPlugin)
        .add_plugins(UiBridgePlugin);
    }
}
