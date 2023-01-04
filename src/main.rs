use std::time::Duration;

use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::prelude::*;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::NONE))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                transparent: false,
                decorations: true,
                alpha_mode: CompositeAlphaMode::PostMultiplied, // work around, track issue https://github.com/bevyengine/bevy/issues/6330
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .add_system(input_force)
        .add_system(remove_force)
        .run();
}

// fn window_init(mut windows: ResMut<Windows>) {
//     let window = windows.primary_mut();
//     window.set_cursor_hittest(false);
// }

#[derive(Debug, Component)]
pub struct Ball;

#[derive(Debug, Component)]
pub struct ForceTimer(Timer);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 长方体地面
    commands
        .spawn(Collider::cuboid(10.0, 0.1, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    // 围墙
    commands
        .spawn(Collider::cuboid(1.0, 10.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(-10.0, -2.0, 0.0)));
    commands
        .spawn(Collider::cuboid(1.0, 10.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(10.0, -2.0, 0.0)));
    commands
        .spawn(Collider::cuboid(10.0, 10.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 10.0)));
    commands
        .spawn(Collider::cuboid(10.0, 10.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, -10.0)));

    // 球体
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        });
}

fn input_force(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    q_ball: Query<Entity, With<Ball>>,
) {
    if keys.just_pressed(KeyCode::Up) {
        commands
            .entity(q_ball.single())
            .insert(ExternalForce {
                force: Vec3::new(0.0, 0.0, -5.0),
                ..default() // torque: Vec3::new(1.0, 2.0, 3.0),
            })
            .insert(ForceTimer(Timer::new(
                Duration::from_millis(200),
                TimerMode::Once,
            )));
    } else if keys.just_pressed(KeyCode::Down) {
        commands
            .entity(q_ball.single())
            .insert(ExternalForce {
                force: Vec3::new(0.0, 0.0, 5.0),
                // torque: Vec3::new(1.0, 2.0, 3.0),
                ..default()
            })
            .insert(ForceTimer(Timer::new(
                Duration::from_millis(200),
                TimerMode::Once,
            )));
    } else if keys.just_pressed(KeyCode::Left) {
        commands
            .entity(q_ball.single())
            .insert(ExternalForce {
                force: Vec3::new(-5.0, 0.0, 0.0),
                torque: Vec3::new(0.1, 0.1, 0.1),
                ..default()
            })
            .insert(ForceTimer(Timer::new(
                Duration::from_millis(20),
                TimerMode::Once,
            )));
    } else if keys.just_pressed(KeyCode::Right) {
        commands
            .entity(q_ball.single())
            .insert(ExternalForce {
                force: Vec3::new(5.0, 0.0, 0.0),
                // torque: Vec3::new(1.0, 2.0, 3.0),
                ..default()
            })
            .insert(ForceTimer(Timer::new(
                Duration::from_millis(20),
                TimerMode::Once,
            )));
    }
}

pub fn remove_force(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut ForceTimer), With<Ball>>,
    time: Res<Time>,
) {
    for (entity, mut force_timer) in &mut q_ball {
        force_timer.0.tick(time.delta());
        if force_timer.0.finished() {
            commands
                .entity(entity)
                .remove::<ExternalForce>()
                .remove::<ForceTimer>();
        }
    }
}