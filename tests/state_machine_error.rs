use rsanim::StateMachineError;

#[test]
fn clone() {
    assert_eq!(
        StateMachineError::InvalidStartingState("test".to_string()).clone(),
        StateMachineError::InvalidStartingState("test".to_string())
    );
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
