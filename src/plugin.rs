use bevy::prelude::*;

use crate::Animator;

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {}
}

pub trait AnimatorAppExt<K, V, F>
where
    K: Clone + Eq + PartialEq + Send + Sync + 'static,
    V: Send + Sync + 'static,
    F: Send + Sync + 'static,
{
    fn add_animator(&mut self) -> &mut Self;
}

impl<K, V, F> AnimatorAppExt<K, V, F> for App
where
    K: Clone + Eq + PartialEq + Send + Sync + 'static,
    V: Send + Sync + 'static,
    F: Send + Sync + 'static,
{
    fn add_animator(&mut self) -> &mut Self {
        self.add_systems(Update, update::<K, V, F>)
    }
}

pub fn update<K, V, F>(q_animator: Query<&mut Animator<K, V, F>>)
where
    K: Clone + Eq + PartialEq + Send + Sync + 'static,
    V: Send + Sync + 'static,
    F: Send + Sync + 'static,
{
    println!("update");
}
