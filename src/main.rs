use crate::app::*;
use crate::game_parameters::*;
use bevy::input::InputPlugin;
use bevy::prelude::*;
mod app;
mod game_parameters;
mod player;

fn main() {
    let mut app = create_app(create_default_game_parameters());
    let add_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fun);

    assert_eq!(false, app.is_plugin_added::<InputPlugin>());
    app.add_plugins(DefaultPlugins);
    assert!(app.is_plugin_added::<InputPlugin>());

    app.run();
}
