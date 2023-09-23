mod button;
pub mod dialog;

use bevy::prelude::*;
use bevy_ninepatch::NinePatchPlugin;
use crate::ui::dialog::UIDialogPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(NinePatchPlugin::<()>::default())
			.add_plugin(UIDialogPlugin)
			.add_system(button::button_interaction)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct Disabled;
