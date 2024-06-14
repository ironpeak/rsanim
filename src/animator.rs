use std::fmt::{Formatter, Debug};

use bevy_ecs::component::Component;

use crate::state_machine::{SMState, SMTransition, SMTransitionEndState, SMTransitionStartState, StateMachine};

#[derive(Component, Clone, Debug)]
pub struct Animator<TKey, TParams> {
    state_machine: StateMachine<TKey, TParams>,
    state_frames: Vec<Vec<Frame>>,
}

impl<TKey, TParams> Animator<TKey, TParams>
where
    TKey: Copy + Eq + PartialEq,
{
    /// Creates a new [`Animator`]
    pub fn new(
        starting_state: TKey,
        states: Vec<(TKey, State<TKey>)>,
        transitions: Vec<Transition<TKey, TParams>>,
        parameters: TParams,
        state_frames: Vec<(TKey, Vec<Frame>)>,
    ) -> Result<Self, AnimatorError<TKey>> {
        let states = states
            .into_iter()
            .map(|(key, state)| SMState {
                key,
                duration: state.duration,
                repeat: state.repeat,
            })
            .collect::<Vec<_>>();
        let starting_state_index = states
            .iter()
            .position(|x| x.key == starting_state)
            .ok_or(AnimatorError::MissingStateFrames(starting_state))?;
        let transitions = transitions
            .into_iter()
            .map(|transition| {
                let start_state = match transition.start_state {
                    TransitionStartState::Any => SMTransitionStartState::Any,
                    TransitionStartState::Node(key) => {
                        let index = states
                            .iter()
                            .position(|x| x.key == key)
                            .ok_or(AnimatorError::MissingStateFrames(key))?;
                        SMTransitionStartState::Node(index)
                    }
                };
                let end_state = match transition.end_state {
                    TransitionEndState::Node(key) => {
                        let index = states
                            .iter()
                            .position(|x| x.key == key)
                            .ok_or(AnimatorError::MissingStateFrames(key))?;
                        SMTransitionEndState::Node(index)
                    }
                };
                Ok(SMTransition {
                    start_state,
                    end_state,
                    trigger: transition.trigger,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut new_state_frames = vec![Vec::new(); states.len()];
        for (key, frames) in state_frames.into_iter() {
            let index = states
                .iter()
                .position(|x| x.key == key)
                .ok_or(AnimatorError::MissingStateFrames(key))?;
            new_state_frames[index] = frames;
        }
        Ok(Animator {
            state_machine: StateMachine::new(starting_state_index, states, transitions, parameters),
            state_frames: new_state_frames,
        })
    }

    /// Updates elapsed time
    pub fn update(&mut self, delta_time: f32) {
        self.state_machine.update(delta_time);
    }

    /// Updates the parameters
    pub fn update_parameters(&mut self, update: &dyn Fn(&mut TParams)) {
        self.state_machine.update_parameters(update);
    }

    /// Returns the current state
    pub fn state(&self) -> TKey {
        self.state_machine.state().key
    }

    /// Updates the parameters
    pub fn parameters(&self) -> &TParams {
        self.state_machine.parameters()
    }

    /// Returns the current frame
    pub fn frame(&self) -> usize {
        let current_state = self.state_machine.state();
        let frames = &self.state_frames[current_state.index];

        let progress = current_state.progress();
        let mut frame = &frames[0];
        for f in frames {
            if f.progress > progress {
                return frame.index;
            }
            frame = f;
        }
        frame.index
    }
}

#[derive(Component, Clone, Debug)]
pub struct AnimatorParams<TParams>{
    pub parameters: TParams,
}

/// An animation frame
#[derive(Clone, Debug)]
pub struct Frame {
    /// When the frame should be displayed [0.0, 1.0).
    pub progress: f32,
    /// The frame index.
    pub index: usize,
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

/// A state
#[derive(Clone, PartialEq, Debug)]
pub struct State<K> {
    /// The current state key
    pub key: K,
    /// The state duration
    pub duration: f32,
    /// Whether the state repeats
    pub repeat: bool,
}

/// A transition
#[derive(Clone, Debug)]
pub struct Transition<TKey, TParams> {
    /// The start state
    pub start_state: TransitionStartState<TKey>,
    /// The end state
    pub end_state: TransitionEndState<TKey>,
    /// The trigger
    pub trigger: TransitionTrigger<TParams>,
}

/// A transition start state
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransitionStartState<TKey> {
    /// Any state
    Any,
    /// A specific state
    Node(TKey),
}

/// A transition end state
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TransitionEndState<TKey> {
    /// A specific state
    Node(TKey),
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