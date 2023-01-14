use rsanim::CurrentState;

#[test]
fn progress_0() {
    let current_state = CurrentState {
        key: "idle".to_string(),
        duration: 0.5,
        elapsed: 0.0,
        repeat: true,
    };

    assert_eq!(current_state.progress(), 0.0);
}

#[test]
fn progress_0_5() {
    let current_state = CurrentState {
        key: "idle".to_string(),
        duration: 0.5,
        elapsed: 0.25,
        repeat: true,
    };

    assert_eq!(current_state.progress(), 0.5);
}

#[test]
fn progress_1() {
    let current_state = CurrentState {
        key: "idle".to_string(),
        duration: 0.5,
        elapsed: 0.5,
        repeat: true,
    };

    assert_eq!(current_state.progress(), 1.0);
}

#[test]
fn debug() {
    let current_state = CurrentState {
        key: "idle".to_string(),
        duration: 0.5,
        elapsed: 0.0,
        repeat: true,
    };

    assert_eq!(
        format!("{:?}", current_state),
        "CurrentState { key: \"idle\", duration: 0.5, elapsed: 0.0, repeat: true }"
    );
}
