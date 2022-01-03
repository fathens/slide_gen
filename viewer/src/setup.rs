use crate::put_cubes::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup.system());
    app.init_resource::<CubesResource>();
    app.add_system(generate_cubes.system());

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
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-8.0, 8.0, 10.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}
