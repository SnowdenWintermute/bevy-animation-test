use bevy::gltf::Gltf;
use bevy::prelude::*;

pub struct ClericCharacterPlugin;
impl Plugin for ClericCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_gltf)
            .add_systems(Startup, spawn_gltf_objects);
    }
}

/// Helper resource for tracking our asset
#[derive(Resource)]
struct ClericAssetPack(Handle<Gltf>);

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("Cleric.glb");
    commands.insert_resource(ClericAssetPack(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<ClericAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });
    }
}
