use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .init_resource::<ActionState<Action>>()
        .insert_resource(input_map());
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Pull,
}

fn input_map() -> InputMap<Action> {
    InputMap::new([
        (Action::Up, KeyCode::KeyW),
        (Action::Down, KeyCode::KeyS),
        (Action::Left, KeyCode::KeyA),
        (Action::Right, KeyCode::KeyD),
        (Action::Pull, KeyCode::KeyF),
    ])
}

pub type Actions<'w> = Res<'w, ActionState<Action>>;
