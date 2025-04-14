use bevy::prelude::*;
use std::{cell::OnceCell, fmt::Debug};

/// An event that should occur at most once per frame.
/// Can be written to in parallel.
/// Singleton.
#[derive(Component)]
pub struct ParallelRareEvent<T>(OnceCell<T>);
impl<T: Debug> ParallelRareEvent<T> {
    pub fn send(&self, event: T) {
        if let Err(event) = self.0.set(event) {
            error!(
                "Could not send event.\nPrevious: {:?}\nNew: {:?}",
                self.0.get().unwrap(),
                event
            );
        }
    }
}