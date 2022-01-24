use crate::move_cubes;
use crate::put_cubes;
use crate::resources::*;
use crate::rotate;
use bevy::prelude::*;

pub fn run() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.init_resource::<CubesResource>();
    app.add_startup_system(setup);
    app.add_startup_system(rotate::setup);
    app.add_startup_system(put_cubes::setup);
    app.add_startup_system(move_cubes::setup);

    app.add_system(rotate::action);
    app.add_system(move_cubes::action);

    app.run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(800.0, 800.0);
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 4.0, 3.0)),
        ..Default::default()
    });
}
