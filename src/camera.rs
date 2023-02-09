use crate::{camera, util};
use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{CompositeAlphaMode, CreateWindow, WindowId},
};
use display_info::DisplayInfo;
// 相机在y轴高度
pub const CAMERA_HEIGHT_SIZE: f32 = 10.0;

pub fn setup_camera(mut commands: Commands, mut create_window_events: EventWriter<CreateWindow>) {
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, camera::CAMERA_HEIGHT_SIZE, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        // transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        // transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let primary_display = util::primary_display().unwrap();

    // TODO 根据display信息创建新窗口
    let non_primary_displays = util::non_primary_displays();
    let mut non_primary_displays_mock: Vec<DisplayInfo> = Vec::new();
    // non_primary_displays_mock.push(DisplayInfo {
    //     id: 2,
    //     x: 20,
    //     y: 600,
    //     width: 800,
    //     height: 600,
    //     rotation: 1.0,
    //     scale_factor: 1.0,
    //     is_primary: false,
    // });
    for display in non_primary_displays_mock {
        let window_id = WindowId::new();
        create_window_events.send(CreateWindow {
            id: window_id,
            descriptor: WindowDescriptor {
                title: "Second window".to_string(),
                transparent: true,
                decorations: true,
                alpha_mode: CompositeAlphaMode::PostMultiplied, // work around, track issue https://github.com/bevyengine/bevy/issues/6330
                width: display.width as f32,
                height: display.height as f32,
                position: WindowPosition::At(Vec2::new(display.x as f32, display.y as f32)),
                ..default()
            },
        });
        // 相机坐标
        let camera_x = -(primary_display.width as f32 / 2.0) + display.x as f32;
        let camera_z = -(primary_display.height as f32 / 2.0) + display.y as f32;
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(camera_x, camera::CAMERA_HEIGHT_SIZE, camera_z)
                .looking_at(Vec3::new(camera_x, 0.0, camera_z), Vec3::NEG_Z),
            camera: Camera {
                target: RenderTarget::Window(window_id),
                ..default()
            },
            ..default()
        });
    }
}
