use rsanim::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Animation {
    Idle,
}

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
    pub jump: bool,
}

#[test]
fn animator_new() {
    Animator::new(
        Animation::Idle,
        vec![(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )],
        vec![],
        Params {
            speed: 1.0,
            jump: false,
        },
        vec![(
            Animation::Idle,
            vec![
                Frame {
                    progress: 0.00,
                    index: 0,
                },
                Frame {
                    progress: 0.33,
                    index: 1,
                },
                Frame {
                    progress: 0.67,
                    index: 2,
                },
            ],
        )],
    )
    .unwrap();
}

#[test]
fn animator_new_empty_state_frames() {
    let err = Animator::new(
        Animation::Idle,
        vec![(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )],
        vec![],
        Params {
            speed: 1.0,
            jump: false,
        },
        vec![(Animation::Idle, vec![])],
    )
    .expect_err("empty state frames");

    assert_eq!(err, AnimatorError::EmptyStateFrames(Animation::Idle));
}

#[test]
fn animator_unsorted_state_frames() {
    let err = Animator::new(
        Animation::Idle,
        vec![(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )],
        vec![],
        Params {
            speed: 1.0,
            jump: false,
        },
        vec![(
            Animation::Idle,
            vec![
                Frame {
                    progress: 0.33,
                    index: 1,
                },
                Frame {
                    progress: 0.00,
                    index: 0,
                },
                Frame {
                    progress: 0.67,
                    index: 2,
                },
            ],
        )],
    )
    .expect_err("unsorted state frames");

    assert_eq!(err, AnimatorError::UnsortedStateFrames(Animation::Idle));
}

#[test]
fn animator_invalid_state_frame_progress() {
    let err = Animator::new(
        Animation::Idle,
        vec![(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )],
        vec![],
        Params {
            speed: 1.0,
            jump: false,
        },
        vec![(
            Animation::Idle,
            vec![Frame {
                progress: 1.33,
                index: 1,
            }],
        )],
    )
    .expect_err("invalid state frame progress");

    assert_eq!(
        err,
        AnimatorError::InvalidStateFrameProgress(Animation::Idle, 1.33)
    );
}

#[test]
fn animator_new_missing_state_frames() {
    let err = Animator::new(
        Animation::Idle,
        vec![(
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        )],
        vec![],
        Params {
            speed: 1.0,
            jump: false,
        },
        vec![],
    )
    .expect_err("missing state frames");

    assert_eq!(err, AnimatorError::EmptyStateFrames(Animation::Idle));
}
