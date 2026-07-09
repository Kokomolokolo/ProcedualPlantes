use bevy::prelude::*;
use crate::planet_types::PlanetType;

pub struct PlanetInfo {
    pub seed: u32,
    pub radius: f32,
    pub subdivs: u32,

    pub planet_type: PlanetType,
    // Die Topologieparameter
    pub amplitude: f32, // von 0-8
    pub frequency: f32, // von 0-4

    // Die verschiedenen Seed, abgeleitet von dem Grundseed
    pub continent_seed: u32,
    pub detail_seed: u32,
    pub continent_freq: f64,
    
}

impl PlanetInfo {
    fn new(seed: u32, radius: f32, subdivs: u32, planet_type: PlanetType, amplitude: f32, frequency: f32, continent_freq: f64) -> Self {
        Self {
            seed,
            radius,
            subdivs,
            planet_type,
            amplitude,
            frequency,
            continent_seed: seed + 42,
            detail_seed: seed + 84,
            continent_freq,
        }
    }
}

impl Default for PlanetInfo {
    fn default() -> Self {
        Self {
            seed: 42,
            radius: 20.,
            subdivs: 20,
            planet_type: PlanetType::Earth,
            amplitude: 3.0,
            frequency: 3.0,
            continent_seed: 84,
            detail_seed: 42 + 84,
            continent_freq: 0.5,
        }
    }
}