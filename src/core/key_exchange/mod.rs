mod error;

use error::KeyExchangeResult;

pub trait Stream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>;
}

pub trait KeyExchange {
    fn perform_key_exchange(&self, stream: &mut dyn Stream) -> KeyExchangeResult<Vec<u8>>;
}

