use bevy::prelude::*;
use bevy::window::CompositeAlphaMode;
use bevy_inspector_egui::prelude::*;
use bevy_rapier3d::prelude::*;

mod ball;
mod camera;
mod util;
mod area;

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
                decorations: false,
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
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(camera::setup_camera)
        .add_startup_system(area::setup_area)
        .add_startup_system(ball::setup_ball)
        .add_system(ball::play_ball)
        .run();
}

// TODO 窗口不接收鼠标事件，将鼠标事件透给下面窗口
// fn window_init(mut windows: ResMut<Windows>) {
//     let window = windows.primary_mut();
//     window.set_cursor_grab_mode(false);
// }