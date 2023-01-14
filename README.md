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
            trigger: Trigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
        },
        Transition {
            start_state: TransitionStartState::Node(Animation::Run),
            end_state: TransitionEndState::Node(Animation::Idle),
            trigger: Trigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
        },
    ],
    Params { speed: 0.0 },
)
.unwrap();
```

Update the state machine as time passes:

```rust
state_machine.update(delta_time);
```

Update the parameters that are used to determine conditional transitions:

```rust
state_machine.update_parameters(&|x| {
    x.speed = 1.0;
});
```

## Bevy

See `examples/bevy.rs`.
