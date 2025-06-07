use crate::{page::Page, views::View};

pub struct Finder<T> {
    elements: Vec<T>,
    page: Page,
    query: Option<String>,
}

impl<T> Finder<T> {
    pub fn new(count: usize, count_per_line: usize) -> Self {
        Finder {
            elements: Vec::new(),
            page: Page::new(count, count_per_line),
            query: None,
        }
    }

    pub fn videos_or<'a>(&'a self, other: &'a Vec<T>) -> &'a Vec<T> {
        if self.query.is_some() {
            &self.elements
        } else {
            other
        }
    }

    pub fn page_or<'a>(&'a self, other: &'a Page) -> &'a Page {
        if self.query.is_some() {
            &self.page
        } else {
            other
        }
    }

    pub fn page_or_mut<'a>(&'a mut self, other: &'a mut Page) -> &'a mut Page {
        if self.query.is_some() {
            &mut self.page
        } else {
            other
        }
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    pub fn reset(&mut self, view: &mut View) {
        view.update_filter(None);
        self.query = None;
    }

    pub fn update(&mut self, view: &mut View, elements: Vec<T>, new_query: &str) {
        self.query = Some(new_query.to_owned());
        self.elements = elements;
        self.page = Page::new(self.elements.len(), self.page.lines_per_element);
        view.update_filter(Some(new_query.to_owned()));
    }
}
