use crate::{
    display::screen::Screen,
    input::Input,
    state::state::{State, UIMessage},
};

pub mod data;
pub mod display;
pub mod external;
pub mod finder;
pub mod input;
pub mod log;
pub mod pages;
pub mod process;
pub mod state;
pub mod storage;
pub mod time;
pub mod view;

fn main() -> anyhow::Result<()> {
    let mut state = State::new()?;
    Screen::clear();
    Input::typing_disable()?;
    state.view_display();

    loop {
        let input = Input::handle_input(&state, state.page())?;

        match state.handle_message(input)? {
            UIMessage::Display => {
                state.view_build()?;
                state.set_error(None);
                state.view_display();
            }
            UIMessage::InvalidInput => {
                state.view_build()?;
                state.set_error(Some("Invalid input".to_owned()));
                state.view_display();
            }
            UIMessage::Move(page) => {
                state.move_to(page)?;
                state.view_display();
            }
            UIMessage::Quit => break,
        }
    }

    Input::typing_enable()?;
    Screen::clear();

    Ok(())
}
