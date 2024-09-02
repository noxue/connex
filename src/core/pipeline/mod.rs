use crate::core::filter::{Filter, error::FilterResult};

pub struct Pipeline {
    filters: Vec<Box<dyn Filter>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }

    pub fn process(&self, data: Vec<u8>) -> FilterResult<Vec<u8>> {
        self.filters
            .iter()
            .try_fold(data, |acc, filter| filter.process(acc))
    }
}
