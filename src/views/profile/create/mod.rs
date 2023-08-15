use crate::prelude::*;

use self::initial::InitialState;

mod initial;

#[derive(Clone, Default)]
enum Stage {
    #[default]
    Initial,
    MinecraftVersion,
    // ? Summary
}

#[derive(Clone, Default)]
pub struct State {
    stage: Stage,
    initial: InitialState,
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, _app: &App, state: &State) {
    match state.stage {
        Stage::Initial => initial::draw(frame, _app, &state.initial),
        Stage::MinecraftVersion => {}
    }
}

pub fn controls(input: Input, _app: &mut App, state: &mut State) {
    match state.stage {
        Stage::Initial => initial::controls(input, _app, state),
        Stage::MinecraftVersion => {}
    }
}
