#![warn(missing_docs)]
//! # Rust Sprite Animator
//!
//! A basic state machine for managing sprite animations.
//!
//! Example usage:
//!
//! ```
//! use rsanim::*;
//! use std::collections::HashMap;
//!
//! #[derive(Clone, Eq, PartialEq, Hash, Debug)]
//! enum Animation {
//!     Idle,
//!     Run,
//! }
//!
//! #[derive(Clone, Debug, PartialEq)]
//! struct Params {
//!     pub speed: f32,
//! }
//!
//! let mut state_machine = StateMachine::new(
//!     Animation::Idle,
//!     HashMap::from([
//!         (
//!             Animation::Idle,
//!             State {
//!                 duration: 0.5,
//!                 repeat: true,
//!             },
//!         ),
//!         (
//!             Animation::Run,
//!             State {
//!                 duration: 1.0,
//!                 repeat: true,
//!             },
//!         ),
//!     ]),
//!     vec![
//!         Transition {
//!             start_state: TransitionStartState::Node(Animation::Idle),
//!             end_state: TransitionEndState::Node(Animation::Run),
//!             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
//!         },
//!         Transition {
//!             start_state: TransitionStartState::Node(Animation::Run),
//!             end_state: TransitionEndState::Node(Animation::Idle),
//!             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
//!         },
//!     ],
//!     Params { speed: 0.0 },
//! )
//! .unwrap();
//! ```
//!
//! Update the state machine as time passes:
//!
//! ```
//! # use rsanim::*;
//! # use std::collections::HashMap;
//!
//! # #[derive(Clone, Eq, PartialEq, Hash, Debug)]
//! # enum Animation {
//! #     Idle,
//! #     Run,
//! # }
//!
//! # #[derive(Clone, Debug, PartialEq)]
//! # struct Params {
//! #     pub speed: f32,
//! # }
//!
//! # let mut state_machine = StateMachine::new(
//! #     Animation::Idle,
//! #     HashMap::from([
//! #         (
//! #             Animation::Idle,
//! #             State {
//! #                 duration: 0.5,
//! #                 repeat: true,
//! #             },
//! #         ),
//! #         (
//! #             Animation::Run,
//! #             State {
//! #                 duration: 1.0,
//! #                 repeat: true,
//! #             },
//! #         ),
//! #     ]),
//! #     vec![
//! #         Transition {
//! #             start_state: TransitionStartState::Node(Animation::Idle),
//! #             end_state: TransitionEndState::Node(Animation::Run),
//! #             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
//! #         },
//! #         Transition {
//! #             start_state: TransitionStartState::Node(Animation::Run),
//! #             end_state: TransitionEndState::Node(Animation::Idle),
//! #             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
//! #         },
//! #     ],
//! #     Params { speed: 0.0 },
//! # )
//! # .unwrap();
//! # let delta_time = 0.1;
//! state_machine.update(delta_time);
//! ```
//!
//! Update the parameters that are used to determine conditional transitions:
//!
//! ```
//! # use rsanim::*;
//! # use std::collections::HashMap;
//!
//! # #[derive(Clone, Eq, PartialEq, Hash, Debug)]
//! # enum Animation {
//! #     Idle,
//! #     Run,
//! # }
//!
//! # #[derive(Clone, Debug, PartialEq)]
//! # struct Params {
//! #     pub speed: f32,
//! # }
//!
//! # let mut state_machine = StateMachine::new(
//! #     Animation::Idle,
//! #     HashMap::from([
//! #         (
//! #             Animation::Idle,
//! #             State {
//! #                 duration: 0.5,
//! #                 repeat: true,
//! #             },
//! #         ),
//! #         (
//! #             Animation::Run,
//! #             State {
//! #                 duration: 1.0,
//! #                 repeat: true,
//! #             },
//! #         ),
//! #     ]),
//! #     vec![
//! #         Transition {
//! #             start_state: TransitionStartState::Node(Animation::Idle),
//! #             end_state: TransitionEndState::Node(Animation::Run),
//! #             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
//! #         },
//! #         Transition {
//! #             start_state: TransitionStartState::Node(Animation::Run),
//! #             end_state: TransitionEndState::Node(Animation::Idle),
//! #             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
//! #         },
//! #     ],
//! #     Params { speed: 0.0 },
//! # )
//! # .unwrap();
//! state_machine.update_parameters(&|x| {
//!     x.speed = 1.0;
//! });
//! ```

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

/// The state machine.
///
/// Use this to track an entity's animation state.
///
/// ```
/// # use rsanim::*;
/// # use std::collections::HashMap;
///
/// #[derive(Clone, Eq, PartialEq, Hash, Debug)]
/// enum Animation {
///     Idle,
///     Run,
/// }
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Params {
///     pub speed: f32,
/// }
///
/// let mut state_machine = StateMachine::new(
///     Animation::Idle,
///     HashMap::from([
///         (
///             Animation::Idle,
///             State {
///                 duration: 0.5,
///                 repeat: true,
///             },
///         ),
///         (
///             Animation::Run,
///             State {
///                 duration: 1.0,
///                 repeat: true,
///             },
///         ),
///     ]),
///     vec![
///         Transition {
///             start_state: TransitionStartState::Node(Animation::Idle),
///             end_state: TransitionEndState::Node(Animation::Run),
///             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
///         },
///         Transition {
///             start_state: TransitionStartState::Node(Animation::Run),
///             end_state: TransitionEndState::Node(Animation::Idle),
///             trigger: Trigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
///         },
///     ],
///     Params { speed: 0.0 },
/// )
/// .unwrap();
///
/// state_machine.update_parameters(&|x| {
///    x.speed = 1.0;
/// });
///
/// state_machine.update(0.1);
/// ```
#[derive(Clone, Debug)]
pub struct StateMachine<K, V> {
    current_state: CurrentState<K>,
    states: HashMap<K, State>,
    transitions: Vec<Transition<K, V>>,
    parameters: V,
}

impl<K, V> StateMachine<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
{
    /// Creates a new [`StateMachine`]
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

    /// Updates the parameters
    pub fn update_parameters(&mut self, update: &dyn Fn(&mut V)) {
        update(&mut self.parameters);

        let start_state = TransitionStartState::Node(self.current_state.key.clone());

        if let Some(transition) = self.transitions.iter().find(|x| {
            (x.start_state == start_state || x.start_state == TransitionStartState::Any)
                && match &x.trigger {
                    Trigger::Condition(condition) => condition(&self.parameters),
                    _ => false,
                }
        }) {
            let TransitionEndState::Node(end_state_name) = &transition.end_state;
            let end_state = match self.states.get(end_state_name) {
                Some(state) => state,
                None => unreachable!(),
            };

            self.current_state.key = end_state_name.clone();
            self.current_state.duration = end_state.duration;
            self.current_state.elapsed = 0.0;
            self.current_state.repeat = end_state.repeat;
        };
    }

    /// Updates elapsed time
    pub fn update(&mut self, delta_time: f32) {
        if self.current_state.elapsed < self.current_state.duration {
            self.current_state.elapsed += delta_time;

            if self.current_state.elapsed >= self.current_state.duration {
                if self.current_state.repeat {
                    self.current_state.elapsed =
                        self.current_state.elapsed % self.current_state.duration;
                } else {
                    self.current_state.elapsed = self.current_state.duration;
                }

                let start_state = TransitionStartState::Node(self.current_state.key.clone());

                if let Some(transition) = self.transitions.iter().find(|x| {
                    matches!(x.trigger, Trigger::End)
                        && (x.start_state == start_state
                            || x.start_state == TransitionStartState::Any)
                }) {
                    let TransitionEndState::Node(end_state_name) = &transition.end_state;
                    let end_state = match self.states.get(end_state_name) {
                        Some(state) => state,
                        None => unreachable!(),
                    };

                    self.current_state.key = end_state_name.clone();
                    self.current_state.duration = end_state.duration;
                    self.current_state.elapsed = 0.0;
                    self.current_state.repeat = end_state.repeat;
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
    pub trigger: Trigger<V>,
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
pub enum Trigger<V> {
    /// A condition
    Condition(Box<fn(&V) -> bool>),
    /// End
    End,
}

impl<V> Debug for Trigger<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Trigger::Condition(_) => write!(f, "Condition"),
            Trigger::End => write!(f, "End"),
        }
    }
}
