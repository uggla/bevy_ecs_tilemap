use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_ecs_tilemap::prelude::*;

mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("rotate.tmx");

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tiled Map Editor Rotated Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(helpers::tiled::TiledMapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .run();
}
