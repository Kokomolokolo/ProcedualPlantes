use bevy::{asset::RenderAssetUsages, prelude::*};
use rand::{Rng, rng};

use crate::{planet_information::PlanetInfo, planet_mesh::gen_planet_mesh, planet_types::PlanetType};

pub struct GalaxyPlugin;

impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_galaxy, setup_stars));
    }
}

pub fn setup_galaxy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let types = vec!(PlanetType::Earth, PlanetType::Lava, PlanetType::Desert, PlanetType::Ice);
    for i in 0..3 {
        for j in 0..3 {
            let seed = 42 + i;
    
            let planet_mesh = gen_planet_mesh(
                PlanetInfo {
                    seed,
                    planet_type: types[rng().random_range(0..4)],
                    subdivs: 50,
                    // radius: 50.,
                    ..default()
                }
            );
    
            commands.spawn((
                Mesh3d(meshes.add(planet_mesh)),
                MeshMaterial3d(materials.add(StandardMaterial {
                    perceptual_roughness: 0.8,
                    ..default()
                })),
                Transform::from_xyz(i as f32 * 100., 200.0, j as f32 * 100.),
            ));
        }
        let seed = 42 + i;

        let planet_mesh = gen_planet_mesh(
            PlanetInfo {
                seed,
                planet_type: types[i as usize],
                subdivs: 50,
                // radius: 50.,
                ..default()
            }
        );

        commands.spawn((
            Mesh3d(meshes.add(planet_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                perceptual_roughness: 0.8,
                ..default()
            })),
            Transform::from_xyz(i as f32 * 100., 0.0, 0.),
        ));
    }
    // Licht
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(40.0, 80.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    commands.spawn(AmbientLight { // Macht das überhaupt was?
        color: Color::WHITE,
        brightness: 1000.0,
        ..default()
    });
}

fn setup_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();

    let star_count = 3000;
    let mut postions = Vec::new();

    for _ in 0..star_count {
        // Zufällige Richtung im Kreis generieren
        let theta = rng.random_range(0.0..std::f32::consts::TAU);
        let phi = rng.random_range(0.0..std::f32::consts::PI);

        let radius = 5000.;

        let x = radius * phi.sin() * theta.cos();
        let y = radius * phi.sin() * theta.sin();
        let z = radius * phi.cos();

        postions.push(Vec3::new(x, y, z));
    }

    let mut mesh = Mesh::new(bevy::mesh::PrimitiveTopology::PointList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, postions);

    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::WHITE * 3.0,
        unlit: true,
        ..default()
    });

    commands.spawn(( 
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d( star_material )
    ));
    // Für dunkelen Himmel, sonst immer leicht grau
    commands.insert_resource(ClearColor(Color::BLACK));
}