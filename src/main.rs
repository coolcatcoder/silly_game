/// Just a silly game where a wall with a hole approaches. You have a set number of blocks, rearrange yourself to fit through the hole.
/// but that is just the battle. Having one of your blocks not go through a hole lowers your health. Like deltarune, but squares go through square holes.
use bevy::prelude::*;

mod actions;
mod battle;
mod events;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, actions::plugin, battle::plugin))
        .run();
}
