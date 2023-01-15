use rsanim::TransitionTrigger;

#[test]
fn clone() {
    let trigger = TransitionTrigger::<String>::End;
    assert!(matches!(trigger.clone(), TransitionTrigger::End));
}

#[test]
fn debug_end() {
    assert_eq!(format!("{:?}", TransitionTrigger::<String>::End), "End");
}

#[test]
fn debug_condition() {
    assert_eq!(
        format!(
            "{:?}",
            TransitionTrigger::<String>::Condition(Box::new(|_| true))
        ),
        "Condition"
    );
}
