use bevy::prelude::*;
use crate::player::Player;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		if !cfg!(debug_assertions) { return; }

		app
			.register_type::<Player>()
			// .add_plugin(WorldInspectorPlugin)
		;
	}
}
