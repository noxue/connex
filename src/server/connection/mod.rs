pub mod state;

use crate::core::{filter::Filter, key_exchange::KeyExchange, pipeline::Pipeline};
use state::ConnectionState;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};
use uuid::Uuid;

pub struct Connection {
    session_id: String,
    pipeline_in: Pipeline,
    pipeline_out: Pipeline,

    // tokio reader and writer
    reader: Option<OwnedReadHalf>,
    writer: Option<OwnedWriteHalf>,

    // exchange key
    key_exchange: Option<Arc<Mutex<dyn KeyExchange>>>,
}

impl Connection {
    pub fn new(reader: OwnedReadHalf, writer: OwnedWriteHalf) -> Connection {
        Connection {
            session_id: Uuid::new_v4().to_string(),
            pipeline_in: Pipeline::new(),
            pipeline_out: Pipeline::new(),
            reader: Some(reader),
            writer: Some(writer),
            key_exchange: None,
        }
    }

    // set key exchange
    pub fn set_key_exchange(&mut self, key_exchange: Arc<Mutex<dyn KeyExchange>>) {
        self.key_exchange = Some(key_exchange);
    }

    // add filter to pipeline_in
    pub fn add_filter_in(&mut self, filter: Box<dyn Filter>) {
        self.pipeline_in.add_filter(filter);
    }

    // add filter to pipeline_out
    pub fn add_filter_out(&mut self, filter: Box<dyn Filter>) {
        self.pipeline_out.add_filter(filter);
    }

    pub async fn start(&mut self) {
        
        
    }
}

pub struct ConnectionManager {
    connections: HashMap<String, Connection>,
}
