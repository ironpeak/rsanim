use std::collections::HashMap;

pub struct StateMachine<T> {
    current_state: CurrentState,
    states: HashMap<String, State>,
    transitions: Vec<Transition<T>>,
    parameters: T,
}

pub struct CurrentState {
    pub name: String,
    pub elapsed: f32,
}

pub struct State {
    pub duration: f32,
}

pub enum StateNode {
    Any,
    Entry,
    End,
    Name(String),
}

pub struct Transition<T> {
    pub start_state: StateNode,
    pub end_state: StateNode,
    pub trigger: Trigger<T>,
}

pub enum Trigger<T> {
    Condition(Box<dyn Fn(&T) -> bool>),
}
