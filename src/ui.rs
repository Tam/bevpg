use bevy::prelude::*;

// Plugin
// =========================================================================

pub struct UiPlugin;

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(button_interaction)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct Disabled;

// Systems
// =========================================================================

fn button_interaction (
	mut query : Query<(&Interaction, &mut Style, Option<&Disabled>), (Changed<Interaction>, With<Button>)>,
) {
	for (interaction, mut style, disabled) in &mut query {
		if disabled.is_some() {
			style.position = UiRect::top(Val::Px(0.));
			continue;
		}

		match *interaction {
			Interaction::Clicked => {
				style.position = UiRect::top(Val::Px(3.));
			}
			Interaction::Hovered => {
				style.position = UiRect::top(Val::Px(-3.));
			}
			Interaction::None => {
				style.position = UiRect::top(Val::Px(0.));
			}
		}
	}
}
