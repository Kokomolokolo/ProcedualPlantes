use bevy::prelude::*;

use crate::planet_mesh::gen_planet_mesh;

pub struct GalaxyPlugin;

impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_galaxy);
    }
}


pub fn setup_galaxy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    for i in 0..5 {
        let seed = 42 + i;

        let planet_mesh = gen_planet_mesh(20.0, 7, seed);

        commands.spawn((
            Mesh3d(meshes.add(planet_mesh)),
            MeshMaterial3d(materials.add(
                StandardMaterial {
                    perceptual_roughness: 0.8,
                    ..default()
                }
            )),
            Transform::from_xyz(i as f32 * 100., 0.0, 0.0)
        ));
        
    }
    // Licht
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(40.0, 80.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}