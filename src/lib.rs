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
                state: match states.get(&starting_state) {
                    Some(state) => state.clone(),
                    None => panic!("invalid starting state"),
                },
                name: starting_state,
                elapsed: 0.0,
            },
            states,
            transitions,
            parameters: Rc::new(parameters),
        }
    }

    pub fn update_parameters(&mut self, update: Box<dyn Fn(Rc<T>) -> Rc<T>>) {
        self.parameters = update(self.parameters.clone());

        let start_state = TransitionStartState::Name(self.current_state.name.clone());

        match self.transitions.iter().find(|x| {
            (x.start_state == start_state || x.start_state == TransitionStartState::Any)
                && match &x.trigger {
                    Trigger::Condition(condition) => condition(&self.parameters),
                    _ => false,
                }
        }) {
            Some(transition) => {
                self.current_state.name = match &transition.end_state {
                    TransitionEndState::Name(name) => name.clone(),
                };
                self.current_state.elapsed = 0.0;
                self.current_state.state = match self.states.get(&self.current_state.name) {
                    Some(state) => state.clone(),
                    None => {
                        panic!("transition end state {} not found", self.current_state.name)
                    }
                };
            }
            None => {}
        };
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.current_state.elapsed < self.current_state.state.duration {
            self.current_state.elapsed += delta_time;

            if self.current_state.elapsed >= self.current_state.state.duration {
                if self.current_state.state.repeat {
                    self.current_state.elapsed = 0.0;
                }

                let start_state = TransitionStartState::Name(self.current_state.name.clone());

                match self.transitions.iter().find(|x| {
                    matches!(x.trigger, Trigger::End)
                        && (x.start_state == start_state
                            || x.start_state == TransitionStartState::Any)
                }) {
                    Some(transition) => {
                        self.current_state.name = match &transition.end_state {
                            TransitionEndState::Name(name) => name.clone(),
                        };
                        self.current_state.elapsed = 0.0;
                        self.current_state.state = match self.states.get(&self.current_state.name) {
                            Some(state) => state.clone(),
                            None => {
                                panic!("transition end state {} not found", self.current_state.name)
                            }
                        };

                        return;
                    }
                    None => {}
                }
            }
        }
    }
}

pub struct CurrentState {
    pub name: String,
    pub elapsed: f32,
    pub state: State,
}

#[derive(Clone)]
pub struct State {
    pub duration: f32,
    pub repeat: bool,
}

pub struct Transition<T> {
    pub start_state: TransitionStartState,
    pub end_state: TransitionEndState,
    pub trigger: Trigger<T>,
}

#[derive(Clone, PartialEq)]
pub enum TransitionStartState {
    Any,
    Name(String),
}

#[derive(Clone, PartialEq)]
pub enum TransitionEndState {
    Name(String),
}

pub enum Trigger<T> {
    Condition(Box<fn(&Rc<T>) -> bool>),
    End,
}
