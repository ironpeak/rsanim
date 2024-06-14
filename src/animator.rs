use bevy::prelude::*;
use crate::{state_machine::{CurrentState, State}, StateMachine, Transition};

#[derive(Component, Clone, Debug)]
pub struct Animator<TKey, TParams, TFrame> {
    state_machine: StateMachine<TKey, TParams>,
    state_frames: Vec<Vec<Frame<TFrame>>>,
}

impl<TKey, TParams, TFrame> Animator<TKey, TParams, TFrame>
where
    TKey: Clone + Eq + PartialEq,
{
    /// Creates a new [`Animator`]
    pub fn new(
        starting_state: TKey,
        states: Vec<(TKey, State<TKey>)>,
        transitions: Vec<Transition<TKey, TParams>>,
        parameters: TParams,
        state_frames: Vec<(TKey, Vec<Frame<TFrame>>)>,
    ) -> Result<Self, AnimatorError<TKey>> {
        todo!()
        // for (state, _) in &state_machine.states {
        //     match state_frames.iter().find(|(k, _)| k == state).map(|(_, state)| state) {
        //         Some(frames) => {
        //             if frames.is_empty() {
        //                 return Err(AnimatorError::EmptyStateFrames(state.clone()));
        //             }

        //             // make sure frames are sorted by progress
        //             let mut last_progress = -1.0;
        //             for frame in frames {
        //                 if frame.progress < last_progress {
        //                     return Err(AnimatorError::UnsortedStateFrames(state.clone()));
        //                 }
        //                 if frame.progress < 0.0 || frame.progress > 1.0 {
        //                     return Err(AnimatorError::InvalidStateFrameProgress(
        //                         state.clone(),
        //                         frame.progress,
        //                     ));
        //                 }
        //                 last_progress = frame.progress;
        //             }
        //         }
        //         None => return Err(AnimatorError::MissingStateFrames(state.clone())),
        //     }
        // }

        // Ok(Self {
        //     state_machine,
        //     state_frames,
        // })
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
    pub fn state(&self) -> &CurrentState<TKey> {
        self.state_machine.state()
    }

    /// Updates the parameters
    pub fn parameters(&self) -> &TParams {
        self.state_machine.parameters()
    }

    /// Returns the current frame
    pub fn frame(&self) -> &TFrame {
        let current_state = self.state_machine.state();
        let frames = match self.state_frames.iter().find(|(k, _)| k == &current_state.key).map(|(_, state)| state) {
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