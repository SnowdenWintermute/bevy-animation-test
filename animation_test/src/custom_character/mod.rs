use crate::asset_loader::{AssetLoaderState, AssetPack};
use bevy::{gltf::Gltf, prelude::*};

#[derive(Resource, Debug)]
struct Animations(Vec<Handle<AnimationClip>>);

pub struct CustomCharacterPlugin;
impl Plugin for CustomCharacterPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup_character)
        app.add_systems(
            OnEnter(AssetLoaderState::Done),
            (setup_character_master_gltf_style,),
        )
        .add_systems(
            Update,
            run_animations.run_if(in_state(AssetLoaderState::Done)),
        );
    }
}

fn setup_character_master_gltf_style(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    println!("checking for gltf");
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        println!("spawning scene");
        commands.spawn(SceneBundle {
            // scene: gltf.named_scenes["Scene"].clone(),
            scene: gltf.scenes[0].clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });

        commands.insert_resource(Animations(
            vec![gltf.named_animations["Idle"].clone()],
            // vec![asset_server.load("animation_only.gltf#Animation0")],
        ));
    }
}

fn run_animations(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    println!("{:#?}", animations.0);
    for mut animation_player in &mut animation_players {
        animation_player.play(animations.0[0].clone_weak()).repeat();
    }
}

// fn setup_character(asset_server: Res<AssetServer>, mut commands: Commands) {
//     commands.insert_resource(Animations(
//         vec![asset_server.load("Cleric.gltf#Animation0")],
//         // vec![asset_server.load("animation_only.gltf#Animation0")],
//     ));

//     commands.spawn(SceneBundle {
//         scene: asset_server.load("Monk.gltf#Scene0"),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..Default::default()
//     });

//     commands.spawn(SceneBundle {
//         scene: asset_server.load("Cleric.gltf#Scene0"),
//         transform: Transform::from_xyz(2.0, 00.0, 0.0),
//         ..Default::default()
//     });
//     // commands.spawn(SceneBundle {
//     //     scene: asset_server.load("witch_feet.glb#Scene0"),
//     //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//     //     ..Default::default()
//     // });
//     // commands.spawn(SceneBundle {
//     //     scene: asset_server.load("witch_legs.glb#Scene0"),
//     //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//     //     ..Default::default()
//     // });
//     // commands.spawn(SceneBundle {
//     //     scene: asset_server.load("scifi_body.glb#Scene0"),
//     //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//     //     ..Default::default()
//     // });
// }
