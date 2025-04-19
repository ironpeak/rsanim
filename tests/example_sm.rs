use rsanim::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
    pub jump: bool,
}

fn create_sm(starting_state: String, params: Params) -> StateMachine<String, Params> {
    StateMachine::new(
        starting_state,
        HashMap::from([
            (
                "idle".to_string(),
                State {
                    duration: 0.5,
                    repeat: true,
                },
            ),
            (
                "walk".to_string(),
                State {
                    duration: 1.0,
                    repeat: true,
                },
            ),
            (
                "jump".to_string(),
                State {
                    duration: 0.25,
                    repeat: false,
                },
            ),
        ]),
        vec![
            Transition {
                start_state: TransitionStartState::Node("idle".to_string()),
                end_state: TransitionEndState::Node("walk".to_string()),
                trigger: TransitionTrigger::Condition(Box::new(|x: &Params| {
                    x.speed > 0.0 && !x.jump
                })),
            },
            Transition {
                start_state: TransitionStartState::Node("walk".to_string()),
                end_state: TransitionEndState::Node("idle".to_string()),
                trigger: TransitionTrigger::Condition(Box::new(|x: &Params| {
                    x.speed <= 0.0 && !x.jump
                })),
            },
            Transition {
                start_state: TransitionStartState::Any,
                end_state: TransitionEndState::Node("jump".to_string()),
                trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.jump)),
            },
            Transition {
                start_state: TransitionStartState::Node("jump".to_string()),
                end_state: TransitionEndState::Node("walk".to_string()),
                trigger: TransitionTrigger::End,
            },
        ],
        params,
    )
    .unwrap()
}

#[test]
fn starts_in_starting_state() {
    let sm = create_sm(
        "idle".to_string(),
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "idle".to_string(),
            duration: 0.5,
            elapsed: 0.0,
            repeat: true,
        }
    );
}

#[test]
fn idle_repeats() {
    let mut sm = create_sm(
        "idle".to_string(),
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    assert_eq!(sm.state().progress(), 0.0);

    sm.update(0.25);
    assert_eq!(sm.state().progress(), 0.5);

    sm.update(0.20);
    assert_eq!(sm.state().progress(), 0.9);

    sm.update(0.05);
    assert_eq!(sm.state().progress(), 0.0);
}

#[test]
fn walk_repeats() {
    let mut sm = create_sm(
        "walk".to_string(),
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    assert_eq!(sm.state().progress(), 0.0);

    sm.update(0.5);
    assert_eq!(sm.state().progress(), 0.5);

    sm.update(0.4);
    assert_eq!(sm.state().progress(), 0.9);

    sm.update(0.1);
    assert_eq!(sm.state().progress(), 0.0);
}

#[test]
fn transition_idle_to_walk() {
    let mut sm = create_sm(
        "idle".to_string(),
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    sm.update_parameters(&|x| {
        x.speed = 1.0;
    });

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "walk".to_string(),
            duration: 1.0,
            elapsed: 0.0,
            repeat: true,
        }
    );
}

#[test]
fn transition_idle_to_jump() {
    let mut sm = create_sm(
        "idle".to_string(),
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    sm.update_parameters(&|x| {
        x.jump = true;
    });

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "jump".to_string(),
            duration: 0.25,
            elapsed: 0.0,
            repeat: false,
        }
    );
}

#[test]
fn transition_walk_to_idle() {
    let mut sm = create_sm(
        "walk".to_string(),
        Params {
            speed: 1.0,
            jump: false,
        },
    );

    sm.update_parameters(&|x| {
        x.speed = 0.0;
    });

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "idle".to_string(),
            duration: 0.5,
            elapsed: 0.0,
            repeat: true,
        }
    );
}

#[test]
fn transition_walk_to_jump() {
    let mut sm = create_sm(
        "walk".to_string(),
        Params {
            speed: 1.0,
            jump: false,
        },
    );

    sm.update_parameters(&|x| {
        x.jump = true;
    });

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "jump".to_string(),
            duration: 0.25,
            elapsed: 0.0,
            repeat: false,
        }
    );
}

#[test]
fn transition_end_jump_to_idle() {
    let mut sm = create_sm(
        "jump".to_string(),
        Params {
            speed: 1.0,
            jump: true,
        },
    );

    sm.update_parameters(&|x| {
        x.jump = false;
    });

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "jump".to_string(),
            duration: 0.25,
            elapsed: 0.0,
            repeat: false,
        }
    );

    sm.update(0.25);

    assert_eq!(
        sm.state(),
        &CurrentState {
            key: "walk".to_string(),
            duration: 1.0,
            elapsed: 0.0,
            repeat: true,
        }
    );
}
