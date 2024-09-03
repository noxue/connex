mod error;

use error::KeyExchangeResult;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error>;
}

pub trait Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>;
    fn flush(&mut self) -> Result<(), std::io::Error>;
}

pub trait KeyExchange: Send + Sync {
    fn perform_key_exchange(
        &self,
        reaer: &mut dyn Reader,
        writer: &mut dyn Writer,
    ) -> KeyExchangeResult<Vec<u8>>;
}

// impl Reader for OwnedReadHalf {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
//         OwnedReadHalf::read(self, buf)
//     }

//     fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
//         OwnedReadHalf::read_exact(self, buf)
//     }
// }

// impl Writer for OwnedWriteHalf {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
//         OwnedWriteHalf::write(self, buf)
//     }

//     fn flush(&mut self) -> Result<(), std::io::Error> {
//         OwnedWriteHalf::flush(self)
//     }
// }
