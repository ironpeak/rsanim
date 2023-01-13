use std::hash::Hash;
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct StateMachine<K, V> {
    current_state: CurrentState<K>,
    states: HashMap<K, State>,
    transitions: Vec<Transition<K, V>>,
    parameters: Rc<V>,
}

impl<K, V> StateMachine<K, V>
where
    K: Clone + Eq + Hash,
{
    pub fn new(
        starting_state: K,
        states: HashMap<K, State>,
        transitions: Vec<Transition<K, V>>,
        parameters: V,
    ) -> Result<Self, StateMachineError<K>> {
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
                TransitionStartState::Node(key) => {
                    if !states.contains_key(key) {
                        return Err(StateMachineError::InvalidTransitionStartState(key.clone()));
                    }
                }
            }
            match &transition.end_state {
                TransitionEndState::Node(key) => {
                    if !states.contains_key(key) {
                        return Err(StateMachineError::InvalidTransitionEndState(key.clone()));
                    }
                }
            }
        }
        Ok(Self {
            current_state: CurrentState {
                key: starting_state,
                duration: start.duration,
                elapsed: 0.0,
                repeat: start.repeat,
            },
            states,
            transitions,
            parameters: Rc::new(parameters),
        })
    }

    pub fn state(&self) -> &CurrentState<K> {
        &self.current_state
    }

    pub fn parameters(&self) -> &Rc<V> {
        &self.parameters
    }

    pub fn update_parameters(&mut self, update: Box<dyn Fn(Rc<V>) -> Rc<V>>) {
        self.parameters = update(self.parameters.clone());

        let start_state = TransitionStartState::Node(self.current_state.key.clone());

        match self.transitions.iter().find(|x| {
            (x.start_state == start_state || x.start_state == TransitionStartState::Any)
                && match &x.trigger {
                    Trigger::Condition(condition) => condition(&self.parameters),
                    _ => false,
                }
        }) {
            Some(transition) => {
                let end_state_name = match &transition.end_state {
                    TransitionEndState::Node(key) => key,
                };
                let end_state = match self.states.get(end_state_name) {
                    Some(state) => state,
                    None => unreachable!(),
                };

                self.current_state.key = end_state_name.clone();
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

                let start_state = TransitionStartState::Node(self.current_state.key.clone());

                match self.transitions.iter().find(|x| {
                    matches!(x.trigger, Trigger::End)
                        && (x.start_state == start_state
                            || x.start_state == TransitionStartState::Any)
                }) {
                    Some(transition) => {
                        let end_state_name = match &transition.end_state {
                            TransitionEndState::Node(name) => name,
                        };
                        let end_state = match self.states.get(end_state_name) {
                            Some(state) => state,
                            None => unreachable!(),
                        };

                        self.current_state.key = end_state_name.clone();
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

#[derive(Clone, PartialEq, Debug)]
pub enum StateMachineError<K> {
    InvalidStartingState(K),
    InvalidTransitionStartState(K),
    InvalidTransitionEndState(K),
}

#[derive(Clone, PartialEq, Debug)]
pub struct CurrentState<K> {
    pub key: K,
    pub duration: f32,
    pub elapsed: f32,
    pub repeat: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub duration: f32,
    pub repeat: bool,
}

#[derive(Clone)]
pub struct Transition<K, T> {
    pub start_state: TransitionStartState<K>,
    pub end_state: TransitionEndState<K>,
    pub trigger: Trigger<T>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransitionStartState<K> {
    Any,
    Node(K),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransitionEndState<K> {
    Node(K),
}

#[derive(Clone)]
pub enum Trigger<T> {
    Condition(Box<fn(&Rc<T>) -> bool>),
    End,
}
