
// 协商session id
// negotiate session id
pub mod negotiate;

// 连接状态
// connected state
pub mod connected;

// 断开连接
// disconnect
pub mod disconnected;

use super::Connection;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait ConnectionState: Send + Sync {
    async fn read(&self, conn: &mut Connection, buf: &mut [u8]) -> Result<usize, std::io::Error>;
    async fn read_exact(
        &self,
        conn: &mut Connection,
        buf: &mut [u8],
    ) -> Result<usize, std::io::Error>;
    async fn write(&self, conn: &mut Connection, buf: &[u8]) -> Result<usize, std::io::Error>;
    async fn flush(&self, conn: &mut Connection) -> Result<(), std::io::Error> {
        conn.writer.as_mut().unwrap().flush().await
    }
}
