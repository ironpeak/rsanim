pub mod animator;
pub mod state_machine;
pub mod plugin;

pub mod prelude {
    pub use super::{animator::{Animator, AnimatorError, Frame}, state_machine::{StateMachine, Transition, TransitionEndState, TransitionStartState, TransitionTrigger}};
}

use bevy::{app::PluginGroupBuilder, prelude::*};
use plugin::AnimatorPlugin;
use prelude::*;

pub struct AnimPlugins;

impl PluginGroup for AnimPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AnimatorPlugin)
    }
}
