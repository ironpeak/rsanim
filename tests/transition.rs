use rsanim::prelude::*;

#[test]
fn clone() {
    let transition = Transition {
        start_state: TransitionStartState::Any,
        end_state: TransitionEndState::Node("test".to_string()),
        trigger: TransitionTrigger::<String>::End,
    };

    assert_eq!(
        format!("{:?}", transition.clone()),
        format!("{:?}", transition)
    );
}

#[test]
fn debug() {
    let transition = Transition {
        start_state: TransitionStartState::Any,
        end_state: TransitionEndState::Node("test".to_string()),
        trigger: TransitionTrigger::<String>::End,
    };

    assert_eq!(
        format!("{:?}", transition),
        "Transition { start_state: Any, end_state: Node(\"test\"), trigger: End }"
    );
}
