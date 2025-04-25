use rsanim::prelude::*;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerAnimState {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerAnimParams {}

#[derive(Component)]
pub struct Player {
    pub anim: StateMachine<PlayerAnimState, PlayerAnimParams>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                title: "Island Crawlers".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // setup
        .add_systems(Startup, setup)
        // player
        .add_systems(Update, player_update)
        .add_systems(Update, player_render.after(player_update))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            viewport_origin: Vec2::new(0.5, 0.5),
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(16.0, 16.0, 1.0),
            ..default()
        },
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        },
        Player {
            anim: StateMachine::new(
                PlayerAnimState::Red,
                HashMap::from([
                    (
                        PlayerAnimState::Red,
                        rsanim::State {
                            duration: 1.0,
                            repeat: false,
                        },
                    ),
                    (
                        PlayerAnimState::Green,
                        rsanim::State {
                            duration: 1.0,
                            repeat: false,
                        },
                    ),
                    (
                        PlayerAnimState::Blue,
                        rsanim::State {
                            duration: 1.0,
                            repeat: false,
                        },
                    ),
                ]),
                vec![
                    Transition {
                        start_state: TransitionStartState::Node(PlayerAnimState::Red),
                        end_state: TransitionEndState::Node(PlayerAnimState::Green),
                        trigger: TransitionTrigger::End,
                    },
                    Transition {
                        start_state: TransitionStartState::Node(PlayerAnimState::Green),
                        end_state: TransitionEndState::Node(PlayerAnimState::Blue),
                        trigger: TransitionTrigger::End,
                    },
                    Transition {
                        start_state: TransitionStartState::Node(PlayerAnimState::Blue),
                        end_state: TransitionEndState::Node(PlayerAnimState::Red),
                        trigger: TransitionTrigger::End,
                    },
                ],
                PlayerAnimParams {},
            )
            .unwrap(),
        },
    ));
}

pub fn player_update(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let Ok((mut player, mut transform)) = query.single_mut() else {
        return;
    };

    player.anim.update(time.delta_secs());

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        transform.translation =
            transform.translation + direction.normalize() * 128.0 * time.delta_secs();
    }
}

pub fn player_render(mut query: Query<(&Player, &mut Sprite)>) {
    let Ok((player, mut sprite)) = query.single_mut() else {
        return;
    };

    let color_intensity = 1.0 - player.anim.state().progress() / 2.0;

    match player.anim.state().key {
        PlayerAnimState::Red => {
            sprite.color = Color::srgb(color_intensity, 0.0, 0.0);
        }
        PlayerAnimState::Green => {
            sprite.color = Color::srgb(0.0, color_intensity, 0.0);
        }
        PlayerAnimState::Blue => {
            sprite.color = Color::srgb(0.0, 0.0, color_intensity);
        }
    }
}
