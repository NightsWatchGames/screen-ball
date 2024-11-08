use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::{CursorOptions, WindowLevel};

#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;

mod area;
mod ball;
mod camera;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                transparent: true,
                decorations: false,
                window_level: WindowLevel::AlwaysOnTop,
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                position: WindowPosition::At(IVec2::new(0, 0)),
                cursor_options: CursorOptions {
                    hit_test: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_systems(
            Startup,
            (
                camera::setup_primary_window,
                camera::setup_camera,
                area::setup_area,
                ball::setup_ball,
            )
                .chain(),
        )
        .add_systems(Update, (ball::play_ball, area::update_wall))
        .run();
}
