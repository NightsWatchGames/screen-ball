use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;
use bevy_inspector_egui::prelude::*;
use bevy_rapier3d::prelude::*;

const AREA_LENGTH_HALF_SIZE: f32 = 50.0;
const AREA_WIDTH_HALF_SIZE: f32 = 30.0;
const WALL_HEIGHT_HALF_SIZE: f32 = 10.0;
const WALL_THICKNESS: f32 = 0.1;

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
        .run();
}

// fn window_init(mut windows: ResMut<Windows>) {
//     let window = windows.primary_mut();
//     window.set_cursor_hittest(false);
// }

#[derive(Debug, Component)]
pub struct Ball;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        // transform: Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
        transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 长方体地面
    commands
        .spawn(Collider::cuboid(
            AREA_LENGTH_HALF_SIZE,
            0.1,
            AREA_WIDTH_HALF_SIZE,
        ))
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::ZERO,
        )));

    // 围墙（从Y向NEG_Y俯视）
    // 左围墙
    commands
        .spawn(Collider::cuboid(
            WALL_THICKNESS,
            WALL_HEIGHT_HALF_SIZE,
            AREA_WIDTH_HALF_SIZE,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -AREA_LENGTH_HALF_SIZE,
            WALL_HEIGHT_HALF_SIZE,
            0.0,
        )));
    // 右围墙
    commands
        .spawn(Collider::cuboid(
            WALL_THICKNESS,
            WALL_HEIGHT_HALF_SIZE,
            AREA_WIDTH_HALF_SIZE,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            AREA_LENGTH_HALF_SIZE,
            WALL_HEIGHT_HALF_SIZE,
            0.0,
        )));
    // 下围墙
    commands
        .spawn(Collider::cuboid(
            AREA_LENGTH_HALF_SIZE,
            WALL_HEIGHT_HALF_SIZE,
            WALL_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_HALF_SIZE,
            AREA_WIDTH_HALF_SIZE,
        )));
    // 上围墙
    commands
        .spawn(Collider::cuboid(
            AREA_LENGTH_HALF_SIZE,
            WALL_HEIGHT_HALF_SIZE,
            WALL_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_HALF_SIZE,
            -AREA_WIDTH_HALF_SIZE,
        )));

    let ball_handle: Handle<Mesh> = asset_server.load("models/football.gltf#Mesh0/Primitive0");
    // 球体
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(Collider::ball(0.5))
        // 恢复系数，影响碰撞后的反弹程度 https://en.wikipedia.org/wiki/Coefficient_of_restitution
        .insert(Restitution::coefficient(1.0))
        .insert(PbrBundle {
            // mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            mesh: ball_handle,
            // material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        });
}

fn input_force(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    q_ball: Query<Entity, With<Ball>>,
) {
    for entity in &q_ball {
        if keys.just_pressed(KeyCode::Up) {
            // TODO 可以改在setup中创建，这里直接更新component
            commands.entity(entity).insert((
                // 冲量，作用在物体上的力在时间上的累积 https://en.wikipedia.org/wiki/Impulse_(physics)
                ExternalImpulse {
                    impulse: Vec3::new(0.0, 0.0, -7.0),
                    ..default()
                },
                // 阻尼 https://en.wikipedia.org/wiki/Damping
                Damping {
                    linear_damping: 0.2,
                    angular_damping: 1.0,
                },
            ));
        } else if keys.just_pressed(KeyCode::Down) {
            commands.entity(entity).insert((
                // 冲量，作用在物体上的力在时间上的累积 https://en.wikipedia.org/wiki/Impulse_(physics)
                ExternalImpulse {
                    impulse: Vec3::new(0.0, 0.0, 7.0),
                    ..default()
                },
                // 阻尼 https://en.wikipedia.org/wiki/Damping
                Damping {
                    linear_damping: 0.2,
                    angular_damping: 1.0,
                },
            ));
        } else if keys.just_pressed(KeyCode::Left) {
            commands.entity(entity).insert((
                // 冲量，作用在物体上的力在时间上的累积 https://en.wikipedia.org/wiki/Impulse_(physics)
                ExternalImpulse {
                    impulse: Vec3::new(-7.0, 0.0, 0.0),
                    ..default()
                },
                // 阻尼 https://en.wikipedia.org/wiki/Damping
                Damping {
                    linear_damping: 0.2,
                    angular_damping: 1.0,
                },
            ));
        } else if keys.just_pressed(KeyCode::Right) {
            commands.entity(entity).insert((
                // 冲量，作用在物体上的力在时间上的累积 https://en.wikipedia.org/wiki/Impulse_(physics)
                ExternalImpulse {
                    impulse: Vec3::new(7.0, 0.0, 0.0),
                    torque_impulse: Vec3::new(0.5, 0.3, 0.5),
                    ..default()
                },
                // 阻尼 https://en.wikipedia.org/wiki/Damping
                Damping {
                    linear_damping: 0.2,
                    angular_damping: 1.0,
                },
            ));
        }
    }
}
