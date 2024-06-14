use std::fmt::{Debug, Formatter};

#[derive(Clone, Debug)]
pub struct StateMachine<K, V> {
    pub(crate) current_state: CurrentState<K>,
    pub(crate) states: Vec<(K, State)>,
    pub(crate) transitions: Vec<Transition<K, V>>,
    pub(crate) parameters: V,
}

impl<K, V> StateMachine<K, V>
where
    K: Clone + Eq + PartialEq,
{
    /// Creates a new [`StateMachine`]
    pub fn new(
        starting_state: K,
        states: Vec<(K, State)>,
        transitions: Vec<Transition<K, V>>,
        parameters: V,
    ) -> Result<Self, StateMachineError<K>> {
        // validate that the starting state exists
        let start = match states.iter().find(|(k, _)| k == &starting_state).map(|(_, state)| state) {
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
                    if !states.iter().any(|(k, _)| k == key) {
                        return Err(StateMachineError::InvalidTransitionStartState(key.clone()));
                    }
                }
            }
            match &transition.end_state {
                TransitionEndState::Node(key) => {
                    if !states.iter().any(|(k, _)| k == key) {
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
            parameters,
        })
    }

    /// Returns the current state
    pub fn state(&self) -> &CurrentState<K> {
        &self.current_state
    }

    /// Returns the parameters
    pub fn parameters(&self) -> &V {
        &self.parameters
    }

    fn transition(&mut self) {
        let mut visited = Vec::new();

        loop {
            let start_state = TransitionStartState::Node(self.current_state.key.clone());
            let state_ended = self.current_state.elapsed >= self.current_state.duration;
            if let Some(transition) = self.transitions.iter().find(|x| {
                (x.start_state == start_state || x.start_state == TransitionStartState::Any)
                    && match &x.end_state {
                        TransitionEndState::Node(node) => node != &self.current_state.key,
                    }
                    && match &x.trigger {
                        TransitionTrigger::Condition(condition) => condition(&self.parameters),
                        TransitionTrigger::End => state_ended,
                    }
            }) {
                let TransitionEndState::Node(end_state_key) = &transition.end_state;

                if visited.contains(end_state_key) {
                    // We have already visited this state, so we should stop
                    break;
                }

                let end_state = match self.states.iter().find(|(k, _)| k == end_state_key).map(|(_, state)| state) {
                    Some(state) => state,
                    None => unreachable!(),
                };

                self.current_state.key = end_state_key.clone();
                self.current_state.duration = end_state.duration;
                self.current_state.elapsed = 0.0;
                self.current_state.repeat = end_state.repeat;

                visited.push(end_state_key.clone());
            } else {
                break;
            }
        }
    }

    /// Updates the parameters
    pub fn update_parameters(&mut self, update: &dyn Fn(&mut V)) {
        update(&mut self.parameters);

        let start_state = TransitionStartState::Node(self.current_state.key.clone());

        // Only trigger conditional transitions since the time has not changed
        if let Some(transition) = self.transitions.iter().find(|x| {
            (x.start_state == start_state || x.start_state == TransitionStartState::Any)
                && match &x.end_state {
                    TransitionEndState::Node(node) => node != &self.current_state.key,
                }
                && match &x.trigger {
                    TransitionTrigger::Condition(condition) => condition(&self.parameters),
                    _ => false,
                }
        }) {
            let TransitionEndState::Node(end_state_key) = &transition.end_state;
            let end_state = match self.states.iter().find(|(k, _)| k == end_state_key).map(|(_, state)| state) {
                Some(state) => state,
                None => unreachable!(),
            };

            self.current_state.key = end_state_key.clone();
            self.current_state.duration = end_state.duration;
            self.current_state.elapsed = 0.0;
            self.current_state.repeat = end_state.repeat;

            // Make sure we transition through any more transitions
            self.transition();
        };
    }

    /// Updates elapsed time
    pub fn update(&mut self, delta_time: f32) {
        if self.current_state.elapsed < self.current_state.duration {
            self.current_state.elapsed += delta_time;

            if self.current_state.elapsed >= self.current_state.duration {
                if self.current_state.repeat {
                    self.current_state.elapsed %= self.current_state.duration
                } else {
                    self.current_state.elapsed = self.current_state.duration;
                }

                let start_state = TransitionStartState::Node(self.current_state.key.clone());

                // Only trigger end transitions since the parameters have not changed
                if let Some(transition) = self.transitions.iter().find(|x| {
                    matches!(x.trigger, TransitionTrigger::End)
                        && (x.start_state == start_state
                            || x.start_state == TransitionStartState::Any)
                        && match &x.end_state {
                            TransitionEndState::Node(node) => node != &self.current_state.key,
                        }
                }) {
                    let TransitionEndState::Node(end_state_key) = &transition.end_state;
                    let end_state = match self.states.iter().find(|(k, _)| k == end_state_key).map(|(_, state)| state) {
                        Some(state) => state,
                        None => unreachable!(),
                    };

                    self.current_state.key = end_state_key.clone();
                    self.current_state.duration = end_state.duration;
                    self.current_state.elapsed = 0.0;
                    self.current_state.repeat = end_state.repeat;

                    // Make sure we transition through any more transitions
                    self.transition();
                }
            }
        }
    }
}

/// A state machine error
#[derive(Clone, PartialEq, Debug)]
pub enum StateMachineError<K> {
    /// The starting state does not exist
    InvalidStartingState(K),
    /// The start state of a transition does not exist
    InvalidTransitionStartState(K),
    /// The end state of a transition does not exist
    InvalidTransitionEndState(K),
}

/// A state machine's current state
#[derive(Clone, PartialEq, Debug)]
pub struct CurrentState<K> {
    /// The current state key
    pub key: K,
    /// The current state duration
    pub duration: f32,
    /// The current state elapsed time
    pub elapsed: f32,
    /// Whether the current state repeats
    pub repeat: bool,
}

impl<K> CurrentState<K> {
    /// Returns the current state's progress [0.0, 1.0)
    pub fn progress(&self) -> f32 {
        self.elapsed / self.duration
    }

    /// Returns whether the current state is finished
    pub fn finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

/// A state
#[derive(Clone, PartialEq, Debug)]
pub struct State {
    /// The state duration
    pub duration: f32,
    /// Whether the state repeats
    pub repeat: bool,
}

/// A transition
#[derive(Clone, Debug)]
pub struct Transition<K, V> {
    /// The start state
    pub start_state: TransitionStartState<K>,
    /// The end state
    pub end_state: TransitionEndState<K>,
    /// The trigger
    pub trigger: TransitionTrigger<V>,
}

/// A transition start state
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransitionStartState<K> {
    /// Any state
    Any,
    /// A specific state
    Node(K),
}

/// A transition end state
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransitionEndState<K> {
    /// A specific state
    Node(K),
}

/// A trigger
#[derive(Clone)]
pub enum TransitionTrigger<V> {
    /// A condition
    Condition(Box<fn(&V) -> bool>),
    /// End
    End,
}

impl<V> Debug for TransitionTrigger<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionTrigger::Condition(_) => write!(f, "Condition"),
            TransitionTrigger::End => write!(f, "End"),
        }
    }
}