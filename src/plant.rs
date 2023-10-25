use crate::player::{Money, PlayerMarker};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const PLANTS_TEXTURE_FILES: &[&str] = &["plants/plant1.png", "plants/plant2.png"];

#[derive(Component)]
pub struct PlantMarker;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlantAge(Timer);

#[derive(Bundle)]
pub struct PlantBundle {
    name: Name,
    pub marker: PlantMarker,
    pub sprite_bundle: SpriteBundle,
    pub age: PlantAge,
}

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_plant, plant_lifecycle))
            .register_type::<PlantAge>();
    }
}

impl PlantBundle {
    pub fn new(texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            name: Name::new("Plant"),
            marker: PlantMarker,
            sprite_bundle: SpriteBundle {
                sprite: Sprite::default(),
                texture,
                transform,
                ..default()
            },
            age: PlantAge(Timer::from_seconds(10.0, TimerMode::Once)),
        }
    }
}

pub fn spawn_plant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<PlayerMarker>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        let mut rng = thread_rng();
        let texture_index: usize = rng.gen_range(0..PLANTS_TEXTURE_FILES.len());

        money.0 -= 10.0;
        info!("Spent $10 on a Plant, remaining money: ${}", money.0);
        info!(
            "Using plant texture {}",
            PLANTS_TEXTURE_FILES[texture_index]
        );

        let texture = asset_server.load(PLANTS_TEXTURE_FILES[texture_index]);

        commands.spawn(PlantBundle::new(texture, *player_transform));
    }
}

pub fn plant_lifecycle(
    mut commands: Commands,
    mut money: ResMut<Money>,
    time: Res<Time>,
    mut plants: Query<(Entity, &mut PlantAge)>,
) {
    let money_received = 15.0;
    for (plant_entity, mut plant_age) in &mut plants {
        plant_age.0.tick(time.delta());

        if plant_age.0.finished() {
            money.0 += money_received;

            commands.entity(plant_entity).despawn();

            info!("Plant Died. In compensation, player received ${money_received}, current money: ${}", money.0);
        }
    }
}
