use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioPlugin as KiraAudioPlugin;
use rand::Rng;
use crate::combat::{CombatState, FightEvent};
use crate::GameState;
use crate::math::clamp01;

// Plugin
// =========================================================================

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(KiraAudioPlugin)
			.add_audio_channel::<BgMusicChannel>()
			.add_audio_channel::<CombatMusicChannel>()
			.add_audio_channel::<SfxChannel>()
			.add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
			.add_system(volume_control)
			.add_system_set(
				SystemSet::on_enter(GameState::Combat)
					.with_system(start_combat_music)
			)
			.add_system_set(
				SystemSet::on_update(GameState::Combat)
					.with_system(play_hit_sfx)
			)
			.add_system_set(
				SystemSet::on_exit(GameState::Combat)
					.with_system(stop_music::<CombatMusicChannel>)
			)
			.add_system_set(
				SystemSet::on_enter(GameState::Overworld)
					.with_system(start_bg_music)
			)
			.add_system_set(
				SystemSet::on_resume(GameState::Overworld)
					.with_system(resume_music::<BgMusicChannel>)
			)
			.add_system_set(
				SystemSet::on_pause(GameState::Overworld)
					.with_system(pause_music::<BgMusicChannel>)
			)
			.add_system_set(
				SystemSet::on_enter(CombatState::Success)
					.with_system(play_success_sfx)
			)
		;
	}
}

// State
// =========================================================================

#[derive(Resource)]
pub struct BgMusicChannel;
#[derive(Resource)]
pub struct CombatMusicChannel;
#[derive(Resource)]
pub struct SfxChannel;

#[derive(Resource)]
pub struct AudioState {
	hit_sfx : Handle<AudioSource>,
	success_sfx : Handle<AudioSource>,

	combat_music : Handle<AudioSource>,
	overworld_music : Handle<AudioSource>,

	volume : f64,
}

// Systems
// =========================================================================

fn load_audio (
	mut commands : Commands,
	assets : Res<AssetServer>,
	bg_music_channel : Res<AudioChannel<BgMusicChannel>>,
	combat_music_channel : Res<AudioChannel<CombatMusicChannel>>,
	sfx_channel : Res<AudioChannel<SfxChannel>>,
) {
	let hit_sfx = assets.load("audio/sfx/hit.ogg");
	let success_sfx = assets.load("audio/sfx/success.ogg");
	let combat_music = assets.load("audio/music/Cruising-for-Goblins.ogg");
	let overworld_music = assets.load("audio/music/Kirk-Osamayo-Video-Game-Snowy-Night.ogg");

	let volume: f64 = 0.5;

	bg_music_channel.set_volume(volume * 0.25);
	combat_music_channel.set_volume(volume * 0.25);
	sfx_channel.set_volume(volume);

	commands.insert_resource::<AudioState>(AudioState {
		hit_sfx,
		success_sfx,
		combat_music,
		overworld_music,

		volume,
	});
}

fn volume_control (
	keyboard : Res<Input<KeyCode>>,
	mut state: ResMut<AudioState>,
	bg_music_channel : Res<AudioChannel<BgMusicChannel>>,
	combat_music_channel : Res<AudioChannel<CombatMusicChannel>>,
	sfx_channel : Res<AudioChannel<SfxChannel>>,
) {
	if keyboard.just_pressed(KeyCode::Minus) { state.volume -= 0.1; }
	if keyboard.just_pressed(KeyCode::Equals) { state.volume += 0.1; }

	state.volume = clamp01(state.volume as f32) as f64;
	bg_music_channel.set_volume(state.volume * 0.25);
	combat_music_channel.set_volume(state.volume * 0.25);
	sfx_channel.set_volume(state.volume);
}

fn start_bg_music (
	bg_music_channel : Res<AudioChannel<BgMusicChannel>>,
	state : Res<AudioState>,
) {
	bg_music_channel.play(state.overworld_music.clone()).looped();
}

fn start_combat_music (
	combat_music_channel : Res<AudioChannel<CombatMusicChannel>>,
	state : Res<AudioState>,
) {
	combat_music_channel.play(state.combat_music.clone()).looped();
}

fn pause_music <T : Resource> (
	channel : Res<AudioChannel<T>>,
) {
	channel.pause();
}

fn resume_music <T : Resource> (
	channel : Res<AudioChannel<T>>,
) {
	channel.resume();
}

fn stop_music <T : Resource> (
	channel : Res<AudioChannel<T>>,
) {
	channel.stop();
}

fn play_hit_sfx (
	channel : Res<AudioChannel<SfxChannel>>,
	state : Res<AudioState>,
	events : EventReader<FightEvent>,
) {
	if !events.is_empty() {
		channel
			.play(state.hit_sfx.clone())
			.with_playback_rate(rand::thread_rng().gen_range(0.5..=1.5f64));
	}
}

fn play_success_sfx (
	channel : Res<AudioChannel<SfxChannel>>,
	state : Res<AudioState>,
) {
	channel.play(state.success_sfx.clone());
}
