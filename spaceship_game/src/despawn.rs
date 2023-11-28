use crate::spaceship::Spaceship;
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_distant_entities);
    }
}

fn despawn_distant_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform)>,
    // query: Query<(Entity, &GlobalTransform), Without<Spaceship>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
