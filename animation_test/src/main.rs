mod asset_loader;
mod camera;
mod custom_character;
mod gltf_master_asset;
mod level;
use asset_loader::{AssetLoaderPlugin, AssetLoaderState, AssetPack};
use bevy::{gltf::Gltf, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use camera::CameraPlugin;
use custom_character::CustomCharacterPlugin;
// use gltf_master_asset::ClericCharacterPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use level::PlanePlugin;

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// enum AppState {
//     Loading,
//     InGame,
//     Paused,
// }

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
        // .add_plugins(ClericCharacterPlugin)
        // DOWNLOADED
        .add_plugins(PanOrbitCameraPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}

// fn setup_character_master_gltf_style(
//     mut commands: Commands,
//     asset_pack: Res<AssetPack>,
//     assets_gltf: Res<Assets<Gltf>>,
// ) {
//     println!("checking for gltf");
//     if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
//         println!("spawning scene");
//         commands.spawn(SceneBundle {
//             // scene: gltf.named_scenes["Scene"].clone(),
//             // scene: gltf.scenes[0].clone(),
//             scene: gltf.named_scenes["Scene"].clone(),
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             ..Default::default()
//         });
//     }
// }
