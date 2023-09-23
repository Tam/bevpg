use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

pub struct UIDialogPlugin;

impl Plugin for UIDialogPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(init_dialog_ui)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
struct UIDialogRoot;

// Systems
// =========================================================================

fn init_dialog_ui (
	mut commands : Commands,
	assets : Res<AssetServer>,
	mut patches : ResMut<Assets<NinePatchBuilder<()>>>,
) {
	let texture : Handle<Image> = assets.load("ui/dialog-box.png");
	let nine_patch = patches.add(NinePatchBuilder::by_margins(
		30,30,30,30
	));

	commands.spawn((
		UIDialogRoot,
		NinePatchBundle {
			style: Style {
				position_type: PositionType::Absolute,
				position: UiRect {
					bottom: Val::Px(30.),
					left: Val::Percent(10.),
					right: Val::Percent(10.),
					..default()
				},
				size: Size::new(Val::Percent(80.), Val::Px(150.)),
				..default()
			},
			nine_patch_data: NinePatchData {
				nine_patch,
				texture,
				..default()
			},
			..default()
		}
	));
}
