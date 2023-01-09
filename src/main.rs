use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;
use bevy_inspector_egui::prelude::*;
use bevy_rapier3d::prelude::*;
use display_info::DisplayInfo;

mod util;

const WALL_HEIGHT_HALF_SIZE: f32 = 10.0;
const WALL_THICKNESS: f32 = 0.1;
// 相机在y轴高度
const CAMERA_HEIGHT_SIZE: f32 = 10.0;
// 球半径
const BALL_RADIUS: f32 = 0.5;

fn main() {
    // 计算屏幕大小和窗口位置
    let primary_display = util::primary_display();
    if primary_display.is_none() {
        panic!("There is no primary display");
    }
    let primary_display = primary_display.unwrap();

    App::new()
        // .insert_resource(ClearColor(Color::NONE))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                transparent: false,
                decorations: true,
                alpha_mode: CompositeAlphaMode::PostMultiplied, // work around, track issue https://github.com/bevyengine/bevy/issues/6330
                // always_on_top: true,
                position: WindowPosition::At(Vec2::new(
                    primary_display.x as f32,
                    primary_display.y as f32,
                )),
                monitor: MonitorSelection::Primary,
                width: primary_display.width as f32,
                height: primary_display.height as f32,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_area)
        .add_system(input_force)
        .add_system(play_ball)
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
        transform: Transform::from_xyz(0.0, CAMERA_HEIGHT_SIZE, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        // transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        // transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 计算屏幕大小和窗口位置
    let primary_display = util::primary_display();
    if primary_display.is_none() {
        panic!("There is no primary display");
    }
    let primary_display = primary_display.unwrap();

    // 区域大小跟相机高度有关，0.00092为手工测试出的比例
    let factor = CAMERA_HEIGHT_SIZE * 0.00092;
    let area_length = factor * primary_display.width as f32;
    let area_width = factor * primary_display.height as f32;
    println!("area: {} x {}", area_length, area_width);

    // 长方体地面
    commands
        .spawn(Collider::cuboid(area_length / 2.0, 0.1, area_width / 2.0))
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::ZERO,
        )));

    // 围墙（从Y向NEG_Y俯视）
    // 左围墙
    commands
        .spawn(Collider::cuboid(
            WALL_THICKNESS,
            WALL_HEIGHT_HALF_SIZE,
            area_width / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            0.0,
        )));
    // 右围墙
    commands
        .spawn(Collider::cuboid(
            WALL_THICKNESS,
            WALL_HEIGHT_HALF_SIZE,
            area_width / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            0.0,
        )));
    // 下围墙
    commands
        .spawn(Collider::cuboid(
            area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            WALL_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_HALF_SIZE,
            area_width / 2.0,
        )));
    // 上围墙
    commands
        .spawn(Collider::cuboid(
            area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            WALL_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_HALF_SIZE,
            -area_width / 2.0,
        )));

    let ball_handle: Handle<Mesh> = asset_server.load("models/football.gltf#Mesh0/Primitive0");
    // 球体
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(Collider::ball(BALL_RADIUS))
        // 恢复系数，影响碰撞后的反弹程度 https://en.wikipedia.org/wiki/Coefficient_of_restitution
        .insert(Restitution::coefficient(1.0))
        .insert(PbrBundle {
            // mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            mesh: ball_handle,
            // material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        })
        .insert((
            // 冲量，作用在物体上的力在时间上的累积 https://en.wikipedia.org/wiki/Impulse_(physics)
            ExternalImpulse {
                impulse: Vec3::new(0.0, 0.0, 0.0),
                torque_impulse: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            // 阻尼 https://en.wikipedia.org/wiki/Damping
            Damping {
                linear_damping: 0.2,
                angular_damping: 1.0,
            },
        ));
}

fn input_force(keys: Res<Input<KeyCode>>, mut q_ball: Query<&mut ExternalImpulse, With<Ball>>) {
    for mut external_impulse in &mut q_ball {
        if keys.just_pressed(KeyCode::Up) {
            external_impulse.impulse += Vec3::new(0.0, 0.0, -7.0);
        } else if keys.just_pressed(KeyCode::Down) {
            external_impulse.impulse += Vec3::new(0.0, 0.0, 7.0);
        } else if keys.just_pressed(KeyCode::Left) {
            external_impulse.impulse += Vec3::new(-7.0, 0.0, 0.0);
        } else if keys.just_pressed(KeyCode::Right) {
            external_impulse.impulse += Vec3::new(7.0, 0.0, 0.0);
            external_impulse.torque_impulse += Vec3::new(0.5, 0.3, 0.5);
        }
    }
}

pub fn play_ball(
    mut commands: Commands,
    mut q_ball: Query<(&mut ExternalImpulse, &Transform), With<Ball>>,
    mut motion_evr: EventReader<MouseMotion>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    // cursor_position原点在窗口左下角
    if let Some(cursor_position) = window.cursor_position() {
        let factor = CAMERA_HEIGHT_SIZE * 0.00092;
        let x = (cursor_position.x - window.width() / 2.0) * factor;
        let z = (window.height() / 2.0 - cursor_position.y) * factor;
        // 鼠标映射到3d世界的坐标（类比脚踢球）
        let cursor_3d_pos = Vec3::new(x, BALL_RADIUS, z);
        println!(
            "cursor_position: {}, cursor_3d_pos: {:?}",
            cursor_position, cursor_3d_pos,
        );
        for (mut external_impulse, ball_transform) in &mut q_ball {
            if cursor_3d_pos.distance(ball_transform.translation) <= BALL_RADIUS {
                println!("cursor hitted ball");
                // motion.delta.x 鼠标左滑为负、右滑为正
                // motion.delta.y 鼠标上滑为负、下滑为正
                for motion_ev in motion_evr.iter() {
                    // println!("Mouse moved: X: {} px, Y: {} px", motion_ev.delta.x, motion_ev.delta.y);
                    // TODO test
                    external_impulse.impulse +=
                        Vec3::new(0.1 * motion_ev.delta.x, 0.0, 0.1 * motion_ev.delta.y);
                    external_impulse.torque_impulse += Vec3::new(0.1, 0.1, 0.1);
                }
            }
        }
    }
}
