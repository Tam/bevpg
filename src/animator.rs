use bevy::prelude::*;
use crate::PIXEL_SIZE;

pub fn walk_cycle (path : Vec<Name>) -> AnimationClip {
	let mut animation = AnimationClip::default();

	animation.add_curve_to_path(
		EntityPath { parts: path.clone() },
		VariableCurve {
			keyframe_timestamps: vec![0., 0.2, 0.4, 0.6, 0.8],
			keyframes: Keyframes::Translation(vec![
				Vec3::new(0., 0., 0.),
				Vec3::new(0., PIXEL_SIZE * 2., 0.),
				Vec3::new(0., 0., 0.),
				Vec3::new(0., PIXEL_SIZE * 2., 0.),
				Vec3::new(0., 0., 0.),
			]),
		},
	);

	animation.add_curve_to_path(
		EntityPath { parts: path.clone() },
		VariableCurve {
			keyframe_timestamps: vec![0., 0.2, 0.4, 0.6, 0.8],
			keyframes: Keyframes::Rotation(vec![
				Quat::IDENTITY,
				Quat::from_rotation_z(0.15),
				Quat::IDENTITY,
				Quat::from_rotation_z(-0.15),
				Quat::IDENTITY,
			]),
		},
	);

	animation
}
