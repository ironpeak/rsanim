pub mod animator;
pub mod state_machine;
pub mod plugin;

pub mod prelude {
    pub use super::animator::{Animator, AnimatorParams, State, Transition, TransitionEndState, TransitionStartState, TransitionTrigger, AnimatorError, Frame};
}

use prelude::*;