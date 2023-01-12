use crate::camera;
use bevy::prelude::*;
// 相机在y轴高度
pub const CAMERA_HEIGHT_SIZE: f32 = 10.0;

pub fn setup_camera(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 100.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, camera::CAMERA_HEIGHT_SIZE, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        // transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        // transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
