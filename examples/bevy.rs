use std::collections::HashMap;

use rsanim::{
    State, StateMachine, Transition, TransitionEndState, TransitionStartState, TransitionTrigger,
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
    pub anim: StateMachine<PlayerAnimState, PlayerAnimParams>,
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            window_origin: WindowOrigin::Center,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                scale: Vec3::new(16.0, 16.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            ..default()
        },
        Player {
            anim: StateMachine::new(
                PlayerAnimState::Red,
                HashMap::from([
                    (
                        PlayerAnimState::Red,
                        State {
                            duration: 1.0,
                            repeat: false,
                        },
                    ),
                    (
                        PlayerAnimState::Green,
                        State {
                            duration: 1.0,
                            repeat: false,
                        },
                    ),
                    (
                        PlayerAnimState::Blue,
                        State {
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

pub fn player_render(mut query: Query<(&Player, &mut Sprite)>) {
    let (player, mut transform) = query.single_mut();

    let color_intensity = 1.0 - player.anim.state().progress() / 2.0;

    match player.anim.state().key {
        PlayerAnimState::Red => {
            transform.color = Color::rgb(color_intensity, 0.0, 0.0);
        }
        PlayerAnimState::Green => {
            transform.color = Color::rgb(0.0, color_intensity, 0.0);
        }
        PlayerAnimState::Blue => {
            transform.color = Color::rgb(0.0, 0.0, color_intensity);
        }
    }
}
