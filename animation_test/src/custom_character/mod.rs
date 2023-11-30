#![allow(dead_code)]
mod animation_names;
mod link_animations;
use crate::{
    asset_loader::{AssetLoaderState, AssetPack},
    custom_character::animation_names::RpgAnimationNames,
};
use bevy::{gltf::Gltf, prelude::*};

use self::link_animations::{link_animations, AnimationEntityLink};

#[derive(Resource, Debug)]
struct Animations(Vec<Handle<AnimationClip>>);

pub struct CustomCharacterPlugin;
impl Plugin for CustomCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AssetLoaderState::Done),
            (setup_character_master_gltf_style,),
        )
        .add_systems(
            Update,
            (
                run_animations.run_if(in_state(AssetLoaderState::Done)),
                link_animations.run_if(in_state(AssetLoaderState::Done)),
                list_scene_entities.run_if(in_state(AssetLoaderState::Done)),
            ),
        );
    }
}

#[derive(Component, Debug)]
pub struct PlayerCharacterName(String);

fn setup_character_master_gltf_style(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    println!("checking for gltf");
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        println!("spawning scene");
        let scene_entity_1 = commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            PlayerCharacterName("Player 1".to_string()),
        ));
        println!("spawned entity: {:#?}", &scene_entity_1.id());
        println!("spawning scene");
        let scene_entity_2 = commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform::from_xyz(2.0, 0.0, 0.0),
                ..Default::default()
            },
            PlayerCharacterName("Player 2".to_string()),
        ));
        println!("spawned entity: {:#?}", &scene_entity_2.id());
        // let name_entity = commands.spawn(PlayerCharacterName(String::from("Player1")));
        // println!("spawned name entity: {:#?}", &name_entity.id());
        // scene_entity.add_child(name_entity);

        commands.insert_resource(Animations(
            vec![gltf.named_animations[RpgAnimationNames::Death.as_str()].clone()],
            // vec![gltf.named_animations["Idle"].clone()],
        ));
    }
}

// fn run_animations(
//     animations: Res<Animations>,
//     mut query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
// ) {
//     for (entity, mut animation_player) in &mut query {
//         println!("playing animation for {:#?}", entity);
//         animation_player.play(animations.0[0].clone_weak()).repeat();
//     }
// }
fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut player_character_query: Query<
        (&PlayerCharacterName, &AnimationEntityLink),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
) {
    for (name, animation_entity_link) in player_character_query.iter_mut() {
        if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity_link.0) {
            if name.0 == "Player 1".to_string() {
                animation_player.play(animations.0[0].clone_weak()).repeat();
            }
        }
    }
}

fn list_scene_entities(
    query: Query<(Entity, &PlayerCharacterName, &AnimationEntityLink), Added<AnimationEntityLink>>,
) {
    for (entity, character_name, animation_entity_link) in query.iter() {
        println!(
            "entity {:#?} has player_character_name of {} and player {:#?}",
            entity, character_name.0, animation_entity_link
        );
    }
}
