use bevy::prelude::*;
use crate::TILE_SIZE;

// PLugin
// =========================================================================

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
		;
	}
}

// Resources
// =========================================================================

#[derive(Resource)]
pub struct Tilesheet (Handle<TextureAtlas>);

#[derive(Resource)]
pub struct PixelFont (pub Handle<Font>);

// Systems
// =========================================================================

fn load_assets (
	mut commands : Commands,
	assets : Res<AssetServer>,
	mut atlas : ResMut<Assets<TextureAtlas>>,
) {
	let tilesheet_image = assets.load("tilesheet.png");
	let tilesheet_atlas = TextureAtlas::from_grid(
		tilesheet_image,
		Vec2::splat(16.),
		49,
		22,
		Some(Vec2::splat(1.)),
		None,
	);

	let handle = atlas.add(tilesheet_atlas);
	commands.insert_resource(Tilesheet(handle));

	let font = assets.load("Kenney Mini.ttf");
	commands.insert_resource(PixelFont(font));
}

// Utilities
// =========================================================================

pub fn spawn_tilesheet_sprite (
	commands : &mut Commands,
	tilesheet : &Res<Tilesheet>,
	index : usize,
	translation : Vec3,
	tint : Option<Color>,
) -> Entity {
	spawn_tilesheet_sprite_with_size(
		commands,
		tilesheet,
		index,
		translation,
		tint,
		Some(Vec2::splat(TILE_SIZE)),
	)
}

pub fn spawn_tilesheet_sprite_with_size (
	commands : &mut Commands,
	tilesheet : &Res<Tilesheet>,
	index : usize,
	translation : Vec3,
	tint : Option<Color>,
	size : Option<Vec2>,
) -> Entity {
	let mut sprite = TextureAtlasSprite::new(index);
	sprite.custom_size = size;
	if let Some(tint) = tint { sprite.color = tint; }

	commands.spawn(SpriteSheetBundle {
		sprite,
		texture_atlas: tilesheet.0.clone(),
		transform: Transform {
			translation,
			..default()
		},
		..default()
	}).id()
}
