use bevy::prelude::*;
use crate::{GameState, TILE_SIZE};
use crate::combat::CombatStats;
use crate::core::assets::{PixelFont, spawn_tilesheet_sprite, Tilesheet};
use crate::player::Player;

// Plugin
// =========================================================================

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system_set(
				SystemSet::on_enter(GameState::Overworld)
					.with_system(setup_dialog_ui)
			)
			.add_system_set(
				SystemSet::on_update(GameState::Overworld)
					.with_system(npc_dialog)
					.with_system(highlight_npc)
			)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct NpcDialogUIRoot;

#[derive(Component)]
pub struct NpcDialogUIText;

#[derive(Component)]
pub struct NpcBubble;

#[derive(Component)]
pub enum Npc {
	Healer,
}

// Systems
// =========================================================================

fn setup_dialog_ui (
	mut commands : Commands,
	pixel_font : Res<PixelFont>,
	tilesheet : Res<Tilesheet>,
) {
	let id = spawn_tilesheet_sprite(
		&mut commands,
		&tilesheet,
		49 * 14 - 11,
		Vec3::ZERO,
		None,
	);

	commands.entity(id).insert((
		NpcBubble,
		Transform {
			translation: Vec3::new(0., TILE_SIZE * 0.75, 0.),
			scale: Vec3::splat(0.5),
			..default()
		},
		Visibility { is_visible: false },
	));

	commands.spawn((
		NpcDialogUIRoot,
		NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				size: Size::new(Val::Percent(100.), Val::Percent(100.)),
				padding: UiRect::all(Val::Px(30.)),
				align_items: AlignItems::FlexEnd,
				..default()
			},
			visibility: Visibility {
				is_visible: false,
				..default()
			},
			..default()
		},
	)).with_children(|parent| {
		let mut bg = Color::hex("432E3B").unwrap();
		bg.set_a(0.9);

		parent.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(80.), Val::Px(150.)),
				margin: UiRect::left(Val::Percent(10.)),
				padding: UiRect::all(Val::Px(30.)),
				..default()
			},
			background_color: BackgroundColor::from(bg),
			..default()
		}).with_children(|parent| {
			parent.spawn((
				TextBundle::from_section(
					"hi",
					TextStyle {
						font: pixel_font.0.clone(),
						font_size: 40.,
						color: Color::WHITE,
					},
				),
				NpcDialogUIText,
			));

			parent.spawn(TextBundle::from_section(
				"Press SPACE to continue",
				TextStyle {
					font: pixel_font.0.clone(),
					font_size: 24.,
					color: Color::WHITE,
				},
			).with_style(Style {
				position_type: PositionType::Absolute,
				position: UiRect::new(Val::Undefined, Val::Px(30.), Val::Undefined, Val::Px(30.)),
				..default()
			}));
		});
	});
}

fn npc_dialog (
	mut player_query : Query<(&mut Player, &Transform, &mut CombatStats)>,
	mut ui : Query<&mut Visibility, With<NpcDialogUIRoot>>,
	mut ui_text : Query<&mut Text, With<NpcDialogUIText>>,
	npc_query : Query<(&Npc, &Transform)>,
	keyboard : Res<Input<KeyCode>>,
) {
	let (mut player, player_transform, mut stats) = player_query.single_mut();

	if !player.active {
		if keyboard.any_just_pressed([KeyCode::Space, KeyCode::E]) {
			ui.single_mut().is_visible = false;
			player.active = true;
		}

		return;
	}

	if !keyboard.just_pressed(KeyCode::E) { return; }

	for _npc in npc_query.iter().filter(|(_, transform)| {
		Vec2::distance(
			transform.translation.truncate(),
			player_transform.translation.truncate(),
		) < TILE_SIZE * 1.25
	}) {
		ui_text.single_mut().sections[0].value = "Heal, heal, HEAL!".into();
		ui.single_mut().is_visible = true;
		player.active = false;
		stats.health = stats.max_health;
	}
}

fn highlight_npc (
	mut commands : Commands,
	mut player_query : Query<&Transform, With<Player>>,
	npc_query : Query<(Entity, &Npc, &Transform)>,
	mut bubble_query : Query<(Entity, &mut Visibility), With<NpcBubble>>,
) {
	let player_transform = player_query.single_mut();
	let (id, mut visibility) = bubble_query.single_mut();

	for (entity, _npc, _) in npc_query.iter().filter(|(_, _, transform)| {
		Vec2::distance(
			transform.translation.truncate(),
			player_transform.translation.truncate(),
		) < TILE_SIZE * 1.5
	}) {
		commands.entity(id).remove_parent();

		commands
			.entity(entity)
			.push_children(&[id]);

		visibility.is_visible = true;

		return;
	}

	commands.entity(id).remove_parent();
	visibility.is_visible = false;
}
