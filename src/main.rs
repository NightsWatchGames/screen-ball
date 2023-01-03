use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                transparent: true,
                decorations: false,
                alpha_mode: CompositeAlphaMode::PostMultiplied,  // work around, track issue https://github.com/bevyengine/bevy/issues/6330
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn window_init(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_cursor_hittest(false);
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..default()
    });
}