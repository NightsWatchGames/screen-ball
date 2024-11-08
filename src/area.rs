use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::CAMERA_HEIGHT_SIZE;

const WALL_HEIGHT_SIZE: f32 = 20.0;
const WALL_THICKNESS: f32 = 0.1;

#[derive(Component)]
pub struct Wall(pub String);

pub fn setup_area(mut commands: Commands) {
    // TODO 多窗口支持：如果两个区域的墙存在重叠，则此区域是连通的
    let projection = PerspectiveProjection::default();
    let area_height = projection.fov.tan() * CAMERA_HEIGHT_SIZE;
    let area_width = area_height * projection.aspect_ratio;
    println!("Setup area: {} x {}", area_width, area_width);

    // 长方体地面
    commands.spawn((
        Collider::cuboid(100000.0, 0.1, 100000.0),
        RigidBody::Static,
        Transform::from_translation(Vec3::ZERO),
    ));

    // 围墙（从Y向NEG_Y俯视）
    // 左围墙
    commands.spawn((
        Wall("left".to_string()),
        Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, area_height),
        RigidBody::Static,
        Transform::from_xyz(-area_width / 2.0, WALL_HEIGHT_SIZE / 2.0, 0.0),
    ));
    // 右围墙
    commands.spawn((
        Wall("right".to_string()),
        Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, area_height),
        RigidBody::Static,
        Transform::from_xyz(area_width / 2.0, WALL_HEIGHT_SIZE / 2.0, 0.0),
    ));
    // 上围墙
    commands.spawn((
        Wall("up".to_string()),
        Collider::cuboid(area_width, WALL_HEIGHT_SIZE, WALL_THICKNESS),
        RigidBody::Static,
        Transform::from_xyz(0.0, WALL_HEIGHT_SIZE / 2.0, -area_height / 2.0),
    ));
    // 下围墙
    commands.spawn((
        Wall("down".to_string()),
        Collider::cuboid(area_width, WALL_HEIGHT_SIZE, WALL_THICKNESS),
        RigidBody::Static,
        Transform::from_xyz(0.0, WALL_HEIGHT_SIZE / 2.0, area_height / 2.0),
    ));
}

pub fn update_wall(
    q_camera: Query<&Projection>,
    mut q_wall: Query<(&mut Collider, &mut Transform, &Wall)>,
    mut height: Local<f32>,
    mut width: Local<f32>,
) {
    let proj = q_camera.single();
    let (fov, aspect_ratio) = match proj {
        Projection::Perspective(per) => (per.fov, per.aspect_ratio),
        Projection::Orthographic(_) => unreachable!(),
    };

    let new_height = fov.tan() * CAMERA_HEIGHT_SIZE;
    let new_width = new_height * aspect_ratio;

    if new_height - *height > 0.001 || new_width - *width > 0.001 {
        println!(
            "Area changed from {} x {} to {} x {}",
            *width, *height, new_width, new_height
        );

        for (mut collider, mut transform, wall) in q_wall.iter_mut() {
            match wall.0.as_str() {
                "left" => {
                    *collider = Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, new_height);
                    transform.translation.x = -new_width / 2.0;
                }
                "right" => {
                    *collider = Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, new_height);
                    transform.translation.x = new_width / 2.0;
                }
                "up" => {
                    *collider = Collider::cuboid(new_width, WALL_HEIGHT_SIZE, WALL_THICKNESS);
                    transform.translation.z = -new_height / 2.0;
                }
                "down" => {
                    *collider = Collider::cuboid(new_width, WALL_HEIGHT_SIZE, WALL_THICKNESS);
                    transform.translation.z = new_height / 2.0;
                }
                _ => unreachable!(),
            }
        }

        *width = new_width;
        *height = new_height;
    }
}
