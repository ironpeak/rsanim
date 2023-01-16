# rsanim

A basic state machine for managing sprite animations.

Example usage:

```rust
use rsanim::*;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Animation {
    Idle,
    Run,
}

#[derive(Clone, Debug, PartialEq)]
struct Params {
    pub speed: f32,
}

let mut state_machine = StateMachine::new(
    Animation::Idle,
    HashMap::from([
        (
            Animation::Idle,
            State {
                duration: 0.5,
                repeat: true,
            },
        ),
        (
            Animation::Run,
            State {
                duration: 1.0,
                repeat: true,
            },
        ),
    ]),
    vec![
        Transition {
            start_state: TransitionStartState::Node(Animation::Idle),
            end_state: TransitionEndState::Node(Animation::Run),
            trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
        },
        Transition {
            start_state: TransitionStartState::Node(Animation::Run),
            end_state: TransitionEndState::Node(Animation::Idle),
            trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
        },
    ],
    Params { speed: 0.0 },
)
.unwrap();

let animator = Animator::new(
    state_machine,
    HashMap::from([
        (
            Animation::Idle,
            vec![
                Frame {
                    value: 0,
                    progress: 0.00,
                },
                Frame {
                    value: 1,
                    progress: 0.33,
                },
                Frame {
                    value: 2,
                    progress: 0.67,
                },
            ],
        ),
        (
            Animation::Run,
            vec![
                Frame {
                    value: 0,
                    progress: 0.00,
                },
                Frame {
                    value: 1,
                    progress: 0.33,
                },
                Frame {
                    value: 2,
                    progress: 0.67,
                },
            ],
        ),
    ]),
)
.unwrap();
```

Update the state machine's elapsed time:

```rust
let delta_time = 0.1;
animator.update(delta_time);
```

Update the state machine's parameters that are used to determine conditional transitions:

```rust
animator.update_parameters(&|x| {
    x.speed = 1.0;
});
```

## Bevy

See `examples/bevy.rs`.
