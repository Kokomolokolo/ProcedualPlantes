use bevy::prelude::*;

pub struct PlanetInfo {
    pub seed: u32,
    pub radius: f32,
    pub subdivs: u32,

    // Die Topologieparameter
    pub amplitude: f32, // von 0-8
    pub frequency: f32, // von 0-4

    // Die verschiedenen Seed, abgeleitet von dem Grundseed
    pub continent_seed: u32,
}

impl PlanetInfo {
    fn new(seed: u32, radius: f32, subdivs: u32, amplitude: f32, frequency: f32) -> Self {
        Self {
            seed,
            radius,
            subdivs,
            amplitude,
            frequency,
            continent_seed: seed + 42
        }
    }
}

impl Default for PlanetInfo {
    fn default() -> Self {
        Self {
            seed: 42,
            radius: 20.,
            subdivs: 18,
            amplitude: 3.0,
            frequency: 3.0,
            continent_seed: 84,
        }
    }
}