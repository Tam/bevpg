use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;
use crate::{GameState, PIXEL_SIZE, TILE_SIZE};
use crate::combat::CombatStats;
use crate::core::animator;
use crate::core::assets::{spawn_tilesheet_sprite, Tilesheet};
use crate::core::tilemap::{EncounterSpawner, TileCollider};
use crate::core::transition::create_fadeout;

// Plugin
// =========================================================================

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system_set(
				SystemSet::on_enter(GameState::Overworld)
					.with_system(spawn_player)
			)
			.add_system_set(
				SystemSet::on_resume(GameState::Overworld)
					.with_system(show_player)
			)
			.add_system_set(
				SystemSet::on_update(GameState::Overworld)
					.with_system(player_movement)
					.with_system(cam_follow_player.after(player_movement))
					.with_system(player_encounter_checker.after(player_movement))
			)
			.add_system_set(
				SystemSet::on_pause(GameState::Overworld)
					.with_system(hide_player)
			)
		;
	}
}

// Components
// =========================================================================

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
	pub active : bool,
	speed : f32,
	just_moved : bool,
	walk_cycle: Handle<AnimationClip>,
	pub xp : usize,
}

#[derive(Component)]
pub struct EncounterTimer (Timer);

// Systems
// =========================================================================

fn spawn_player (
	mut commands : Commands,
	tilesheet : Res<Tilesheet>,
	mut animations : ResMut<Assets<AnimationClip>>,
) {
	let player_name = Name::new("Player");
	let player_sprite_name = Name::new("Player Sprite");

	let walk_cycle = animator::walk_cycle(vec![
		player_name.clone(),
		player_sprite_name.clone(),
	]);
	let walk_cycle_handle = animations.add(walk_cycle);

	let player_sprite_id = spawn_tilesheet_sprite(
		&mut commands,
		&tilesheet,
		25,
		Vec3::ZERO,
		None,
	);

	commands.entity(player_sprite_id).insert(player_sprite_name);

	commands
		.spawn((
			player_name,
			AnimationPlayer::default(),
			Transform {
				translation: Vec3::new(2. * TILE_SIZE, -2. * TILE_SIZE, 900.),
				..default()
			},
			GlobalTransform::default(),
			Visibility::default(),
			ComputedVisibility::default(),
			Player {
				active: true,
				speed: 4.,
				just_moved: false,
				walk_cycle: walk_cycle_handle,
				xp: 0,
			},
			EncounterTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
			CombatStats {
				max_health: 15,
				health: 15,
				attack: 3,
				defence: 1,
			},
		)).push_children(&[player_sprite_id]);
}

fn show_player (mut query : Query<(&mut Visibility, &mut Player)>) {
	let (mut visibility, mut player) = query.single_mut();
	visibility.is_visible = true;
	player.active = true;
}

fn hide_player (mut query : Query<&mut Visibility, With<Player>>) {
	query.single_mut().is_visible = false;
}

fn player_movement (
	mut player_query : Query<(&mut Player, &mut Transform, &mut AnimationPlayer)>,
	wall_query : Query<&Transform, (With<TileCollider>, Without<Player>)>,
	keyboard : Res<Input<KeyCode>>,
	time : Res<Time>,
) {
	let (mut player, mut transform, mut anim_player) = player_query.single_mut();
	player.just_moved = false;

	if !player.active {
		anim_player.set_elapsed(0.);
		anim_player.stop_repeating();
		return;
	}

	let mut delta_x = 0.0;
	let mut delta_y = 0.0;

	if keyboard.pressed(KeyCode::A) { delta_x += -1. }
	if keyboard.pressed(KeyCode::D) { delta_x += 1. }
	if keyboard.pressed(KeyCode::W) { delta_y += 1. }
	if keyboard.pressed(KeyCode::S) { delta_y += -1. }

	let dist = player.speed * TILE_SIZE * time.delta_seconds();
	let prev = transform.translation;

	let target = transform.translation + Vec3::new(delta_x * dist, 0., 0.);
	if !wall_query
		.iter()
		.any(|&transform| tile_collision_check(target, transform.translation))
	{ transform.translation = target; }

	let target = transform.translation + Vec3::new(0., delta_y * dist, 0.);
	if !wall_query
		.iter()
		.any(|&transform| tile_collision_check(target, transform.translation))
	{ transform.translation = target; }

	transform.translation.z = prev.z;
	player.just_moved = transform.translation.x != prev.x || transform.translation.y != prev.y;

	if player.just_moved {
		anim_player.play(player.walk_cycle.clone()).repeat();
	} else {
		anim_player.set_elapsed(0.);
		anim_player.stop_repeating();
	}
}

fn player_encounter_checker (
	mut commands : Commands,
	mut player_query : Query<(&mut Player, &Transform, &mut EncounterTimer), With<Player>>,
	encounter_query : Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
	time : Res<Time>,
) {
	let (mut player, player_transform, mut encounter_timer) = player_query.single_mut();
	let player_pos = player_transform.translation;

	if !player.just_moved { return; }

	if !encounter_query
		.iter()
		.any(|&transform| tile_collision_check(player_pos, transform.translation))
	{ return; }

	encounter_timer.0.tick(time.delta());
	if !encounter_timer.0.just_finished() { return; }

	encounter_timer.0.set_duration(
		Duration::from_millis(rand::thread_rng().gen_range(1000..=3000))
	);

	player.active = false;
	create_fadeout(
		&mut commands,
		Some(GameState::Combat),
	);
}

fn cam_follow_player (
	mut camera_query : Query<&mut Transform, (With<Camera>, Without<Player>)>,
	player_query : Query<&Transform, (With<Player>, Without<Camera>)>,
) {
	let mut camera_transform = camera_query.single_mut();
	let player_transform = player_query.single();

	camera_transform.translation.x = player_transform.translation.x;
	camera_transform.translation.y = player_transform.translation.y;
}

// Helpers
// =========================================================================

fn tile_collision_check(
	target_player_pos : Vec3,
	tile_translation: Vec3,
) -> bool {
	collide(
		target_player_pos,
		Vec2::splat(TILE_SIZE - PIXEL_SIZE),
		tile_translation,
		Vec2::splat(TILE_SIZE)
	).is_some()
}
