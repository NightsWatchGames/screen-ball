use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;
use bevy_inspector_egui::prelude::*;
use bevy_rapier3d::prelude::*;

mod ball;
mod camera;
mod util;

const WALL_HEIGHT_HALF_SIZE: f32 = 10.0;
const WALL_THICKNESS: f32 = 0.1;

fn main() {
    // 计算屏幕大小和窗口位置
    let primary_display = util::primary_display();
    if primary_display.is_none() {
        panic!("There is no primary display");
    }
    let primary_display = primary_display.unwrap();

    App::new()
        .insert_resource(ClearColor(Color::NONE))
        // .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                transparent: true,
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
        .add_startup_system(camera::setup_camera)
        .add_startup_system(setup_area)
        .add_startup_system(ball::setup_ball)
        .add_system(ball::play_ball)
        .run();
}

// TODO 窗口不接收鼠标事件，将鼠标事件透给下面窗口
// fn window_init(mut windows: ResMut<Windows>) {
//     let window = windows.primary_mut();
//     window.set_cursor_grab_mode(false);
// }

fn setup_area(
    mut commands: Commands,
) {
    // 计算屏幕大小和窗口位置
    let primary_display = util::primary_display();
    if primary_display.is_none() {
        panic!("There is no primary display");
    }
    let primary_display = primary_display.unwrap();

    // 区域大小跟相机高度有关，0.00092为手工测试出的比例
    let factor = camera::CAMERA_HEIGHT_SIZE * 0.00092;
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

}
