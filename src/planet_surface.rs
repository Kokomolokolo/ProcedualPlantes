use bevy::prelude::*;
use bevy::math::VectorSpace;
use noise::{NoiseFn, Perlin};

use crate::{planet_information::PlanetInfo, planet_types::PlanetType};

pub struct SurfacePoint {
    pub height_modifier: f32,
    pub color: LinearRgba,
}

pub fn calculate_surface(
    planet_type: PlanetType, dir: Vec3, 
    base_seed: &Perlin, continent_seed: &Perlin, detail_seed: &Perlin, 
    planet_info: &PlanetInfo
) -> SurfacePoint {
    // Surface generierung basierend auf dem Planetentyp
    match planet_type {
        PlanetType::Earth => {
            // Noise Wert:
            let frequenzy = planet_info.frequency as f64;
            let height_noise = base_seed.get([
                frequenzy * dir.x as f64, frequenzy * dir.y as f64, frequenzy * dir.z as f64
            ]) as f32;

            let continent_noise = continent_seed.get([
                planet_info.continent_freq * dir.x as f64, planet_info.continent_freq * dir.y as f64, planet_info.continent_freq * dir.z as f64
            ]);

            let detail_noise = detail_seed.get([
                7. * dir.x as f64, 7. * dir.y as f64, 7. * dir.z as f64
            ]);

            let continent = (continent_noise + 1.0) / 2.0;
            let mountains = (height_noise + 1.0) / 2.0;
            let detail = (detail_noise as f32 + 1.0) / 2.0;

            let amplitude = planet_info.amplitude;
            let height_modifier= height_noise * amplitude;
            
            // Farben auf den Planeten basierend auf der Höhe:
            let deep_sea = LinearRgba::new(0.05, 0.15, 0.45, 1.0);
            let shallow_water = LinearRgba::new(0.1, 0.3, 0.65, 1.0);
            let sand = LinearRgba::new(0.85, 0.75, 0.55, 1.0);
            let grass = LinearRgba::new(0.15, 0.5, 0.2, 1.0);
            let mountain = LinearRgba::new(0.4, 0.35, 0.3, 1.0);
            let snow = LinearRgba::new(0.95, 0.95, 0.95, 1.0);
            
            let t = (height_noise + 1.0) / 2.0; // Ändert den Noise Wert auf Bereich zwischen 0 und 1

            let color = if t < 0.3 {
                deep_sea.lerp(shallow_water, t / 0.3)
            } else if t < 0.45 {
                let factor = (t - 0.3) / (0.45 - 0.3);
                shallow_water.lerp(sand, factor)
            } else if t < 0.5 {
                let factor = (t-0.45) / (0.50 - 0.45);
                sand.lerp(grass, factor)
            } else if t < 0.75 {
                let factor = (t - 0.50) / (0.75 - 0.50);
                grass.lerp(mountain, factor)
            } else {
                let factor = (t - 0.75) / (1.0 - 0.75);
                mountain.lerp(snow, factor)
            };

            SurfacePoint { height_modifier, color }
        }
        PlanetType::Desert => {
            SurfacePoint { height_modifier: 1.0, color: LinearRgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 } }
        }
        PlanetType::Ice => {
            SurfacePoint { height_modifier: 1.0, color: LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 } }
        }
        PlanetType::Lava => {
            SurfacePoint { height_modifier: 1.0, color: LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 } }
        }
    }
}