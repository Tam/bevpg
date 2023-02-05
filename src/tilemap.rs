use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;
use crate::assets::{spawn_tilesheet_sprite,Tilesheet};
use crate::{GameState, TILE_SIZE};
use crate::npc::Npc;

// Plugin
// =========================================================================

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system_set(
				SystemSet::on_enter(GameState::Overworld)
					.with_system(create_simple_map)
			)
			.add_system_set(
				SystemSet::on_resume(GameState::Overworld)
					.with_system(show_map)
			)
			.add_system_set(
				SystemSet::on_pause(GameState::Overworld)
					.with_system(hide_map)
			)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct EncounterSpawner;

// Systems
// =========================================================================

fn create_simple_map (
	mut commands : Commands,
	tilesheet : Res<Tilesheet>,
) {
	let file = File::open("assets/maps/test.txt").expect("Map file missing!");
	let mut tiles = Vec::new();

	for (y, line) in BufReader::new(file).lines().enumerate() {
		if line.is_err() { continue }

		let line = line.unwrap();

		for (x, char) in line.chars().enumerate() {
			let tile = spawn_tilesheet_sprite(
				&mut commands,
				&tilesheet,
				char_to_tile_index(char),
				Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.),
				None
			);

			match char {
				'#' => { commands.entity(tile).insert(TileCollider); }
				'~' => { commands.entity(tile).insert(EncounterSpawner); }
				'@' => {
					commands.entity(tile)
						.insert(Npc::Healer)
						.insert(TileCollider);
				}
				_ => {}
			}

			tiles.push(tile);
		}
	}

	commands
		.spawn((
			Name::new("Map"),
			GlobalTransform::default(),
			Transform::default(),
			Visibility::default(),
			ComputedVisibility::default(),
			Map,
		))
		.push_children(&tiles)
	;
}

fn show_map (mut query : Query<&mut Visibility, With<Map>>) {
	query.single_mut().is_visible = true;
}

fn hide_map (mut query : Query<&mut Visibility, With<Map>>) {
	query.single_mut().is_visible = false;
}

// Helpers
// =========================================================================

fn char_to_tile_index (c : char) -> usize {
	match c {
		'#' => 49 * 3 + 22,
		'~' => 5,
		'@' => 49 * 2 - 18,
		_ => 0,
	}
}
