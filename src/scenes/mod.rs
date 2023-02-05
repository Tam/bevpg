pub mod main_menu;
pub mod overworld;

use bevy::prelude::*;
use crate::scenes::main_menu::MainMenuPlugin;
use crate::scenes::overworld::OverworldPlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(MainMenuPlugin)
			.add_plugin(OverworldPlugin)
		;
	}
}
