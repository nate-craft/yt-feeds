use enum_dispatch::enum_dispatch;

use crate::{
    data::{
        channel::ChannelInfo,
        channels::{ChannelIndex, Channels},
    },
    input::TextInputType,
    process::Process,
    state::state::{PlaySource, UIMessage},
    view::view::View,
};

pub type ActionResult = anyhow::Result<UIMessage>;

pub enum MixedResult<T> {
    Process(Process<T>),
    Action(ActionResult),
}

#[enum_dispatch(Lifecycle)]
pub trait Lifecycle {
    fn init(&mut self, channels: &mut Channels, view: &mut View) -> ActionResult {
        Ok(UIMessage::Display)
    }
}

#[enum_dispatch(More)]
pub trait More {
    fn more(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(TextInput)]
pub trait TextInput<T> {
    fn text_input_start(&mut self, _: &mut Channels, _: &mut View) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }

    fn text_input_submit(&mut self, _: &mut Channels, _: &mut View) -> MixedResult<T> {
        MixedResult::Action(Ok(UIMessage::InvalidInput))
    }

    fn text_input_exit(&mut self, _: &mut Channels, _: &mut View) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }

    fn text_input(&mut self, _: &mut Channels, _: TextInputType, _: &mut View) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }

    fn text(&self) -> Option<&str> {
        None
    }

    fn text_is_active(&self) -> bool {
        false
    }
}

#[enum_dispatch(Finder)]
pub trait Finder {
    fn finder_start(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }

    fn finder_exit(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }

    fn finder_input(&self, _: &mut Channels, _: TextInputType) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }

    fn finder_active(&self) -> bool {
        false
    }
}

#[enum_dispatch(Information)]
pub trait Information {
    fn information(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(Refresh)]
pub trait Refresh {
    fn refresh(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(Play)]
pub trait Play {
    fn play(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn play_detached(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn play_save(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn save(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn detach(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn cancel(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(Select)]
pub trait Select {
    fn select(&self, _: &mut Channels, _: usize) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn previous(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
    fn next(&self, _: &mut Channels) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(Subscribe)]
pub trait Subscribe {
    fn subscribe(&self, _: &mut Channels, _: ChannelInfo) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(Unsubscribe)]
pub trait Unsubscribe {
    fn unsubscribe(&self, _: &mut Channels, _: ChannelIndex) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}

#[enum_dispatch(WatchLater)]
pub trait WatchLater {
    fn watch_later_add(&self, _: &mut Channels, source: PlaySource) -> ActionResult {
        Ok(UIMessage::InvalidInput)
    }
}
