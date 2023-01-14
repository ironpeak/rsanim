use rsanim::{State, StateMachine};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
    pub jump: bool,
}

fn create_sm(starting_state: String, params: Params) -> StateMachine<String, Params> {
    StateMachine::new(
        starting_state,
        HashMap::from([(
            "idle".to_string(),
            State {
                duration: 0.5,
                repeat: true,
            },
        )]),
        vec![],
        params,
    )
    .unwrap()
}

#[test]
fn sm_parameters() {
    let sm = create_sm(
        "idle".to_string(),
        Params {
            speed: 0.0,
            jump: false,
        },
    );

    assert_eq!(
        sm.parameters(),
        &Params {
            speed: 0.0,
            jump: false,
        }
    );
}

#[test]
fn sm_update_parameters() {
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
        sm.parameters(),
        &Params {
            speed: 1.0,
            jump: false,
        }
    );
}
