pub mod error;

use crate::core::filter::error::FilterResult;

pub trait Filter : Send + Sync{
    fn process(&self, data: Vec<u8>) -> FilterResult<Vec<u8>>;
}
