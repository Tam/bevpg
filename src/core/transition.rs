use bevy::prelude::*;
use crate::GameState;
use crate::util::math::{clamp01, lerp3};

// Plugin
// =========================================================================

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(fadeout)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
struct ScreenFade {
	alpha : f32,
	sent : bool,
	next_state : Option<GameState>,
	timer : Timer,
}

// Systems
// =========================================================================

fn fadeout (
	mut commands : Commands,
	mut query : Query<(Entity, &mut ScreenFade, &mut BackgroundColor)>,
	mut state : ResMut<State<GameState>>,
	time : Res<Time>,
) {
	for (id, mut fade, mut fill) in query.iter_mut() {
		fade.timer.tick(time.delta());

		fade.alpha = clamp01(lerp3(0., 1.25, 0., fade.timer.percent()));
		fill.0.set_a(fade.alpha);

		if fade.timer.percent() > 0.5 && !fade.sent {
			if let Some(next) = fade.next_state {
				state.push(next).unwrap();
			} else {
				state.pop().unwrap();
			}
			fade.sent = true;
		}

		if fade.timer.just_finished() {
			commands.entity(id).despawn_recursive();
		}
	}
}

// Utilities
// =========================================================================

pub fn create_fadeout (
	commands : &mut Commands,
	next_state : Option<GameState>,
) {
	let mut color = Color::hex("432E3B").unwrap();
	color.set_a(0.0);

	commands
		.spawn(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				size: Size::new(Val::Percent(100.), Val::Percent(100.)),
				..default()
			},
			z_index: ZIndex::Global(999),
			background_color: BackgroundColor(color),
			..default()
		})
		.insert(ScreenFade {
			alpha: 0.,
			sent: false,
			next_state,
			timer: Timer::from_seconds(1., TimerMode::Once),
		})
		.insert(Name::new("Fadeout"))
	;
}
