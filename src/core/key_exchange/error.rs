use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum KeyExchangeError {
    #[snafu(display("I/O error during key exchange: {}", source))]
    IoError { source: std::io::Error },

    #[snafu(display("Encryption error during key exchange: {}", source))]
    EncryptionError { source: Box<dyn std::error::Error> },

    #[snafu(display("Decryption error during key exchange: {}", source))]
    DecryptionError { source: Box<dyn std::error::Error> },

    #[snafu(display("Protocol error during key exchange: {}", message))]
    ProtocolError { message: String },

    #[snafu(display("Data format error during key exchange: {}", message))]
    DataFormatError { message: String },

    #[snafu(display("Timeout error during key exchange: {}", message))]
    TimeoutError { message: String },

    #[snafu(display("{}", message))]
    UserDefinedError { message: String },
}

pub type KeyExchangeResult<T> = std::result::Result<T, KeyExchangeError>;
