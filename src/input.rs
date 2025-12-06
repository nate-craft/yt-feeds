use std::io;

use anyhow::anyhow;

use crate::{
    data::channel::ChannelInfo,
    pages::{
        page_feed_mixed::PageFeedMixed, page_home::PageHome,
        page_search_video_typing::PageSearchVideoTyping, page_watch_later::PageWatchLater,
    },
    state::{
        search::Search,
        state::{Page, PlaySource, State},
    },
};

pub struct Input;

#[derive(Clone)]
pub enum TextInputType {
    Init,
    Char(char),
    Backspace,
    Submit,
}

#[derive(Clone)]
pub enum Message {
    Page(Page),
    Select(usize),
    Back,
    Quit,
    TextInput(TextInputType),
    FinderStart,
    FinderInput(TextInputType),
    FinderEscape,
    InvalidInput,
    Refresh,
    Play,
    PlayDetached,
    PlaySave,
    Save,
    More,
    WatchLaterAdd(PlaySource),
    ListPrevious,
    ListNext,
    Information,
    Subscribe(ChannelInfo),
    // Useful to send a message with nothing to do but redraw (e.g., on window resize)
    Resize,
}

impl Input {
    pub fn typing_enable() -> anyhow::Result<()> {
        crossterm::execute!(io::stdout(), crossterm::cursor::Show)
            .map_err(|err| anyhow!(err).context("Could not show cursor"))?;

        crossterm::terminal::disable_raw_mode()
            .map_err(|err| anyhow!(err).context("Could not enable disable mode"))
    }

    pub fn typing_enable_quiet() -> anyhow::Result<()> {
        crossterm::terminal::disable_raw_mode()
            .map_err(|err| anyhow!(err).context("Could not enable disable mode"))
    }    

    pub fn typing_disable() -> anyhow::Result<()> {
        crossterm::execute!(io::stdout(), crossterm::cursor::Hide)
            .map_err(|err| anyhow!(err).context("Could not hide cursor"))?;

        crossterm::terminal::enable_raw_mode()
            .map_err(|err| anyhow!(err).context("Could not enable disable mode"))
    }

    // Does not validate what kind of capailities a page has
    // Only returns what was asked
    // Capability of a page is handled in state.rs
    pub fn handle_input(state: &State, page: &Page) -> anyhow::Result<Message> {
        use crossterm::event::Event;
        use crossterm::event::KeyCode;
        use crossterm::event::KeyEvent;
        use crossterm::event::KeyModifiers;
        use Message as Msg;

        loop {
            let event = crossterm::event::read().unwrap();
            if let Event::Resize(_, _) = event {
                return Ok(Message::Resize);
            } else if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event
            {
                if let KeyCode::Enter = code && state.typing_enabled() && state.page().text_input_capable() {
                    return match page {
                        Page::SearchVideoTyping(_) | Page::SearchChannelTyping(_) => {
                            Ok(Message::TextInput(TextInputType::Submit))
                        }
                        _ => Ok(Message::FinderInput(TextInputType::Submit)),
                    };
                } else if let KeyCode::Backspace = code && state.typing_enabled() && state.page().text_input_capable() {
                    return match page {
                        Page::SearchVideoTyping(_) | Page::SearchChannelTyping(_) => {
                            Ok(Message::TextInput(TextInputType::Backspace))
                        }
                        _ => Ok(Message::FinderInput(TextInputType::Backspace)),
                    };
                } else if let KeyCode::Char('c') = code {
                    if modifiers.eq(&KeyModifiers::CONTROL) {
                        return Ok(Message::Quit);
                    } else if state.typing_enabled() && state.page().text_input_capable() {
                        return match page {
                            Page::SearchVideoTyping(_) | Page::SearchChannelTyping(_) => {
                                Ok(Message::TextInput(TextInputType::Char('c')))
                            }
                            _ => Ok(Message::FinderInput(TextInputType::Char('c'))),
                        };
                    } else {
                        return match page {
                            Page::Video(_) => Ok(Msg::Save),
                            _ => Ok(Msg::Page(Page::SearchVideoTyping(
                                PageSearchVideoTyping::new(Search::Video(None)),
                            ))),
                        };
                    }
                } else if let KeyCode::Char(c) = code {
                    if state.typing_enabled() && state.page().text_input_capable() {
                        return match page {
                            Page::SearchVideoTyping(_) | Page::SearchChannelTyping(_) => {
                                Ok(Message::TextInput(TextInputType::Char(c)))
                            }
                            _ => Ok(Message::FinderInput(TextInputType::Char(c))),
                        };
                    }

                    if let Some(num) = c.to_digit(10) {
                        return Ok(Message::Select(num as usize));
                    }

                    return match c {
                        'q' => Ok(Msg::Quit),
                        'r' => Ok(Msg::Refresh),
                        '/' => Ok(Msg::FinderStart),
                        'b' => Ok(Msg::Back),
                        'h' => Ok(Msg::Page(Page::Home(PageHome))),
                        'a' => Ok(Msg::Page(Page::FeedMixed(PageFeedMixed))),
                        'v' => Ok(Msg::Page(Page::SearchVideoTyping(
                            PageSearchVideoTyping::new(Search::Channel(None)),
                        ))),
                        's' => match page {
                            Page::SearchChannel(page) => Ok(Msg::Subscribe(page.channel.clone())),
                            Page::Video(_) => Ok(Msg::Save),
                            _ => Ok(Msg::InvalidInput),
                        },
                        'p' => match page {
                            Page::Video(_) => Ok(Msg::Play),
                            _ => Ok(Msg::ListPrevious),
                        },
                        'n' => Ok(Msg::ListNext),
                        'P' => Ok(Msg::PlaySave),
                        'd' => Ok(Msg::PlayDetached),
                        'm' => Ok(Msg::More),
                        'w' => match page {
                            Page::Video(video) => Ok(Msg::WatchLaterAdd(video.source.clone())),
                            _ => Ok(Msg::Page(Page::WatchLater(PageWatchLater))),
                        },
                        'i' => Ok(Msg::Information),
                        _ => Ok(Msg::InvalidInput),
                    };
                } else if let KeyCode::Esc = code {
                    return match page {
                        Page::SearchVideoTyping(_) | Page::SearchChannelTyping(_) => Ok(Msg::Back),
                        _ => Ok(Msg::FinderEscape),
                    };
                }
            }
        }
    }
}
