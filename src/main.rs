use bevy::{prelude::*};

mod planet_mesh;
mod camera;
mod galaxy;
mod planet_information;

use crate::camera::CameraPlugin;
use crate::galaxy::GalaxyPlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins);
    app.add_plugins(GalaxyPlugin);
    app.add_plugins(CameraPlugin);
    app.run();
}
