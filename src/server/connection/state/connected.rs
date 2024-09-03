use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::server::connection::{state::ConnectionState, Connection};

pub struct ConnectedState;

#[async_trait]
impl ConnectionState for ConnectedState {
    async fn read(&self, conn: &mut Connection, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        conn.reader.as_mut().unwrap().read(buf).await
    } 

    async fn read_exact(&self, conn: &mut Connection, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        conn.reader.as_mut().unwrap().read_exact(buf).await
    }

    async fn write(&self, conn: &mut Connection, buf: &[u8]) -> Result<usize, std::io::Error> {
        conn.writer.as_mut().unwrap().write(buf).await
    }
}
