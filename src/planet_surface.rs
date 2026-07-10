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
            let mut height_modifier= height_noise * amplitude;
            
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
                height_modifier += detail_noise as f32 * 1.3;
                let factor = (t-0.45) / (0.50 - 0.45);
                sand.lerp(grass, factor)
            } else if t < 0.75 {
                let factor = (t - 0.50) / (0.75 - 0.50);
                height_modifier += detail_noise as f32 * 1.5;
                grass.lerp(mountain, factor)
            } else {
                let factor = (t - 0.75) / (1.0 - 0.75);
                height_modifier += detail_noise.abs() as f32 * 3.0;
                mountain.lerp(snow, factor)
            };

            SurfacePoint { height_modifier, color }
        }
        PlanetType::Desert => {
            // Noise Wert:
            let frequenzy = planet_info.frequency as f64;
            let dune_noise = base_seed.get([
                frequenzy * dir.x as f64, frequenzy * dir.y as f64, frequenzy * dir.z as f64
            ]) as f32;

            let continent_noise = continent_seed.get([
                planet_info.continent_freq * dir.x as f64, planet_info.continent_freq * dir.y as f64, planet_info.continent_freq * dir.z as f64
            ]);

            let detail_noise = detail_seed.get([
                7. * dir.x as f64, 7. * dir.y as f64, 7. * dir.z as f64
            ]);

            let dune = (dune_noise * 5.0).sin();
            
            let continent = (continent_noise + 1.0) / 2.0;
            
            let detail = (detail_noise as f32 + 1.0) / 2.0;

            let amplitude = planet_info.amplitude / 2.0;
            let mut height_modifier = dune * amplitude;
            
            // Farben auf den Planeten basierend auf der Höhe:
            let base_sand = LinearRgba::new(0.9, 0.8, 0.6, 1.0);
            let dunes = LinearRgba::new(0.45, 0.58, 0.38, 1.0);
            let mountains = LinearRgba::new(0.75, 0.65, 0.45, 1.0);

            
            let t = (dune + 1.0) / 2.0; // Ändert den Noise Wert auf Bereich zwischen 0 und 1

            let color = if t < 0.5 {
                base_sand.lerp(dunes, t / 0.5)
            } else {
                let factor = (t - 0.5) / (1.0 - 0.5);
                height_modifier += detail_noise.sin() as f32 * 2.0;
                dunes.lerp(mountains, factor)
            };

            SurfacePoint { height_modifier, color }
        }
        PlanetType::Ice => {
            // Noise Wert:
            let frequenzy = planet_info.frequency as f64;
            let height_noise = base_seed.get([
                frequenzy * dir.x as f64, frequenzy * dir.y as f64, frequenzy * dir.z as f64
            ]) as f32;

            let temp_noise = continent_seed.get([
                planet_info.continent_freq * dir.x as f64, planet_info.continent_freq * dir.y as f64, planet_info.continent_freq * dir.z as f64
            ]);

            let detail_noise = detail_seed.get([
                7. * dir.x as f64, 7. * dir.y as f64, 7. * dir.z as f64
            ]);

            let glacier_ridge = 1.0 - height_noise.abs();            

            let mut height_modifier = 0.0;
            
            let detail = (detail_noise as f32 + 1.0) / 2.0;
            
            // Farben auf den Planeten basierend auf der Höhe:
            let eis_see = LinearRgba::new(0.05, 0.35, 0.55, 1.0);     // Tiefes, klares Seeblau/Cyan
            let gletscher_eis = LinearRgba::new(0.4, 0.7, 0.85, 1.0);  // Helleres, dickes Packeis
            let schnee_flach = LinearRgba::new(0.85, 0.9, 0.95, 1.0);  // Normale Schneefläche
            let fels_grau = LinearRgba::new(0.35, 0.38, 0.4, 1.0);    // Graue, felsige Bergwände
            let weisse_spitze = LinearRgba::new(0.98, 0.98, 1.0, 1.0); // Strahlend weiße Gipfel

            
            let t = (height_modifier + 1.0) / 2.0; // Ändert den Noise Wert auf Bereich zwischen 0 und 1
            let color_t = (t + detail_noise as f32 * 0.5).clamp(0.0, 1.0); // Leichte Farbvarriation mit detail noise

            
            let color = if color_t < 0.35 {
                height_modifier = 0.2 * planet_info.amplitude; // Gering, da flacher Eisse
                eis_see.lerp(gletscher_eis, t / 0.35)
            } else if color_t < 0.45 {
                let factor = (color_t - 0.35) / (0.45 - 0.25);
                
                height_modifier = t * planet_info.amplitude;
                
                gletscher_eis.lerp(schnee_flach, factor)
            } else if color_t < 0.75 {
                height_modifier = (color_t * planet_info.amplitude) + (detail_noise.abs() as f32 * 1.5);
                let factor = (color_t - 0.45) / (0.65 - 0.45);
                schnee_flach.lerp(fels_grau, factor)
            } else {
                let factor = (color_t - 0.75) / (1.0 - 0.75);
                height_modifier = (color_t * planet_info.amplitude) + (detail_noise.abs() as f32 * 2.5);
                fels_grau.lerp(weisse_spitze, factor)
            };

            SurfacePoint { height_modifier, color }
        }
        PlanetType::Lava => {
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

            let crust = (continent_noise.abs()).clamp(0.0, 1.);
            let mountains = (height_noise + 1.0) / 2.0;
            let detail = (detail_noise as f32 + 1.0) / 2.0;

            let amplitude = planet_info.amplitude;
            let mut height_modifier= height_noise * amplitude;
            
            // Farben auf den Planeten basierend auf der Höhe:
            let deep_lava = LinearRgba::new(2.5, 0.35, 0.0, 1.0);
            let dark_rock = LinearRgba::new(0.9, 0.05, 0.0, 1.0);
            let burnt_ground = LinearRgba::new(0.18, 0.04, 0.02, 1.0);
            let mountains = LinearRgba::new(0.32, 0.1, 0.08, 1.0);
            let red = LinearRgba::new(2.0, 0.0, 0.0, 1.0);

            
            let t = (height_noise + 1.0) / 2.0; // Ändert den Noise Wert auf Bereich zwischen 0 und 1

            let color = if t < 0.4 {
                deep_lava.lerp(dark_rock, t / 0.4)
            } else if t < 0.55 {
                let factor = (t - 0.4) / (0.55 - 0.4);
                dark_rock.lerp(burnt_ground, factor)
            } else if t < 0.75 {
                height_modifier += detail_noise.abs() as f32 * 1.3;
                let factor = (t-0.55) / (0.75 - 0.55);
                burnt_ground.lerp(mountains, factor)
            } else {
                let factor = (t - 0.75) / (1.0 - 0.75);
                height_modifier += detail_noise.abs() as f32 * 3.0;
                mountains.lerp(red, factor)
            };

            SurfacePoint { height_modifier, color }
        }
    }
}