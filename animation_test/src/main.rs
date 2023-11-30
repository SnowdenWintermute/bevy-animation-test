mod asset_loader;
mod camera;
mod custom_character;
mod level;
use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use camera::CameraPlugin;
use custom_character::CustomCharacterPlugin;
use level::PlanePlugin;

fn main() {
    App::new()
        // BEVY
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.95,
        })
        .add_plugins(DefaultPlugins)
        // CUSTOM
        .add_plugins(CameraPlugin)
        .add_plugins(PlanePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CustomCharacterPlugin)
        // .add_systems(
        //     OnEnter(AssetLoaderState::Done),
        //     setup_character_master_gltf_style,
        // )
        // DOWNLOADED
        .add_plugins(PanOrbitCameraPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}
