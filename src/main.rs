use bevy::{prelude::*};

mod planet_mesh;
mod camera;
mod galaxy;
mod planet_information;
mod planet_types;
mod planet_surface;
mod startup_screen;

use crate::camera::CameraPlugin;
use crate::galaxy::GalaxyPlugin;
use crate::startup_screen::IntroPlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }));
    app.add_plugins((GalaxyPlugin, IntroPlugin, CameraPlugin));
    app.run();
}
