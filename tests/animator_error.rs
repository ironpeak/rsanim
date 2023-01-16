use rsanim::AnimatorError;

#[test]
fn clone() {
    let error = AnimatorError::EmptyStateFrames("test".to_string());

    assert_eq!(format!("{:?}", error.clone()), format!("{:?}", error));
}

#[test]
fn debug() {
    assert_eq!(
        format!("{:?}", AnimatorError::EmptyStateFrames("test".to_string())),
        "EmptyStateFrames(\"test\")"
    );
}
