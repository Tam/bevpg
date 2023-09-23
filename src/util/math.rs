use std::ops::RangeInclusive;

pub fn clamp01 (a : f32) -> f32 {
	a.clamp(0., 1.)
}

pub fn lerp3 (a : f32, b : f32, c : f32, t : f32) -> f32 {
	if t <= 0.5 {
		lerp(a..=b, t * 2.)
	} else {
		lerp((b * 2.)..=c, t)
	}
}

pub fn lerp (range : RangeInclusive<f32>, t : f32) -> f32 {
	(1. - t) * *range.start() + t * *range.end()
}
