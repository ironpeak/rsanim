use rsanim::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Animation {
    Idle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Params {
    pub speed: f32,
    pub jump: bool,
}

#[test]
fn clone() {
    let animator = Animator::new(
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

    assert_eq!(format!("{:?}", animator.clone()), format!("{:?}", animator));
}

#[test]
fn debug() {
    let animator = Animator::new(
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

    assert_eq!(format!("{:?}", animator), "Animator { state_machine: StateMachine { current_state: SMCurrentState { index: 0, key: Idle, duration: 0.5, elapsed: 0.0, repeat: true }, states: [SMState { key: Idle, duration: 0.5, repeat: true }], transitions: [], parameters: Params { speed: 1.0, jump: false } }, state_frames: [[Frame { progress: 0.0, index: 0 }, Frame { progress: 0.33, index: 1 }, Frame { progress: 0.67, index: 2 }]] }");
}
