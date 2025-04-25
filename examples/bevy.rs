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
    pub anim: Animator<PlayerAnimState, PlayerAnimParams, Handle<Image>>,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            viewport_origin: Vec2::new(0.5, 0.5),
            ..OrthographicProjection::default_2d()
        }),
    ));

    let state_machine = StateMachine::new(
        PlayerAnimState::Red,
        HashMap::from([
            (
                PlayerAnimState::Red,
                rsanim::State {
                    duration: 0.5,
                    repeat: false,
                },
            ),
            (
                PlayerAnimState::Green,
                rsanim::State {
                    duration: 0.5,
                    repeat: false,
                },
            ),
            (
                PlayerAnimState::Blue,
                rsanim::State {
                    duration: 0.5,
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
    .unwrap();

    commands.spawn((
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        Sprite {
            image: asset_server.load("red_0.png"),
            ..default()
        },
        Player {
            anim: Animator::new(
                state_machine,
                HashMap::from([
                    (
                        PlayerAnimState::Red,
                        vec![
                            Frame {
                                value: asset_server.load("red_0.png"),
                                progress: 0.00,
                            },
                            Frame {
                                value: asset_server.load("red_1.png"),
                                progress: 0.33,
                            },
                            Frame {
                                value: asset_server.load("red_2.png"),
                                progress: 0.67,
                            },
                        ],
                    ),
                    (
                        PlayerAnimState::Green,
                        vec![
                            Frame {
                                value: asset_server.load("green_0.png"),
                                progress: 0.00,
                            },
                            Frame {
                                value: asset_server.load("green_1.png"),
                                progress: 0.33,
                            },
                            Frame {
                                value: asset_server.load("green_2.png"),
                                progress: 0.67,
                            },
                        ],
                    ),
                    (
                        PlayerAnimState::Blue,
                        vec![
                            Frame {
                                value: asset_server.load("blue_0.png"),
                                progress: 0.00,
                            },
                            Frame {
                                value: asset_server.load("blue_1.png"),
                                progress: 0.33,
                            },
                            Frame {
                                value: asset_server.load("blue_2.png"),
                                progress: 0.67,
                            },
                        ],
                    ),
                ]),
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

    let frame = player.anim.frame();

    sprite.image = frame.clone();
}
