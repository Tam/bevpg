use std::cmp::max;
use std::f32::consts::PI;
use bevy::prelude::*;
use crate::core::assets::{PixelFont, spawn_tilesheet_sprite, Tilesheet};
use crate::core::transition::create_fadeout;
use crate::GameState;
use crate::player::Player;
use crate::ui::Disabled;

// Plugin
// =========================================================================

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_state(CombatState::PlayerTurn)
			.add_event::<FightEvent>()
			.insert_resource(AttackEffects {
				timer: Timer::from_seconds(0.7, TimerMode::Repeating),
				flash: 0.1,
				shake: 0.5,
				current_shake: 0.,
			})
			.add_system_set(
				SystemSet::on_enter(GameState::Combat)
					.with_system(spawn_enemy)
					.with_system(spawn_combat_ui)
					.with_system(start_combat)
			)
			.add_system_set(
				SystemSet::on_update(GameState::Combat)
					.with_system(escape_combat)
					.with_system(combat_camera)
					.with_system(combat_input)
					.with_system(damage_calculation.label("damage_calculation"))
					.with_system(update_combat_ui.after("damage_calculation"))
			)
			.add_system_set(
				SystemSet::on_update(CombatState::EnemyTurn(false))
					.with_system(process_enemy_turn)
			)
			.add_system_set(
				SystemSet::on_update(CombatState::EnemyAttack)
					.with_system(handle_attack_effects)
			)
			.add_system_set(
				SystemSet::on_enter(CombatState::PlayerTurn)
					.with_system(set_ui_disabled(false))
			)
			.add_system_set(
				SystemSet::on_exit(CombatState::PlayerTurn)
					.with_system(set_ui_disabled(true))
			)
			.add_system_set(
				SystemSet::on_update(CombatState::PlayerAttack)
					.with_system(handle_attack_effects)
			)
			.add_system_set(
				SystemSet::on_enter(CombatState::Success)
					.with_system(handle_success)
			)
			.add_system_set(
				SystemSet::on_exit(GameState::Combat)
					.with_system(despawn_enemy)
					.with_system(despawn_combat_ui)
			)
		;
	}
}

// States
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CombatState {
	PlayerTurn,
	PlayerAttack,
	EnemyTurn (bool),
	EnemyAttack,
	Success,
}

// Events
// =========================================================================

pub struct FightEvent {
	target : Entity,
	damage : isize,
	next_state : CombatState,
}

// Resources
// =========================================================================

#[derive(Resource)]
pub struct AttackEffects {
	timer : Timer,
	flash : f32,
	shake : f32,
	current_shake : f32,
}

// Components
// =========================================================================

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct CombatStats {
	pub max_health : isize,
	pub health : isize,
	pub attack : isize,
	pub defence : isize,
}

#[derive(Component)]
pub struct CombatUIRoot;

#[derive(Component)]
pub struct EnemyHealthText;

#[derive(Component)]
pub struct PlayerHealthText;

// Systems
// =========================================================================

// Combat Interaction
// -------------------------------------------------------------------------

fn start_combat (
	mut state : ResMut<State<CombatState>>,
) {
	let _ = state.set(CombatState::PlayerTurn);
}

fn handle_attack_effects (
	mut attack_fx : ResMut<AttackEffects>,
	time : Res<Time>,
	mut enemy_query : Query<&mut Visibility, With<Enemy>>,
	mut state : ResMut<State<CombatState>>,
) {
	attack_fx.timer.tick(time.delta());
	let mut enemy = enemy_query.single_mut();

	match state.current() {
		&CombatState::PlayerAttack => {
			if attack_fx.timer.just_finished() {
				enemy.is_visible = true;
				state.set(CombatState::EnemyTurn(false)).unwrap();
			} else {
				enemy.is_visible = attack_fx.timer.elapsed_secs() % attack_fx.flash > attack_fx.flash * 0.5;
			}
		},
		&CombatState::EnemyAttack => {
			if attack_fx.timer.just_finished() {
				state.set(CombatState::PlayerTurn).unwrap();
			} else {
				attack_fx.current_shake = attack_fx.shake * f32::sin(
					attack_fx.timer.percent() * 2. * PI
				);
			}
		},
		_ => {},
	}
}

fn damage_calculation (
	mut commands : Commands,
	mut fight_event : EventReader<FightEvent>,
	mut target_query: Query<&mut CombatStats>,
	mut combat_state : ResMut<State<CombatState>>,
) {
	for event in fight_event.iter() {
		let mut target_stats = target_query
			.get_mut(event.target)
			.expect("Target missing combat stats!");

		target_stats.health = max(
			target_stats.health - (event.damage - target_stats.defence),
			0,
		);

		if target_stats.health == 0 {
			create_fadeout(
				&mut commands,
				None,
			);

			combat_state.set(CombatState::Success).expect("Failed to set exit state");
		} else {
			combat_state.set(event.next_state).expect("Failed to set player turn state");
		}
	}
}

fn escape_combat (
	mut commands : Commands,
	query : Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
) {
	for (interaction, name) in &query {
		if *interaction == Interaction::Clicked && name.as_str() == "run" {
			create_fadeout(
				&mut commands,
				None,
			);
		}
	}
}

fn combat_input (
	query : Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
	mut fight_event : EventWriter<FightEvent>,
	player_query : Query<&CombatStats, With<Player>>,
	enemy_query : Query<Entity, With<Enemy>>,
	state : Res<State<CombatState>>,
) {
	if state.current() != &CombatState::PlayerTurn { return; }

	let player = player_query.single();
	let target = enemy_query.single();

	for (interaction, name) in &query {
		if *interaction == Interaction::Clicked && name.as_str() == "fight" {
			fight_event.send(FightEvent {
				target,
				damage: player.attack,
				next_state: CombatState::PlayerAttack,
			});
		}
	}
}

fn process_enemy_turn (
	mut combat_state : ResMut<State<CombatState>>,
	mut fight_event : EventWriter<FightEvent>,
	enemy_query : Query<&CombatStats, With<Enemy>>,
	player_query : Query<Entity, With<Player>>,
) {
	let stats = enemy_query.single();
	let target = player_query.single();

	combat_state.set(CombatState::EnemyTurn(true)).expect("Fail mark enemy state");

	fight_event.send(FightEvent {
		target,
		damage: stats.attack,
		next_state: CombatState::EnemyAttack,
	});
}

fn handle_success (
	mut player_query : Query<&mut Player>,
) {
	player_query.single_mut().xp += 10;
	println!("Player gains 10xp for a total of {}xp!", player_query.single_mut().xp);
}

// Combat UI
// -------------------------------------------------------------------------

fn set_ui_disabled (is_disabled : bool) -> impl Fn(Commands, Query<Entity, With<Button>>) {
	move |
		mut commands : Commands,
		query : Query<Entity, With<Button>>,
	| {
		for btn in &query {
			let mut e = commands.entity(btn);
			if is_disabled { e.insert(Disabled); }
			else { e.remove::<Disabled>(); }
		}
	}
}

fn spawn_combat_ui (
	mut commands : Commands,
	pixel_font : Res<PixelFont>,
) {
	commands
		.spawn((
			CombatUIRoot,
			NodeBundle {
				style: Style {
					size: Size::new(Val::Percent(100.), Val::Percent(100.)),
					position_type: PositionType::Absolute,
					flex_direction: FlexDirection::Column,
					padding: UiRect::all(Val::Px(30.)),
					justify_content: JustifyContent::SpaceBetween,
					..default()
				},
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					align_items: AlignItems::FlexStart,
					..default()
				},
				..default()
			})
				.with_children(|parent| {
					parent.spawn((
						TextBundle::from_section(
							"Enemy HP: X",
							TextStyle {
								font: pixel_font.0.clone(),
								font_size: 40.,
								color: Color::WHITE,
							},
						),
						EnemyHealthText,
					));

					parent.spawn((
						TextBundle::from_section(
							"Player HP: X",
							TextStyle {
								font: pixel_font.0.clone(),
								font_size: 40.,
								color: Color::WHITE,
							},
						),
						PlayerHealthText,
					));
				});

			parent.spawn(NodeBundle {
				style: Style {
					align_items: AlignItems::FlexEnd,
					justify_content: JustifyContent::FlexEnd,
					align_self: AlignSelf::FlexEnd,
					..default()
				},
				..default()
			}).with_children(|parent| {
				parent.spawn((
					ButtonBundle {
						style: Style {
							size: Size::new(Val::Px(150.), Val::Px(65.)),
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							margin: UiRect::right(Val::Px(10.)),
							..default()
						},
						background_color: Color::hex("D5543B").unwrap().into(),
						..default()
					},
					Name::new("fight")
				)).with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Fight",
						TextStyle {
							font: pixel_font.0.clone(),
							font_size: 40.,
							color: Color::WHITE,
						},
					));
				});

				parent.spawn((
					ButtonBundle {
						style: Style {
							size: Size::new(Val::Px(150.), Val::Px(65.)),
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							..default()
						},
						background_color: Color::hex("EAB644").unwrap().into(),
						..default()
					},
					Name::new("run"),
				)).with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Run",
						TextStyle {
							font: pixel_font.0.clone(),
							font_size: 40.,
							color: Color::WHITE,
						},
					));
				});
			});
		});
}

fn update_combat_ui (
	enemy_stats_query : Query<&CombatStats, (With<Enemy>, Without<Player>)>,
	player_stats_query : Query<&CombatStats, (With<Player>, Without<Enemy>)>,
	mut enemy_health_text_query : Query<&mut Text, (With<EnemyHealthText>, Without<PlayerHealthText>)>,
	mut player_health_text_query : Query<&mut Text, (With<PlayerHealthText>, Without<EnemyHealthText>)>,
) {
	if let Ok(mut enemy_health_text) = enemy_health_text_query.get_single_mut() {
		if let Ok(enemy_stats) = enemy_stats_query.get_single() {
			enemy_health_text.sections[0].value = format!("Enemy HP: {}", enemy_stats.health);
		}
	}

	if let Ok(mut player_health_text) = player_health_text_query.get_single_mut() {
		if let Ok(player_stats) = player_stats_query.get_single() {
			player_health_text.sections[0].value = format!("Player HP: {}", player_stats.health);
		}
	}
}

fn despawn_combat_ui (
	mut commands : Commands,
	query : Query<Entity, With<CombatUIRoot>>,
) {
	commands.entity(query.single()).despawn_recursive();
}

// Enemy
// -------------------------------------------------------------------------

fn spawn_enemy (
	mut commands : Commands,
	tilesheet : Res<Tilesheet>,
) {
	let id = spawn_tilesheet_sprite(
		&mut commands,
		&tilesheet,
		49 * 2 + 25,
		Vec3::ZERO,
		None,
	);

	commands
		.entity(id)
		.insert(Name::new("Goblin"))
		.insert(Enemy)
		.insert(CombatStats {
			max_health: 7,
			health: 7,
			attack: 2,
			defence: 1,
		})
	;
}

fn despawn_enemy (
	mut commands : Commands,
	query : Query<Entity, With<Enemy>>,
) {
	for id in &query {
		commands
			.entity(id)
			.despawn_recursive();
	}
}

// Scene
// -------------------------------------------------------------------------

fn combat_camera (
	mut query : Query<&mut Transform, With<Camera>>,
	mut ui_query : Query<&mut Style, With<CombatUIRoot>>,
	attack_fx : Res<AttackEffects>,
) {
	let mut camera = query.single_mut();
	camera.translation.x = attack_fx.current_shake;
	camera.translation.y = 0.;

	ui_query.single_mut().position = UiRect {
		left: Val::Percent(-attack_fx.current_shake),
		right: Val::Percent(attack_fx.current_shake),
		..default()
	};
}
