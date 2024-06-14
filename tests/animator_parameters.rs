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

fn create_animator(starting_state: Animation, params: Params) -> Animator<Animation, Params> {
    Animator::new(
            starting_state,
            vec![(
                Animation::Idle,
                State {
                    duration: 0.5,
                    repeat: true,
                },
            )],
            vec![],
            params,
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
    .unwrap()
}

#[test]
fn animator_parameters() {
    let animator = create_animator(
        Animation::Idle,
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    assert_eq!(
        animator.parameters(),
        &Params {
            speed: 0.0,
            jump: false,
        }
    );
}

#[test]
fn animator_update_parameters() {
    let mut animator = create_animator(
        Animation::Idle,
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    animator.update_parameters(&|x| {
        x.speed = 1.0;
    });

    assert_eq!(
        animator.parameters(),
        &Params {
            speed: 1.0,
            jump: false,
        }
    );
}
