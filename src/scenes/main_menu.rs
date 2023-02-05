use bevy::prelude::*;
use crate::assets::PixelFont;
use crate::GameState;
use crate::transition::create_fadeout;
use crate::ui::Disabled;

// Plugin
// =========================================================================

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(setup_menu)
			.add_system_set(
				SystemSet::on_resume(GameState::MainMenu)
					.with_system(set_ui_visibility(true))
			)
			.add_system_set(
				SystemSet::on_update(GameState::MainMenu)
					.with_system(on_start_click)
			)
			.add_system_set(
				SystemSet::on_pause(GameState::MainMenu)
					.with_system(set_ui_visibility(false))
			)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct MainMenuUIRoot;

#[derive(Component)]
pub struct Active(bool);

// Systems
// =========================================================================

fn setup_menu (
	mut commands : Commands,
	pixel_font : Res<PixelFont>,
) {
	commands.spawn((
		MainMenuUIRoot,
		NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.), Val::Percent(100.)),
				..default()
			},
			..default()
		},
	)).with_children(|parent| {
		parent.spawn((
			ButtonBundle {
				style: Style {
					margin: UiRect::all(Val::Auto),
					padding: UiRect::new(
						Val::Px(30.), Val::Px(30.),
						Val::Px(10.), Val::Px(10.),
					),
					..default()
				},
				background_color: Color::hex("6ED57E").unwrap().into(),
				..default()
			},
		)).with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				"Start Game",
				TextStyle {
					font: pixel_font.0.clone(),
					font_size: 40.,
					color: Color::WHITE,
				},
			));
		});
	});
}

fn on_start_click (
	mut commands : Commands,
	interaction_query : Query<(Entity, &Interaction), (Changed<Interaction>, Without<Disabled>)>,
) {
	for (entity, interaction) in &interaction_query {
		if interaction == &Interaction::Clicked {
			commands.entity(entity).insert(Disabled);
			create_fadeout(
				&mut commands,
				Some(GameState::Overworld)
			);
		}
	}
}

fn set_ui_visibility (is_visible : bool) -> impl Fn(Query<&mut Visibility, With<MainMenuUIRoot>>) {
	move |
		mut query : Query<&mut Visibility, With<MainMenuUIRoot>>,
	| { query.single_mut().is_visible = is_visible; }
}
