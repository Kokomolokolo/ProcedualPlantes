use bevy::prelude::*;
use bevy::math::VectorSpace;
use noise::{NoiseFn, Perlin};
use bevy::render::mesh::{VertexAttributeValues};

use crate::planet_information::PlanetInfo;



pub fn gen_planet_mesh(
    planet_info: PlanetInfo
) -> Mesh {
    let mut planet_mesh = Mesh::from(Sphere::new(planet_info.radius).mesh().ico(planet_info.subdivs).unwrap());

    let perlin = Perlin::new(planet_info.seed);

    // Verticies aus dem Mesh holen
    if let Some(VertexAttributeValues::Float32x3(positions)) = 
        planet_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) 
    {
        // Die Positionswerte die Manipuliert werden. Sorry aber da ist ein clone
        let mut new_positions = positions.clone();
        let mut colors: Vec<[f32; 4]> = Vec::new();

        for vertex in new_positions.iter_mut() {
            let pos = Vec3::from_slice(vertex);
            
            let direction = pos.normalize();

            // Noise Wert:
            let frequenzy = planet_info.frequency as f64;
            let noise_value = perlin.get([
                frequenzy * direction.x as f64, frequenzy * direction.y as f64, frequenzy * direction.z as f64
            ]) as f32;

            
            let amplitude = planet_info.amplitude;
            let height_modifier= noise_value * amplitude;

            let final_pos = direction * (planet_info.radius + height_modifier);
            *vertex = final_pos.to_array();
            
            // Farben auf den Planeten basierend auf der Höhe:
            let deep_sea = LinearRgba::new(0.05, 0.15, 0.45, 1.0);
            let shallow_water = LinearRgba::new(0.1, 0.3, 0.65, 1.0);
            let sand = LinearRgba::new(0.85, 0.75, 0.55, 1.0);
            let grass = LinearRgba::new(0.15, 0.5, 0.2, 1.0);
            let mountain = LinearRgba::new(0.4, 0.35, 0.3, 1.0);
            let snow = LinearRgba::new(0.95, 0.95, 0.95, 1.0);
            
            let t = (noise_value + 1.0) / 2.0; // Ändert den Noise Wert auf Bereich zwischen 0 und 1

            let color = if t < 0.3 {
                deep_sea.lerp(shallow_water, t / 0.3)
            } else if t < 0.45 {
                let factor = (t - 0.3) / (0.45 - 0.3);
                shallow_water.lerp(sand, factor)
            } else if t < 0.65 {
                let factor = (t-0.45) / (0.65 - 0.45);
                sand.lerp(grass, factor)
            } else if t < 0.82 {
                let factor = (t - 0.65) / (0.82 - 0.65);
                grass.lerp(mountain, factor)
            } else {
                let factor = (t - 0.82) / (1.0 - 0.82);
                mountain.lerp(snow, factor)
            };

            colors.push(color.to_f32_array());

        }
        planet_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
        planet_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    
    planet_mesh
}





pub fn gen_earth_planet_mesh(
    planet_info: PlanetInfo
) -> Mesh {
    let mut planet_mesh = Mesh::from(Sphere::new(planet_info.radius).mesh().ico(planet_info.subdivs).unwrap());

    let perlin = Perlin::new(planet_info.seed);
    let continent_perlin = Perlin::new(planet_info.continent_seed);

    // Verticies aus dem Mesh holen
    if let Some(VertexAttributeValues::Float32x3(positions)) = 
        planet_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) 
    {
        // Die Positionswerte die Manipuliert werden. Sorry aber da ist ein clone
        let mut new_positions = positions.clone();
        let mut colors: Vec<[f32; 4]> = Vec::new();

        for vertex in new_positions.iter_mut() {
            let pos = Vec3::from_slice(vertex);
            
            let direction = pos.normalize();
            
            // Noise Wert:
            let frequenzy = planet_info.frequency as f64;
            let height_noise = perlin.get([
                frequenzy * direction.x as f64, frequenzy * direction.y as f64, frequenzy * direction.z as f64
            ]) as f32;

            let continent_value = continent_perlin.get([direction.x as f64, direction.y as f64, direction.z as f64]);

            
            let amplitude = planet_info.amplitude;
            let height_modifier= height_noise * amplitude;

            let final_pos = direction * (planet_info.radius + height_modifier);
            *vertex = final_pos.to_array();
            
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
            } else if t < 0.65 {
                let factor = (t-0.45) / (0.65 - 0.45);
                sand.lerp(grass, factor)
            } else if t < 0.82 {
                let factor = (t - 0.65) / (0.82 - 0.65);
                grass.lerp(mountain, factor)
            } else {
                let factor = (t - 0.82) / (1.0 - 0.82);
                mountain.lerp(snow, factor)
            };
            println!("{}", continent_value);
            if continent_value > 0.0{
                colors.push(deep_sea.to_f32_array());
            } else {
                colors.push(color.to_f32_array());
            }


        }
        planet_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
        planet_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    
    planet_mesh
}