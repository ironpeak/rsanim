use rsanim::prelude::*;

#[test]
fn clone() {
    let transition = TransitionEndState::Node("test".to_string());

    assert_eq!(transition.clone(), transition);
}

#[test]
fn debug() {
    let transition = TransitionEndState::Node("test".to_string());

    assert_eq!(format!("{:?}", transition), "Node(\"test\")");
}
