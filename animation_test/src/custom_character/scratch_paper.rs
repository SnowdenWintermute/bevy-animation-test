// commands.spawn((
//     SceneBundle {
//         scene: gltf.scenes[0].clone(),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..Default::default()
//     },
//     PlayerCharacterName(String::from("Player1")),
// ));
//
//
// commands.spawn((
//     SceneBundle {
//         scene: gltf.scenes[0].clone(),
//         transform: Transform::from_xyz(2.0, 0.0, 0.0),
//         ..Default::default()
//     },
//     PlayerCharacterName(String::from("Player2")),
// ));
//
//
// fn run_animations(
//     animations: Res<Animations>,
//     mut animation_players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
// ) {
//     for mut animation_player in &mut animation_players {
//         animation_player
//             .play(animations.0[0].clone_weak())
//             .repeat()
//             .set_speed(0.5);
//     }
// }
//

#![allow(dead_code)]
mod animation_names;
use crate::{
    asset_loader::{AssetLoaderState, AssetPack},
    custom_character::animation_names::RpgAnimationNames,
};
use bevy::{gltf::Gltf, prelude::*};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

#[derive(Resource, Debug)]
struct Animations(Vec<Handle<AnimationClip>>);

pub struct CustomCharacterPlugin;
impl Plugin for CustomCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), (setup_character,))
            .add_systems(
                Update,
                run_animations.run_if(in_state(AssetLoaderState::Done)),
            );
    }
}

#[derive(Component, Debug)]
pub struct PlayerCharacterName(String);

#[derive(Component)]
struct AnimateThis(i32);

fn setup_character(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        println!("spawning scene");

        commands.spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: gltf.scenes[0].clone(),
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                if entity.contains::<AnimationPlayer>() {
                    println!("inserting player");
                    cmds.insert(AnimateThis(3));
                }
            }),
        });

        commands.spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform::from_xyz(2.0, 0., 0.),
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                if entity.contains::<AnimationPlayer>() {
                    println!("inserting player");
                    cmds.insert(AnimateThis(2));
                }
            }),
        });

        commands.insert_resource(Animations(vec![gltf.named_animations
            [RpgAnimationNames::Death.as_str()]
        .clone()]));
    }
}

#[derive(Event, Debug)]
struct AnimateIndex {
    index: i32,
}

fn run_animations(
    animations: Res<Animations>,
    mut query: Query<(&mut AnimationPlayer, &AnimateThis)>,
    // mut events: EventReader<AnimateIndex>,
) {
    for (mut animation_player, animate_this) in query.iter_mut() {
        println!("animation plyaer found");
        if animate_this.0 == 3 {
            animation_player.play(animations.0[0].clone_weak());
        }
    }
    // for to_animate in events.read() {
    //     println!("index: {:#?}", to_animate);
    //     let (mut to_animate, _) = query
    //         .iter_mut()
    //         .find(|(_, n)| n.0 == to_animate.index)
    //         .unwrap();
    //     to_animate.play(animations.0[0].clone_weak());
    // }
}
