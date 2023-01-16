use std::collections::HashMap;

use rsanim::{
    Animator, Frame, State, StateMachine, Transition, TransitionEndState, TransitionStartState,
    TransitionTrigger,
};

use bevy::{
    prelude::*,
    render::camera::{ScalingMode, WindowOrigin},
    window::{self, PresentMode},
};

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
            window: WindowDescriptor {
                title: "rsanim".to_string(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                width: 800.0,
                height: 640.0,
                ..default()
            },
            ..default()
        }))
        // setup
        .add_startup_system(setup)
        // player
        .add_system(player_update)
        .add_system_set(
            SystemSet::new()
                .with_system(player_render)
                .after(player_update),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_system(window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            window_origin: WindowOrigin::Center,
            ..Default::default()
        },
        ..Default::default()
    });

    let state_machine = StateMachine::new(
        PlayerAnimState::Red,
        HashMap::from([
            (
                PlayerAnimState::Red,
                State {
                    duration: 0.5,
                    repeat: false,
                },
            ),
            (
                PlayerAnimState::Green,
                State {
                    duration: 0.5,
                    repeat: false,
                },
            ),
            (
                PlayerAnimState::Blue,
                State {
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
        SpriteBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            texture: asset_server.load("red_0.png"),
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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();

    player.anim.update(time.delta_seconds());

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        transform.translation =
            transform.translation + direction.normalize() * 128.0 * time.delta_seconds();
    }
}

pub fn player_render(mut query: Query<(&Player, &mut Handle<Image>)>) {
    let (player, mut image) = query.single_mut();

    let frame = player.anim.frame();

    *image = frame.clone();
}
