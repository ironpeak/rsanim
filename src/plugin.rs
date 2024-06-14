use bevy::prelude::*;

use crate::{animator::AnimatorParams, Animator};

pub trait AnimatorAppExt<TKey, TParams>
where
    TKey: Copy + Eq + PartialEq + Send + Sync + 'static,
    TParams: Clone + Send + Sync + 'static,
{
    fn add_animator(&mut self) -> &mut Self;
}

impl<TKey, TParams> AnimatorAppExt<TKey, TParams> for App
where
    TKey: Copy + Eq + PartialEq + Send + Sync + 'static,
    TParams: Clone + Send + Sync + 'static,
{
    fn add_animator(&mut self) -> &mut Self {
        self.add_systems(Update, (
            update_time::<TKey, TParams>, 
            update_params::<TKey, TParams>.after(update_time::<TKey, TParams>),
            update_atlas::<TKey, TParams>.after(update_params::<TKey, TParams>),
        ))
    }
}

pub fn update_params<TKey, TParams>(mut q_animator: Query<(&mut Animator<TKey, TParams>, &AnimatorParams<TParams>), Changed<AnimatorParams<TParams>>>)
where
    TKey: Copy + Eq + PartialEq + Send + Sync + 'static,
    TParams: Clone + Send + Sync + 'static,
{
    for (mut animator, params) in q_animator.iter_mut() {
        animator.update_parameters(&|x| {
            *x = params.parameters.clone();
        });
    }
}

pub fn update_time<TKey, TParams>(mut q_animator: Query<&mut Animator<TKey, TParams>>, time: Res<Time>)
where
    TKey: Copy + Eq + PartialEq + Send + Sync + 'static,
    TParams: Clone + Send + Sync + 'static,
{
    for mut animator in q_animator.iter_mut() {
        animator.update(time.delta_seconds());
    }
}

pub fn update_atlas<TKey, TParams>(mut query: Query<(&Animator<TKey, TParams>, &mut TextureAtlas)>)
where
    TKey: Copy + Eq + PartialEq + Send + Sync + 'static,
    TParams: Clone + Send + Sync + 'static, {
    for (animator, mut texture_atlas) in query.iter_mut() {
        texture_atlas.index = animator.frame();
    }
}
