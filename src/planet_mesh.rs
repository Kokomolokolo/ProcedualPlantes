use bevy::prelude::*;
use noise::Perlin;
use bevy::render::mesh::{VertexAttributeValues};

use crate::{planet_information::PlanetInfo, planet_surface::calculate_surface};



pub fn gen_planet_mesh(
    planet_info: PlanetInfo
) -> Mesh {
    let mut planet_mesh = Mesh::from(Sphere::new(planet_info.radius).mesh().ico(planet_info.subdivs).unwrap());

    let base_seed = Perlin::new(planet_info.seed);
    let continent_seed = Perlin::new(planet_info.continent_seed);
    let detail_seed = Perlin::new(planet_info.detail_seed);

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

            let surface = calculate_surface(
                planet_info.planet_type, direction, &base_seed, &continent_seed, &detail_seed, &planet_info
            );

            let final_pos = direction * (planet_info.radius + surface.height_modifier);
            *vertex = final_pos.to_array();

            colors.push(surface.color.to_f32_array());

        }
        planet_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
        planet_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    
    planet_mesh
}