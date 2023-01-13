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
    ) -> Result<Self, StateMachineError> {
        // validate that the starting state exists
        let start = match states.get(&starting_state) {
            Some(state) => state,
            None => {
                return Err(StateMachineError::InvalidStartingState(starting_state));
            }
        };
        // validate that the start and end states of each transition exist
        for transition in &transitions {
            match &transition.start_state {
                TransitionStartState::Any => {}
                TransitionStartState::Name(name) => {
                    if !states.contains_key(name) {
                        return Err(StateMachineError::InvalidTransitionStartState(name.clone()));
                    }
                }
            }
            match &transition.end_state {
                TransitionEndState::Name(name) => {
                    if !states.contains_key(name) {
                        return Err(StateMachineError::InvalidTransitionEndState(name.clone()));
                    }
                }
            }
        }
        Ok(Self {
            current_state: CurrentState {
                name: starting_state,
                duration: start.duration,
                elapsed: 0.0,
                repeat: start.repeat,
            },
            states,
            transitions,
            parameters: Rc::new(parameters),
        })
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
                let end_state_name = match &transition.end_state {
                    TransitionEndState::Name(name) => name,
                };
                let end_state = match self.states.get(end_state_name) {
                    Some(state) => state,
                    None => unreachable!(),
                };

                self.current_state.name = end_state_name.clone();
                self.current_state.duration = end_state.duration;
                self.current_state.elapsed = 0.0;
                self.current_state.repeat = end_state.repeat;
            }
            None => {}
        };
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.current_state.elapsed < self.current_state.duration {
            self.current_state.elapsed += delta_time;

            if self.current_state.elapsed >= self.current_state.duration {
                if self.current_state.repeat {
                    self.current_state.elapsed = 0.0;
                }

                let start_state = TransitionStartState::Name(self.current_state.name.clone());

                match self.transitions.iter().find(|x| {
                    matches!(x.trigger, Trigger::End)
                        && (x.start_state == start_state
                            || x.start_state == TransitionStartState::Any)
                }) {
                    Some(transition) => {
                        let end_state_name = match &transition.end_state {
                            TransitionEndState::Name(name) => name,
                        };
                        let end_state = match self.states.get(end_state_name) {
                            Some(state) => state,
                            None => unreachable!(),
                        };

                        self.current_state.name = end_state_name.clone();
                        self.current_state.duration = end_state.duration;
                        self.current_state.elapsed = 0.0;
                        self.current_state.repeat = end_state.repeat;
                    }
                    None => {}
                }
            }
        }
    }
}

pub enum StateMachineError {
    InvalidStartingState(String),
    InvalidTransitionStartState(String),
    InvalidTransitionEndState(String),
}

pub struct CurrentState {
    pub name: String,
    pub duration: f32,
    pub elapsed: f32,
    pub repeat: bool,
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
