use rsanim::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
    pub jump: bool,
}

#[test]
fn animator_new() {
    Animator::new(
        StateMachine::new(
            "idle".to_string(),
            HashMap::from([(
                "idle".to_string(),
                State {
                    duration: 0.5,
                    repeat: true,
                },
            )]),
            vec![],
            Params {
                speed: 1.0,
                jump: false,
            },
        )
        .unwrap(),
        HashMap::from([(
            "idle".to_string(),
            vec![
                Frame {
                    progress: 0.00,
                    value: 0,
                },
                Frame {
                    progress: 0.33,
                    value: 1,
                },
                Frame {
                    progress: 0.67,
                    value: 2,
                },
            ],
        )]),
    )
    .unwrap();
}

#[test]
fn animator_new_empty_state_frames() {
    let err = Animator::<String, Params, u8>::new(
        StateMachine::new(
            "idle".to_string(),
            HashMap::from([(
                "idle".to_string(),
                State {
                    duration: 0.5,
                    repeat: true,
                },
            )]),
            vec![],
            Params {
                speed: 1.0,
                jump: false,
            },
        )
        .unwrap(),
        HashMap::from([("idle".to_string(), vec![])]),
    )
    .expect_err("empty state frames");

    assert_eq!(err, AnimatorError::EmptyStateFrames("idle".to_string()));
}

#[test]
fn animator_unsorted_state_frames() {
    let err = Animator::<String, Params, u8>::new(
        StateMachine::new(
            "idle".to_string(),
            HashMap::from([(
                "idle".to_string(),
                State {
                    duration: 0.5,
                    repeat: true,
                },
            )]),
            vec![],
            Params {
                speed: 1.0,
                jump: false,
            },
        )
        .unwrap(),
        HashMap::from([(
            "idle".to_string(),
            vec![
                Frame {
                    progress: 0.33,
                    value: 1,
                },
                Frame {
                    progress: 0.00,
                    value: 0,
                },
                Frame {
                    progress: 0.67,
                    value: 2,
                },
            ],
        )]),
    )
    .expect_err("unsorted state frames");

    assert_eq!(err, AnimatorError::UnsortedStateFrames("idle".to_string()));
}

#[test]
fn animator_invalid_state_frame_progress() {
    let err = Animator::<String, Params, u8>::new(
        StateMachine::new(
            "idle".to_string(),
            HashMap::from([(
                "idle".to_string(),
                State {
                    duration: 0.5,
                    repeat: true,
                },
            )]),
            vec![],
            Params {
                speed: 1.0,
                jump: false,
            },
        )
        .unwrap(),
        HashMap::from([(
            "idle".to_string(),
            vec![Frame {
                progress: 1.33,
                value: 1,
            }],
        )]),
    )
    .expect_err("invalid state frame progress");

    assert_eq!(
        err,
        AnimatorError::InvalidStateFrameProgress("idle".to_string(), 1.33)
    );
}

#[test]
fn animator_new_missing_state_frames() {
    let err = Animator::<String, Params, u8>::new(
        StateMachine::new(
            "idle".to_string(),
            HashMap::from([(
                "idle".to_string(),
                State {
                    duration: 0.5,
                    repeat: true,
                },
            )]),
            vec![],
            Params {
                speed: 1.0,
                jump: false,
            },
        )
        .unwrap(),
        HashMap::new(),
    )
    .expect_err("missing state frames");

    assert_eq!(err, AnimatorError::MissingStateFrames("idle".to_string()));
}
