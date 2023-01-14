use rsanim::Trigger;

#[test]
fn clone() {
    let trigger = Trigger::<String>::End;
    assert!(matches!(trigger.clone(), Trigger::End));
}

#[test]
fn debug_end() {
    assert_eq!(format!("{:?}", Trigger::<String>::End), "End");
}

#[test]
fn debug_condition() {
    assert_eq!(
        format!("{:?}", Trigger::<String>::Condition(Box::new(|_| true))),
        "Condition"
    );
}
