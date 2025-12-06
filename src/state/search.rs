use std::cmp::max;

use crate::{
    data::{channel::ChannelInfo, video::Video},
    input::TextInputType,
};

#[derive(Clone)]
pub struct SearchData<T: Clone> {
    query: String,
    results: Vec<T>,
}

#[derive(Clone)]
pub enum Search {
    Video(Option<SearchData<(Video, ChannelInfo)>>),
    Channel(Option<SearchData<ChannelInfo>>),
}

impl Search {
    pub fn query(&self) -> Option<&str> {
        match self {
            Search::Video(data) => data.as_ref().map(|data| data.query()),
            Search::Channel(data) => data.as_ref().map(|data| data.query()),
        }
    }
}

impl<T: Clone> SearchData<T> {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
        }
    }

    pub fn with_results(&self, data: Vec<T>) -> SearchData<T> {
        let mut cloned = self.clone();
        cloned.results = data;
        cloned
    }

    pub fn input(&mut self, input: TextInputType) {
        match input {
            TextInputType::Init => self.query = String::new(),
            TextInputType::Char(c) => self.query.push(c),
            TextInputType::Backspace => {
                self.query.truncate(max(self.query.len(), 1) - 1);
            }
            TextInputType::Submit => {}
        }
    }

    pub fn results(&self) -> &[T] {
        &self.results
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}
