use rsanim::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Animation {
    Idle,
}

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
    pub jump: bool,
}

#[test]
fn clone() {
    let sm = StateMachine::new(
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

    assert_eq!(format!("{:?}", sm.clone()), format!("{:?}", sm));
}

#[test]
fn debug() {
    let sm = StateMachine::new(
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

    assert_eq!(format!("{:?}", sm), "StateMachine { current_state: CurrentState { key: Idle, duration: 0.5, elapsed: 0.0, repeat: true }, states: {Idle: State { duration: 0.5, repeat: true }}, transitions: [], parameters: Params { speed: 0.0, jump: false } }");
}
