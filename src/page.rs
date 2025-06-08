use std::cmp::{max, min};

use crossterm::terminal;

pub struct Page {
    pub current_index: usize,
    pub count_per_page: usize,
    pub count_total: usize,
    pub lines_per_element: usize,
}

impl Page {
    pub fn new(count_total: usize, lines_per_element: usize) -> Page {
        Page {
            current_index: 0,
            // Avoids terminal scrolling
            // Take smallest between count and the terminal divided up by each element's space
            count_per_page: min(
                min(count_total, 10),
                terminal::size().unwrap().1 as usize / lines_per_element - 4,
            ),
            count_total,
            lines_per_element,
        }
    }

    pub fn current_page<'a, T>(&self, videos: &'a [T]) -> &'a [T] {
        &videos[self.current_index..(self.current_index + self.count_per_page)]
    }

    pub fn last_index(&self) -> usize {
        self.count_total
    }

    pub fn pages_count(&self) -> usize {
        (self.count_total as f32 / max(self.count_per_page, 1) as f32).ceil() as usize
    }

    pub fn page_current(&self) -> usize {
        (self.current_index as f32 / max(self.count_per_page, 1) as f32).ceil() as usize + 1
    }

    pub fn next_page(&mut self) {
        self.current_index = min(
            self.current_index + self.count_per_page,
            self.count_total - self.count_per_page,
        )
    }

    pub fn prev_page(&mut self) {
        self.current_index = max(
            self.current_index as i32 - self.count_per_page as i32,
            0 as i32,
        ) as usize;
    }

    pub fn item_at_index<'a, T>(&self, elements: &'a [T], index: usize) -> Option<&'a T> {
        if self.item_is_at_index(index) {
            elements.get(index + self.current_index)
        } else {
            None
        }
    }

    pub fn take_item_at_index<T>(&self, elements: &mut Vec<T>, index: usize) -> Option<T> {
        if self.item_is_at_index(index) {
            Some(elements.remove(index + self.current_index))
        } else {
            None
        }
    }

    pub fn item_is_at_index(&self, index: usize) -> bool {
        self.count_per_page != 0
            && index <= self.count_per_page - 1
            && index + self.current_index <= self.count_total
    }
}
