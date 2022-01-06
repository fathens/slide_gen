use crate::put_cubes::*;
use crate::resources::*;
use crate::rotate::*;
use bevy::prelude::*;

pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.init_resource::<CubesResource>();
    app.add_startup_system(setup.system());
    app.add_startup_system(generate_cubes.system());
    app.add_system(spawn_camera.system());
    app.add_system(pan_orbit_camera.system());

    app.run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(800.0, 800.0);
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(-10.0, -20.0, -10.0)),
        ..Default::default()
    });
}
