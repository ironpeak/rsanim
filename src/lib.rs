use std::{collections::HashMap, rc::Rc};

pub struct StateMachine<T> {
    current_state: CurrentState,
    states: HashMap<String, State>,
    transitions: Vec<Transition<T>>,
    parameters: Rc<T>,
}

impl<T> StateMachine<T> {
    pub fn new(
        starting_state: String,
        states: HashMap<String, State>,
        transitions: Vec<Transition<T>>,
        parameters: T,
    ) -> Self {
        Self {
            current_state: CurrentState {
                name: starting_state,
                elapsed: 0.0,
                duration: 0.0,
            },
            states,
            transitions,
            parameters: Rc::new(parameters),
        }
    }

    pub fn update_parameters(&mut self, update: Box<dyn Fn(Rc<T>) -> Rc<T>>) {
        self.parameters = update(self.parameters.clone());
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_state.elapsed += delta_time;

        let transitions: Vec<&Transition<T>> = self
            .transitions
            .iter()
            .filter(|x| {
                x.start_state == StateNode::Name(self.current_state.name.clone())
                    || x.start_state == StateNode::Any
            })
            .collect();

        for transition in transitions {
            match &transition.trigger {
                Trigger::Condition(condition) => {
                    if condition(&self.parameters) {
                        self.current_state.name = match &transition.end_state {
                            StateNode::Name(name) => name.clone(),
                            StateNode::Any => panic!("invalid end state any"),
                        };
                        self.current_state.elapsed = 0.0;
                        self.current_state.duration =
                            self.states.get(&self.current_state.name).unwrap().duration;

                        return;
                    }
                }
            }
        }
    }
}

pub struct CurrentState {
    pub name: String,
    pub elapsed: f32,
    pub duration: f32,
}

pub struct State {
    pub duration: f32,
}

#[derive(Clone, PartialEq)]
pub enum StateNode {
    Any,
    Name(String),
}

pub struct Transition<T> {
    pub start_state: StateNode,
    pub end_state: StateNode,
    pub trigger: Trigger<T>,
}

pub enum Trigger<T> {
    Condition(Box<fn(&Rc<T>) -> bool>),
}
