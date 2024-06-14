use std::fmt::Debug;

use crate::animator::TransitionTrigger;

#[derive(Clone, Debug)]
pub(crate) struct StateMachine<TKey, TParams> {
    pub current_state: SMCurrentState<TKey>,
    pub states: Vec<SMState<TKey>>,
    pub transitions: Vec<SMTransition<TParams>>,
    pub parameters: TParams,
}

impl<TKey, TParams> StateMachine<TKey, TParams>
where
    TKey: Copy + Eq + PartialEq,
{
    /// Creates a new [`StateMachine`]
    pub fn new(
        start_state_index: usize,
        states: Vec<SMState<TKey>>,
        transitions: Vec<SMTransition<TParams>>,
        parameters: TParams,
    ) -> Self {
        let start_state = &states[start_state_index];
        Self {
            current_state: SMCurrentState {
                index: start_state_index,
                key: start_state.key,
                duration: start_state.duration,
                elapsed: 0.0,
                repeat: start_state.repeat,
            },
            states,
            transitions,
            parameters,
        }
    }

    /// Returns the current state
    pub fn state(&self) -> &SMCurrentState<TKey> {
        &self.current_state
    }

    /// Returns the parameters
    pub fn parameters(&self) -> &TParams {
        &self.parameters
    }

    fn transition(&mut self) {
        let mut visited = vec![false; self.states.len()];

        loop {
            let start_state = SMTransitionStartState::Node(self.current_state.index);
            let state_ended = self.current_state.elapsed >= self.current_state.duration;
            if let Some(transition) = self.transitions.iter().find(|x| {
                (x.start_state == start_state || x.start_state == SMTransitionStartState::Any)
                    && match &x.end_state {
                        SMTransitionEndState::Node(index) => index != &self.current_state.index,
                    }
                    && match &x.trigger {
                        TransitionTrigger::Condition(condition) => condition(&self.parameters),
                        TransitionTrigger::End => state_ended,
                    }
            }) {
                let SMTransitionEndState::Node(end_state_index) = transition.end_state;

                if visited[end_state_index] {
                    // We have already visited this state, so we should stop
                    break;
                }

                let end_state = &self.states[end_state_index];

                self.current_state.index = end_state_index;
                self.current_state.key = end_state.key;
                self.current_state.duration = end_state.duration;
                self.current_state.elapsed = 0.0;
                self.current_state.repeat = end_state.repeat;

                visited[end_state_index] = true;
            } else {
                break;
            }
        }
    }

    /// Updates the parameters
    pub fn update_parameters(&mut self, update: &dyn Fn(&mut TParams)) {
        update(&mut self.parameters);

        let start_state = SMTransitionStartState::Node(self.current_state.index);

        // Only trigger conditional transitions since the time has not changed
        if let Some(transition) = self.transitions.iter().find(|x| {
            (x.start_state == start_state || x.start_state == SMTransitionStartState::Any)
                && match &x.end_state {
                    SMTransitionEndState::Node(index) => index != &self.current_state.index,
                }
                && match &x.trigger {
                    TransitionTrigger::Condition(condition) => condition(&self.parameters),
                    _ => false,
                }
        }) {
            let SMTransitionEndState::Node(end_state_index) = transition.end_state;
            let end_state = &self.states[end_state_index];

            self.current_state.index = end_state_index;
            self.current_state.key = end_state.key;
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

                let start_state = SMTransitionStartState::Node(self.current_state.index);

                // Only trigger end transitions since the parameters have not changed
                if let Some(transition) = self.transitions.iter().find(|x| {
                    matches!(x.trigger, TransitionTrigger::End)
                        && (x.start_state == start_state
                            || x.start_state == SMTransitionStartState::Any)
                        && match &x.end_state {
                            SMTransitionEndState::Node(index) => index == &self.current_state.index,
                        }
                }) {
                    let SMTransitionEndState::Node(end_state_index) = transition.end_state;
                    let end_state = &self.states[end_state_index];

                    self.current_state.index = end_state_index;
                    self.current_state.key = end_state.key;
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

/// A state machine's current state
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct SMCurrentState<K> {
    /// The current state index
    pub index: usize,
    /// The current state key
    pub key: K,
    /// The current state duration
    pub duration: f32,
    /// The current state elapsed time
    pub elapsed: f32,
    /// Whether the current state repeats
    pub repeat: bool,
}

impl<K> SMCurrentState<K> {
    /// Returns the current state's progress [0.0, 1.0)
    pub fn progress(&self) -> f32 {
        self.elapsed / self.duration
    }
}

/// A state
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct SMState<K> {
    /// The current state key
    pub key: K,
    /// The state duration
    pub duration: f32,
    /// Whether the state repeats
    pub repeat: bool,
}

/// A transition
#[derive(Clone, Debug)]
pub(crate) struct SMTransition<TParams> {
    /// The start state
    pub start_state: SMTransitionStartState,
    /// The end state
    pub end_state: SMTransitionEndState,
    /// The trigger
    pub trigger: TransitionTrigger<TParams>,
}

/// A transition start state
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum SMTransitionStartState {
    /// Any state
    Any,
    /// A specific state
    Node(usize),
}

/// A transition end state
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum SMTransitionEndState {
    /// A specific state
    Node(usize),
}
