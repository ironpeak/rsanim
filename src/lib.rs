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
//!             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
//!         },
//!         Transition {
//!             start_state: TransitionStartState::Node(Animation::Run),
//!             end_state: TransitionEndState::Node(Animation::Idle),
//!             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
//!         },
//!     ],
//!     Params { speed: 0.0 },
//! )
//! .unwrap();
//!
//! let animator = Animator::new(
//!     state_machine,
//!     HashMap::from([
//!         (
//!             Animation::Idle,
//!             vec![
//!                 Frame {
//!                     value: 0,
//!                     progress: 0.00,
//!                 },
//!                 Frame {
//!                     value: 1,
//!                     progress: 0.33,
//!                 },
//!                 Frame {
//!                     value: 2,
//!                     progress: 0.67,
//!                 },
//!             ],
//!         ),
//!         (
//!             Animation::Run,
//!             vec![
//!                 Frame {
//!                     value: 0,
//!                     progress: 0.00,
//!                 },
//!                 Frame {
//!                     value: 1,
//!                     progress: 0.33,
//!                 },
//!                 Frame {
//!                     value: 2,
//!                     progress: 0.67,
//!                 },
//!             ],
//!         ),
//!     ]),
//! )
//! .unwrap();
//! ```
//!
//! Update the state machine's elapsed time:
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
//! #             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
//! #         },
//! #         Transition {
//! #             start_state: TransitionStartState::Node(Animation::Run),
//! #             end_state: TransitionEndState::Node(Animation::Idle),
//! #             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
//! #         },
//! #     ],
//! #     Params { speed: 0.0 },
//! # )
//! # .unwrap();
//!
//! # let mut animator = Animator::new(
//! #     state_machine,
//! #     HashMap::from([
//! #         (
//! #             Animation::Idle,
//! #             vec![
//! #                 Frame {
//! #                     value: 0,
//! #                     progress: 0.00,
//! #                 },
//! #                 Frame {
//! #                     value: 1,
//! #                     progress: 0.33,
//! #                 },
//! #                 Frame {
//! #                     value: 2,
//! #                     progress: 0.67,
//! #                 },
//! #             ],
//! #         ),
//! #         (
//! #             Animation::Run,
//! #             vec![
//! #                 Frame {
//! #                     value: 0,
//! #                     progress: 0.00,
//! #                 },
//! #                 Frame {
//! #                     value: 1,
//! #                     progress: 0.33,
//! #                 },
//! #                 Frame {
//! #                     value: 2,
//! #                     progress: 0.67,
//! #                 },
//! #             ],
//! #         ),
//! #     ]),
//! # )
//! # .unwrap();
//! # let delta_time = 0.1;
//! animator.update(delta_time);
//! ```
//!
//! Update the state machine's parameters that are used to determine conditional transitions:
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
//! #             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
//! #         },
//! #         Transition {
//! #             start_state: TransitionStartState::Node(Animation::Run),
//! #             end_state: TransitionEndState::Node(Animation::Idle),
//! #             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
//! #         },
//! #     ],
//! #     Params { speed: 0.0 },
//! # )
//! # .unwrap();
//!
//! # let mut animator = Animator::new(
//! #     state_machine,
//! #     HashMap::from([
//! #         (
//! #             Animation::Idle,
//! #             vec![
//! #                 Frame {
//! #                     value: 0,
//! #                     progress: 0.00,
//! #                 },
//! #                 Frame {
//! #                     value: 1,
//! #                     progress: 0.33,
//! #                 },
//! #                 Frame {
//! #                     value: 2,
//! #                     progress: 0.67,
//! #                 },
//! #             ],
//! #         ),
//! #         (
//! #             Animation::Run,
//! #             vec![
//! #                 Frame {
//! #                     value: 0,
//! #                     progress: 0.00,
//! #                 },
//! #                 Frame {
//! #                     value: 1,
//! #                     progress: 0.33,
//! #                 },
//! #                 Frame {
//! #                     value: 2,
//! #                     progress: 0.67,
//! #                 },
//! #             ],
//! #         ),
//! #     ]),
//! # )
//! # .unwrap();
//! animator.update_parameters(&|x| {
//!     x.speed = 1.0;
//! });
//! ```

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

/// The animator.
///
/// Used for translating the state machine into a sequence of frames.
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
///             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
///         },
///         Transition {
///             start_state: TransitionStartState::Node(Animation::Run),
///             end_state: TransitionEndState::Node(Animation::Idle),
///             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
///         },
///     ],
///     Params { speed: 0.0 },
/// )
/// .unwrap();
///
/// let mut animator = Animator::new(
///     state_machine,
///     HashMap::from([
///         (
///             Animation::Idle,
///             vec![
///                 Frame {
///                     value: 0,
///                     progress: 0.00,
///                 },
///                 Frame {
///                     value: 1,
///                     progress: 0.33,
///                 },
///                 Frame {
///                     value: 2,
///                     progress: 0.67,
///                 },
///             ],
///         ),
///         (
///             Animation::Run,
///             vec![
///                 Frame {
///                     value: 0,
///                     progress: 0.00,
///                 },
///                 Frame {
///                     value: 1,
///                     progress: 0.33,
///                 },
///                 Frame {
///                     value: 2,
///                     progress: 0.67,
///                 },
///             ],
///         ),
///     ]),
/// )
/// .unwrap();
///
/// animator.update_parameters(&|x| {
///    x.speed = 1.0;
/// });
///
/// animator.update(0.1);
/// ```
#[derive(Clone, Debug)]
pub struct Animator<K, V, F> {
    state_machine: StateMachine<K, V>,
    state_frames: HashMap<K, Vec<Frame<F>>>,
}

impl<K, V, F> Animator<K, V, F>
where
    K: Clone + Eq + PartialEq + Hash,
{
    /// Creates a new [`Animator`]
    pub fn new(
        state_machine: StateMachine<K, V>,
        state_frames: HashMap<K, Vec<Frame<F>>>,
    ) -> Result<Self, AnimatorError<K>> {
        for state in state_machine.states.keys() {
            match state_frames.get(state) {
                Some(frames) => {
                    if frames.is_empty() {
                        return Err(AnimatorError::EmptyStateFrames(state.clone()));
                    }

                    // make sure frames are sorted by progress
                    let mut last_progress = -1.0;
                    for frame in frames {
                        if frame.progress < last_progress {
                            return Err(AnimatorError::UnsortedStateFrames(state.clone()));
                        }
                        if frame.progress < 0.0 || frame.progress > 1.0 {
                            return Err(AnimatorError::InvalidStateFrameProgress(
                                state.clone(),
                                frame.progress,
                            ));
                        }
                        last_progress = frame.progress;
                    }
                }
                None => return Err(AnimatorError::MissingStateFrames(state.clone())),
            }
        }

        Ok(Self {
            state_machine,
            state_frames,
        })
    }

    /// Updates elapsed time
    pub fn update(&mut self, delta_time: f32) {
        self.state_machine.update(delta_time);
    }

    /// Updates the parameters
    pub fn update_parameters(&mut self, update: &dyn Fn(&mut V)) {
        self.state_machine.update_parameters(update);
    }

    /// Returns the current state
    pub fn state(&self) -> &CurrentState<K> {
        self.state_machine.state()
    }

    /// Updates the parameters
    pub fn parameters(&self) -> &V {
        self.state_machine.parameters()
    }

    /// Returns the current frame
    pub fn frame(&self) -> &F {
        let current_state = self.state_machine.state();
        let frames = match self.state_frames.get(&current_state.key) {
            Some(frames) => frames,
            None => unreachable!(),
        };

        let progress = current_state.progress();
        let mut frame = &frames[0];
        for f in frames {
            if f.progress > progress {
                return &frame.value;
            }
            frame = f;
        }
        &frame.value
    }
}

/// An animation frame
#[derive(Clone, Debug)]
pub struct Frame<T> {
    /// When the frame should be displayed [0.0, 1.0).
    pub progress: f32,
    /// The frame value.
    pub value: T,
}

/// A animator error
#[derive(Clone, PartialEq, Debug)]
pub enum AnimatorError<K> {
    /// The state machine contains a state with no frames.
    EmptyStateFrames(K),
    /// The state machine contains a state without any frames.
    MissingStateFrames(K),
    /// The state frames should be sorted by progress.
    UnsortedStateFrames(K),
    /// The state frame progress is invalid.
    InvalidStateFrameProgress(K, f32),
}

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
///             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed > 0.0)),
///         },
///         Transition {
///             start_state: TransitionStartState::Node(Animation::Run),
///             end_state: TransitionEndState::Node(Animation::Idle),
///             trigger: TransitionTrigger::Condition(Box::new(|x: &Params| x.speed <= 0.0)),
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
    pub(crate) current_state: CurrentState<K>,
    pub(crate) states: HashMap<K, State>,
    pub(crate) transitions: Vec<Transition<K, V>>,
    pub(crate) parameters: V,
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

    fn transition(&mut self) {
        let mut visited = HashSet::new();

        loop {
            let start_state = TransitionStartState::Node(self.current_state.key.clone());
            let state_ended = self.current_state.elapsed >= self.current_state.duration;
            if let Some(transition) = self.transitions.iter().find(|x| {
                (x.start_state == start_state || x.start_state == TransitionStartState::Any)
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

                let end_state = match self.states.get(end_state_key) {
                    Some(state) => state,
                    None => unreachable!(),
                };

                self.current_state.key = end_state_key.clone();
                self.current_state.duration = end_state.duration;
                self.current_state.elapsed = 0.0;
                self.current_state.repeat = end_state.repeat;

                visited.insert(end_state_key.clone());
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
                && match &x.trigger {
                    TransitionTrigger::Condition(condition) => condition(&self.parameters),
                    _ => false,
                }
        }) {
            let TransitionEndState::Node(end_state_key) = &transition.end_state;
            let end_state = match self.states.get(end_state_key) {
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
                }) {
                    let TransitionEndState::Node(end_state_key) = &transition.end_state;
                    let end_state = match self.states.get(end_state_key) {
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
