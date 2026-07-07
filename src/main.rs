use bevy::math::VectorSpace;
use bevy::{prelude::*};
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};

mod planet_mesh;
mod camera;
mod galaxy;

use crate::camera::CameraPlugin;
use crate::galaxy::GalaxyPlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins);
    app.add_plugins(GalaxyPlugin);
    app.add_plugins(CameraPlugin);
    app.run();
}
