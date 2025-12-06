#[derive(Clone, Default)]
pub struct FinderData {
    query: Option<String>,
}

impl FinderData {
    pub fn is_active(&self) -> bool {
        self.query.is_some()
    }

    pub fn add(&mut self, added: char) {
        self.query = Some(self.query.take().map_or(String::from(added), |mut query| {
            query.push(added);
            query
        }));
    }

    pub fn delete(&mut self) {
        self.query = Some(self.query.take().map_or(String::new(), |mut query| {
            query.shrink_to(query.len() - 1);
            query
        }));
    }

    pub fn matches(&self, given: &str) -> bool {
        self.query
            .as_ref()
            .map(|query| given.to_lowercase().contains(&query.to_lowercase()))
            .unwrap_or(true)
    }
}
