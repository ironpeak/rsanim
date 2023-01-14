use rsanim::StateMachineError;

#[test]
fn clone() {
    let error = StateMachineError::InvalidStartingState("test".to_string());

    assert_eq!(format!("{:?}", error.clone()), format!("{:?}", error));
}

#[test]
fn debug() {
    assert_eq!(
        format!(
            "{:?}",
            StateMachineError::InvalidStartingState("test".to_string())
        ),
        "InvalidStartingState(\"test\")"
    );
}
