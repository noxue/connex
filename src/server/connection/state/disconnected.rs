use async_trait::async_trait;

use crate::server::connection::{state::ConnectionState, Connection};

pub struct DisconnectedState;

#[async_trait]
impl ConnectionState for DisconnectedState {
    async fn read(&self, _conn: &mut Connection, _buf: &mut [u8]) -> Result<usize, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Disconnected"))
    }

    async fn read_exact(&self, _conn: &mut Connection, _buf: &mut [u8]) -> Result<usize, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Disconnected"))
    }

    async fn write(&self, _conn: &mut Connection, _buf: &[u8]) -> Result<usize, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Disconnected"))
    }
}
