use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;
use crate::core::assets::{spawn_tilesheet_sprite, Tilesheet};
use crate::TILE_SIZE;
use crate::npc::Npc;

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

pub fn create_simple_map (
	name : &str,
	commands : &mut Commands,
	tilesheet : Res<Tilesheet>,
) -> Entity {
	let file = File::open(
		format!("assets/maps/{}.txt", name)
	).expect("Map file missing!");
	let mut tiles = Vec::new();

	for (y, line) in BufReader::new(file).lines().enumerate() {
		if line.is_err() { continue }

		let line = line.unwrap();

		for (x, char) in line.chars().enumerate() {
			let tile = spawn_tilesheet_sprite(
				commands,
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
		.id()
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
