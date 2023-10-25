use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::close_on_esc};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plant::PlantPlugin;
use player::PlayerPlugin;

mod plant;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pixel Gardener".into(),
                        resolution: (1024.0, 768.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Grave)),
        )
        .add_plugins((PlayerPlugin, PlantPlugin))
        .add_systems(Update, close_on_esc)
        .run();
}
