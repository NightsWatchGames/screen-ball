use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::camera;

// 球半径
const BALL_RADIUS: f32 = 1.0;

#[derive(Debug, Component)]
pub struct Ball;

pub fn setup_ball(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
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

pub fn play_ball(
    mut q_ball: Query<(&mut ExternalImpulse, &Transform), With<Ball>>,
    mut motion_evr: EventReader<MouseMotion>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    // cursor_position原点在窗口左下角
    if let Some(cursor_position) = window.cursor_position() {
        let factor = camera::CAMERA_HEIGHT_SIZE * 0.00092;
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
                // motion.delta.x 鼠标左滑为负、右滑为正，motion.delta.y 鼠标上滑为负、下滑为正
                for motion_ev in motion_evr.iter() {
                    // println!("Mouse moved: X: {} px, Y: {} px", motion_ev.delta.x, motion_ev.delta.y);
                    // TODO 根据鼠标移动速度、移动方向与球夹角 判断冲量和转矩冲量大小
                    external_impulse.impulse =
                        Vec3::new(0.5 * motion_ev.delta.x, 0.0, 0.5 * motion_ev.delta.y);
                    // external_impulse.torque_impulse = Vec3::new(0.1, 0.1, 0.1);
                }
            }
        }
    }
}
