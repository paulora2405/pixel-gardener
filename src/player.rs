use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use rand::{thread_rng, Rng};
use std::time::Duration;

const PLAYER_TEXTURE_FILES: &[&str] = &[
    "characters/mani-idle-run.png",
    "characters/gabe-idle-run.png",
];
const ANIMATION_DURATION_SECONDS: f32 = 0.1;
const DIAGONAL_SPEED_MULTIPLIER: f32 = 0.707;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

#[derive(Component, Default)]
pub struct PlayerMarker;

#[derive(Component, Default, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Speed(#[inspector(min = 0.0)] f32);

#[derive(Component, Clone, Default)]
pub enum LookingDirection {
    #[default]
    Right,
    Left,
}

#[derive(Component, Default, Clone)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
}

#[derive(Component, Default)]
pub struct PlayerAnimation {
    idle_idx: usize,
    run_first_idx: usize,
    run_last_idx: usize,
    timer: Timer,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub state: PlayerState,
    pub looking_direction: LookingDirection,
    pub speed: Speed,
    pub sprite_bundle: SpriteSheetBundle,
    pub animation: PlayerAnimation,
    /// Used to name the type in WorldInspector
    name: Name,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Money(100.0))
            .add_systems(Startup, setup_player)
            .add_systems(Update, character_movement)
            .add_systems(Update, character_animation.after(character_movement))
            .register_type::<Money>();
    }
}

impl PlayerBundle {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        player_animation: PlayerAnimation,
    ) -> Self {
        Self {
            name: Name::new("PlayerEntity"),
            speed: Speed(100.0),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(player_animation.idle_idx),
                ..default()
            },
            animation: player_animation,
            ..default()
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::ANTIQUE_WHITE),
        },
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };
    commands.spawn(camera);

    let mut rng = thread_rng();
    let texture_index: usize = rng.gen_range(0..PLAYER_TEXTURE_FILES.len());
    let texture_handle = asset_server.load(PLAYER_TEXTURE_FILES[texture_index]);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_animation = PlayerAnimation {
        idle_idx: 0,
        run_first_idx: 1,
        run_last_idx: 6,
        timer: Timer::from_seconds(ANIMATION_DURATION_SECONDS, TimerMode::Repeating),
    };

    let player = PlayerBundle::new(texture_atlas_handle, player_animation);

    commands.spawn(player);
    info!("Player was setup!");
}

pub fn character_animation(
    time: Res<Time>,
    mut player_q: Query<(
        &mut PlayerAnimation,
        &PlayerState,
        &LookingDirection,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut animation, state, looking_dir, mut sprite) in &mut player_q {
        match state {
            PlayerState::Idle => {
                animation.timer.pause();
                animation.timer.reset();
                animation
                    .timer
                    .set_elapsed(Duration::from_secs_f32(ANIMATION_DURATION_SECONDS));
                sprite.index = animation.idle_idx;
            }
            PlayerState::Running => {
                animation.timer.unpause();
                animation.timer.tick(time.delta());
                if animation.timer.just_finished() {
                    if sprite.index == animation.idle_idx || sprite.index == animation.run_last_idx
                    {
                        sprite.index = animation.run_first_idx;
                    } else {
                        sprite.index += 1;
                    };
                }
                match looking_dir {
                    LookingDirection::Right => sprite.flip_x = false,
                    LookingDirection::Left => sprite.flip_x = true,
                };
            }
        };
    }
}

pub fn character_movement(
    mut player_q: Query<(
        &mut Transform,
        &Speed,
        &mut PlayerState,
        &mut LookingDirection,
    )>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, player_speed, mut state, mut looking_dir) = player_q.single_mut();
    let movement_amout = player_speed.0 * time.delta_seconds();

    let mut v_speed = match (input.pressed(KeyCode::S), input.pressed(KeyCode::W)) {
        (true, false) => -movement_amout,
        (false, true) => movement_amout,
        _ => 0.0,
    };
    let mut h_speed = match (input.pressed(KeyCode::A), input.pressed(KeyCode::D)) {
        (true, false) => {
            *looking_dir = LookingDirection::Left;
            -movement_amout
        }
        (false, true) => {
            *looking_dir = LookingDirection::Right;
            movement_amout
        }
        _ => 0.0,
    };

    *state = PlayerState::Running;
    if v_speed != 0.0 && h_speed != 0.0 {
        v_speed *= DIAGONAL_SPEED_MULTIPLIER;
        h_speed *= DIAGONAL_SPEED_MULTIPLIER;
    } else if v_speed == 0.0 && h_speed == 0.0 {
        *state = PlayerState::Idle;
    }

    transform.translation.x += h_speed;
    transform.translation.y += v_speed;
}
