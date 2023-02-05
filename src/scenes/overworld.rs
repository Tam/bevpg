use bevy::prelude::*;
use crate::GameState;

// Plugin
// =========================================================================

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system_set(
				SystemSet::on_enter(GameState::Overworld)
					.with_system(spawn_scene)
			)
			.add_system_set(
				SystemSet::on_pause(GameState::Overworld)
					.with_system(hide_scene)
			)
			.add_system_set(
				SystemSet::on_resume(GameState::Overworld)
					.with_system(show_scene)
			)
			.add_system_set(
				SystemSet::on_exit(GameState::Overworld)
					.with_system(despawn_scene)
			)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct SceneOverworld;

// Systems
// =========================================================================

fn spawn_scene (
	mut commands : Commands,
) {
	commands.spawn((
		SceneOverworld,
		GlobalTransform::default(),
		Visibility::default(),
	));

	// TODO: attach all entities to SceneOverworld
}

fn despawn_scene (
	mut commands : Commands,
	query : Query<Entity, With<SceneOverworld>>,
) {
	commands.entity(query.single()).despawn_recursive();
}

fn show_scene (
	mut query : Query<&mut Visibility, With<SceneOverworld>>,
) {
	query.single_mut().is_visible = true;
}

fn hide_scene (
	mut query : Query<&mut Visibility, With<SceneOverworld>>,
) {
	query.single_mut().is_visible = false;
}
