use rsanim::{State, StateMachine, StateMachineError, Transition, Trigger};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Animation {
    Idle,
    Walk,
}

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
    pub jump: bool,
}

#[test]
fn sm_new() {
    StateMachine::new(
        Animation::Idle,
        HashMap::from([(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )]),
        vec![],
        Params {
            speed: 0.0,
            jump: false,
        },
    )
    .unwrap();
}

#[test]
fn sm_new_invalid_starting_state() {
    let err = StateMachine::new(
        Animation::Walk,
        HashMap::from([(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )]),
        vec![],
        Params {
            speed: 0.0,
            jump: false,
        },
    )
    .expect_err("invalid starting state");

    assert_eq!(
        err,
        StateMachineError::InvalidStartingState(Animation::Walk)
    );
}

#[test]
fn sm_new_invalid_transition_start_state() {
    let err = StateMachine::new(
        Animation::Idle,
        HashMap::from([(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )]),
        vec![Transition {
            start_state: rsanim::TransitionStartState::Node(Animation::Walk),
            end_state: rsanim::TransitionEndState::Node(Animation::Idle),
            trigger: Trigger::End,
        }],
        Params {
            speed: 0.0,
            jump: false,
        },
    )
    .expect_err("invalid transition start state");

    assert_eq!(
        err,
        StateMachineError::InvalidTransitionStartState(Animation::Walk)
    );
}

#[test]
fn sm_new_invalid_transition_end_state() {
    let err = StateMachine::new(
        Animation::Idle,
        HashMap::from([(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )]),
        vec![Transition {
            start_state: rsanim::TransitionStartState::Node(Animation::Idle),
            end_state: rsanim::TransitionEndState::Node(Animation::Walk),
            trigger: Trigger::End,
        }],
        Params {
            speed: 0.0,
            jump: false,
        },
    )
    .expect_err("invalid transition end state");

    assert_eq!(
        err,
        StateMachineError::InvalidTransitionEndState(Animation::Walk)
    );
}
