mod player;
mod debug;
mod assets;
mod tilemap;
mod combat;
mod transition;
mod math;
mod animator;
mod audio;
mod scenes;
mod ui;
mod npc;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PresentMode;
use crate::assets::AssetsPlugin;
use crate::audio::AudioPlugin;
use crate::combat::CombatPlugin;
use crate::debug::DebugPlugin;
use crate::npc::NpcPlugin;
use crate::player::PlayerPlugin;
use crate::scenes::ScenesPlugin;
use crate::tilemap::TilemapPlugin;
use crate::transition::TransitionPlugin;
use crate::ui::UiPlugin;

const TILE_SIZE : f32 = 1.;
const PIXEL_SIZE : f32 = (100. / 16.) / 100.;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    Overworld,
    Combat,
}

fn main() {
    App::new()
        .add_state(GameState::MainMenu)
        .insert_resource(ClearColor(Color::hex("432E3B").unwrap()))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Bevpg".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    },
                    ..default()
                })
        )
        .add_plugin(DebugPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NpcPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(TransitionPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ScenesPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(
    mut commands : Commands,
) {
    // Camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal(1.),
            scale: 20.,
            ..default()
        },
        ..default()
    });
}
