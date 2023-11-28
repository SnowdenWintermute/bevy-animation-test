use bevy::prelude::*;

#[derive(Default, Resource, Debug)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Rock.glb#Scene0"),
        spaceship: asset_server.load("Spaceship-2.glb#Scene0"),
        missiles: asset_server.load("Pickup Jar.glb#Scene0"),
    }
}
