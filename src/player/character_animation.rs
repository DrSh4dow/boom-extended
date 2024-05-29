use bevy::prelude::*;

use super::player_components::{
    AnimationIndices, AnimationTimer, MovementAction, PlayerMovementState, PlayerOne,
};

pub fn character_animation(
    mut player: Query<
        (
            &mut TextureAtlas,
            &PlayerMovementState,
            &AnimationIndices,
            &mut AnimationTimer,
        ),
        With<PlayerOne>,
    >,
    time: Res<Time>,
) {
    let (mut player_texture_atlas, movement_state, animation_indices, mut animation_timer) =
        player.single_mut();

    animation_timer.tick(time.delta());

    match movement_state.0 {
        MovementAction::Still => {
            player_texture_atlas.index = animation_indices.still;
        }
        MovementAction::Down => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index
                    >= animation_indices.down.1
                    || player_texture_atlas.index < animation_indices.down.0
                {
                    animation_indices.down.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
        MovementAction::Left => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index
                    >= animation_indices.left.1
                    || player_texture_atlas.index < animation_indices.left.0
                {
                    animation_indices.left.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
        MovementAction::Right => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index
                    >= animation_indices.right.1
                    || player_texture_atlas.index < animation_indices.right.0
                {
                    animation_indices.right.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
        MovementAction::Up => {
            if animation_timer.just_finished() {
                player_texture_atlas.index = if player_texture_atlas.index >= animation_indices.up.1
                    || player_texture_atlas.index < animation_indices.up.0
                {
                    animation_indices.up.0
                } else {
                    player_texture_atlas.index + 1
                }
            }
        }
    };
}
